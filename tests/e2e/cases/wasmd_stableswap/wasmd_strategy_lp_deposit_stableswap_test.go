package wasmd_deposit

import (
	"context"
	"encoding/json"
	"fmt"
	"github.com/quasarlabs/quasarnode/tests/e2e/cases/_helpers"
	"strconv"
	"testing"

	sdk "github.com/cosmos/cosmos-sdk/types"
	connectiontypes "github.com/cosmos/ibc-go/v4/modules/core/03-connection/types"
	channeltypes "github.com/cosmos/ibc-go/v4/modules/core/04-channel/types"
	testsuite "github.com/quasarlabs/quasarnode/tests/e2e/suite"
	"github.com/strangelove-ventures/interchaintest/v4/ibc"
	"github.com/strangelove-ventures/interchaintest/v4/testutil"
	"github.com/stretchr/testify/suite"
)

const (
	StartingTokenAmount            int64 = 100_000_000_000
	BondAmount                     int64 = 10_000_000
	lpStrategyContractPath               = "../../../../smart-contracts/artifacts/lp_strategy-aarch64.wasm"
	basicVaultStrategyContractPath       = "../../../../smart-contracts/artifacts/basic_vault-aarch64.wasm"
	vaultRewardsContractPath             = "../../../../smart-contracts/artifacts/vault_rewards-aarch64.wasm"
	osmosisPool1Path                     = "../_utils/pools/high_liquidity_stable/sample_pool1.json"
	osmosisPool2Path                     = "../_utils/pools/high_liquidity_stable/sample_pool2.json"
	osmosisPool3Path                     = "../_utils/pools/high_liquidity_stable/sample_pool3.json"
)

var (
	init1 = map[string]any{
		"lock_period": 6, "pool_id": 1, "pool_denom": "gamm/pool/1", "base_denom": "uosmo",
		"local_denom": "ibc/ED07A3391A112B175915CD8FAF43A2DA8E4790EDE12566649D0C2F97716B8518", "quote_denom": "stake1",
		"return_source_channel": "channel-0", "transfer_channel": "channel-0", "expected_connection": "connection-0",
	}
	init2 = map[string]any{
		"lock_period": 6, "pool_id": 2, "pool_denom": "gamm/pool/2", "base_denom": "uosmo",
		"local_denom": "ibc/ED07A3391A112B175915CD8FAF43A2DA8E4790EDE12566649D0C2F97716B8518", "quote_denom": "usdc",
		"return_source_channel": "channel-0", "transfer_channel": "channel-0", "expected_connection": "connection-0",
	}
	init3 = map[string]any{
		"lock_period": 6, "pool_id": 3, "pool_denom": "gamm/pool/3", "base_denom": "uosmo",
		"local_denom": "ibc/ED07A3391A112B175915CD8FAF43A2DA8E4790EDE12566649D0C2F97716B8518", "quote_denom": "fakestake",
		"return_source_channel": "channel-0", "transfer_channel": "channel-0", "expected_connection": "connection-0",
	}
)

func TestWasmdTestSuite(t *testing.T) {
	if testing.Short() {
		t.Skip()
	}

	b := testsuite.NewE2ETestSuiteBuilder(t)
	b.UseOsmosis()
	b.Link(testsuite.Quasar2OsmosisPath)
	b.AutomatedRelay()

	s := &WasmdTestSuite{
		E2EBuilder:   b,
		E2ETestSuite: b.Build(),
	}
	suite.Run(t, s)
}

type WasmdTestSuite struct {
	E2EBuilder *testsuite.E2ETestSuiteBuilder

	*testsuite.E2ETestSuite

	Quasar2OsmosisConn *connectiontypes.IdentifiedConnection
	Osmosis2QuasarConn *connectiontypes.IdentifiedConnection

	Quasar2OsmosisTransferChan *channeltypes.IdentifiedChannel
	Osmosis2QuasarTransferChan *channeltypes.IdentifiedChannel

	OsmosisDenomInQuasar string
	QuasarDenomInOsmosis string

	LpStrategyContractAddress1 string
	LpStrategyContractAddress2 string
	LpStrategyContractAddress3 string

	ContractsDeploymentWallet *ibc.Wallet

	RewardsStoreID            uint64
	PrimitiveStoreID          uint64
	VaultStoreID              uint64
	BasicVaultContractAddress string
}

func (s *WasmdTestSuite) SetupSuite() {
	t := s.T()
	ctx := context.Background()

	// Wait for IBC connections to be established
	t.Log("Wait for chains to settle up the ibc connection states")
	err := testutil.WaitForBlocks(ctx, 5, s.Quasar(), s.Osmosis())
	s.Require().NoError(err)

	// Find out connections between each pair of chains
	s.Quasar2OsmosisConn = s.GetConnectionsByPath(ctx, testsuite.Quasar2OsmosisPath)[0]
	s.Osmosis2QuasarConn = s.GetConnectionsByPath(ctx, testsuite.Quasar2OsmosisPath)[0]

	// Find out transfer channel between each pair of chains
	s.Quasar2OsmosisTransferChan = s.QueryConnectionChannels(ctx, s.Quasar(), s.Quasar2OsmosisConn.Id)[0]
	s.Osmosis2QuasarTransferChan = s.QueryConnectionChannels(ctx, s.Osmosis(), s.Osmosis2QuasarConn.Id)[0]

	// Generate the ibc denom of native tokens in other chains
	s.OsmosisDenomInQuasar = helpers.IbcDenomFromChannel(s.Quasar2OsmosisTransferChan, s.Osmosis().Config().Denom)
	s.QuasarDenomInOsmosis = helpers.IbcDenomFromChannelCounterparty(s.Quasar2OsmosisTransferChan, s.Quasar().Config().Denom)

	// Setup an account in quasar chain for contract deployment
	s.ContractsDeploymentWallet = s.CreateUserAndFund(ctx, s.Quasar(), StartingTokenAmount)

	// Send tokens to the respective account and create the required pools
	s.CreatePools(ctx)

	// Deploy the lp strategy contract
	s.deployPrimitives(ctx, s.ContractsDeploymentWallet, lpStrategyContractPath, "lp_strategy_test", init1, init2, init3)

	// Deploy reward contract
	s.deployRewardsContract(ctx, s.ContractsDeploymentWallet, vaultRewardsContractPath)

	// deploy basic_vault contract
	s.BasicVaultContractAddress = s.deployVault(ctx, s.ContractsDeploymentWallet, basicVaultStrategyContractPath, "basic_vault",
		map[string]any{
			"total_cap":                     "200000000000",
			"thesis":                        "e2e",
			"vault_rewards_code_id":         s.RewardsStoreID,
			"reward_token":                  map[string]any{"native": "uqsr"},
			"reward_distribution_schedules": []string{},
			"decimals":                      6,
			"symbol":                        "ORN",
			"min_withdrawal":                "1",
			"name":                          "ORION",
			"deposit_denom":                 "ibc/ED07A3391A112B175915CD8FAF43A2DA8E4790EDE12566649D0C2F97716B8518",
			"primitives": []map[string]any{
				{
					"address": s.LpStrategyContractAddress1,
					"weight":  "0.333333333333",
					"init": map[string]any{
						"l_p": init1,
					},
				},
				{
					"address": s.LpStrategyContractAddress2,
					"weight":  "0.333333333333",
					"init": map[string]any{
						"l_p": init2,
					},
				},
				{
					"address": s.LpStrategyContractAddress3,
					"weight":  "0.333333333333",
					"init": map[string]any{
						"l_p": init3,
					},
				},
			},
		})

	// set depositors for all the primitives
	s.setDepositorForContracts(ctx, s.ContractsDeploymentWallet,
		map[string]any{
			"set_depositor": map[string]any{
				"depositor": s.BasicVaultContractAddress,
			},
		},
	)
}

func (s *WasmdTestSuite) TestLpStrategyContract_SuccessfulDeposit() {
	t := s.T()
	ctx := context.Background()

	// create user and check his balance
	acc := s.createUserAndCheckBalances(ctx)

	t.Log("Execute first bond transaction and clearing cache")
	s.executeBondAndClearCache(ctx, acc)

	t.Log("Check that the user shares balance is higher than 0")
	balance := s.getUserSharesBalance(ctx, acc)
	s.Require().True(int64(0) < balance)

	// TODO: remove this later if trapped errors is not needed
	t.Log("Query trapped errors for each one of the primitives")
	trappedErrors := s.getTrappedErrors(ctx, []string{s.LpStrategyContractAddress1, s.LpStrategyContractAddress2, s.LpStrategyContractAddress3})

	t.Log("Parsing trapped errors to obtain seq number and channel id")
	seqError1, channelIdError1 := helpers.ParseTrappedError(trappedErrors[0])
	seqError2, channelIdError2 := helpers.ParseTrappedError(trappedErrors[1])
	seqError3, channelIdError3 := helpers.ParseTrappedError(trappedErrors[2])

	fmt.Println(seqError1, channelIdError1)
	fmt.Println(seqError2, channelIdError2)
	fmt.Println(seqError3, channelIdError3)
}

func (s *WasmdTestSuite) createUserAndCheckBalances(ctx context.Context) *ibc.Wallet {
	t := s.T()

	t.Log("Create testing account on Quasar chain with some QSR tokens for fees")
	acc := s.CreateUserAndFund(ctx, s.Quasar(), 1_000_000) // unused qsr, just for tx fees

	t.Log("Fund testing account with uosmo via IBC transfer from Osmosis chain Treasury account")
	walletAmount0 := ibc.WalletAmount{Address: acc.Bech32Address(s.Quasar().Config().Bech32Prefix), Denom: s.Osmosis().Config().Denom, Amount: BondAmount * 2}
	transfer, err := s.Osmosis().SendIBCTransfer(ctx, s.Osmosis2QuasarTransferChan.ChannelId, s.E2EBuilder.OsmosisAccounts.Treasury.KeyName, walletAmount0, ibc.TransferOptions{})
	s.Require().NoError(err)
	s.Require().NoError(transfer.Validate())

	t.Log("Wait for packet transfer and the ibc transfer to occur")
	err = testutil.WaitForBlocks(ctx, 5, s.Quasar(), s.Osmosis())
	s.Require().NoError(err)

	t.Log("Check tester accounts uosmo balance after executing IBC transfer")
	balanceTester0, err := s.Quasar().GetBalance(ctx, acc.Bech32Address(s.Quasar().Config().Bech32Prefix), s.OsmosisDenomInQuasar)
	s.Require().NoError(err)
	s.Require().Equal(BondAmount*2, balanceTester0)

	return acc
}

func (s *WasmdTestSuite) executeBondAndClearCache(ctx context.Context, acc *ibc.Wallet) {
	t := s.T()

	s.ExecuteContract(
		ctx,
		s.Quasar(),
		acc.KeyName,
		s.BasicVaultContractAddress,
		sdk.NewCoins(sdk.NewInt64Coin(s.OsmosisDenomInQuasar, BondAmount)),
		map[string]any{"bond": map[string]any{}},
		nil,
	)

	t.Log("Wait for quasar and osmosis to settle up ICA packet transfer and the IBC transfer (bond)")
	err := testutil.WaitForBlocks(ctx, 5, s.Quasar(), s.Osmosis())
	// still not error, as on quasar the tx has gone through
	s.Require().NoError(err)

	t.Log("Execute first clear cache on the contracts to perform the joinPool on the osmosis side")
	s.ExecuteContract(
		ctx,
		s.Quasar(),
		s.ContractsDeploymentWallet.KeyName,
		s.BasicVaultContractAddress,
		sdk.Coins{},
		map[string]any{"clear_cache": map[string]any{}},
		nil,
	)

	t.Log("Wait for quasar and osmosis to settle up ICA packet transfer and the IBC transfer (clear_cache)")
	err = testutil.WaitForBlocks(ctx, 15, s.Quasar(), s.Osmosis())
	s.Require().NoError(err)
}

func (s *WasmdTestSuite) getUserSharesBalance(ctx context.Context, acc *ibc.Wallet) int64 {
	var data testsuite.ContractBalanceData
	balanceBytes := s.ExecuteContractQuery(
		ctx,
		s.Quasar(),
		s.BasicVaultContractAddress,
		map[string]any{
			"balance": map[string]any{
				"address": acc.Bech32Address(s.Quasar().Config().Bech32Prefix),
			},
		},
	)
	err := json.Unmarshal(balanceBytes, &data)
	s.Require().NoError(err)
	balance, err := strconv.ParseInt(data.Data.Balance, 10, 64)
	s.Require().NoError(err)

	return balance
}

func (s *WasmdTestSuite) getTrappedErrors(ctx context.Context, lpAddresses []string) []map[string]interface{} {
	var trappedErrors []map[string]interface{}
	for _, lpAddress := range lpAddresses {
		var trappedErrors1 testsuite.ContractTrappedErrorsData
		trappedErrors1Bytes := s.ExecuteContractQuery(
			ctx,
			s.Quasar(),
			lpAddress,
			map[string]any{
				"trapped_errors": map[string]any{},
			},
		)
		err := json.Unmarshal(trappedErrors1Bytes, &trappedErrors1)
		s.Require().NoError(err)
		trappedErrors = append(trappedErrors, trappedErrors1.Data.TrappedErrors)
	}

	return trappedErrors
}
