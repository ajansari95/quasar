package e2e

import (
	"context"
	"encoding/json"
	sdk "github.com/cosmos/cosmos-sdk/types"
	connectiontypes "github.com/cosmos/ibc-go/v4/modules/core/03-connection/types"
	channeltypes "github.com/cosmos/ibc-go/v4/modules/core/04-channel/types"
	testsuite "github.com/quasarlabs/quasarnode/tests/e2e/suite"
	"github.com/strangelove-ventures/interchaintest/v4/ibc"
	"github.com/strangelove-ventures/interchaintest/v4/testutil"
	"github.com/stretchr/testify/suite"
	"math/rand"
	"strconv"
	"testing"
	"time"
)

const (
	StartingTokenAmount            int64 = 100_000_000_000
	lpStrategyContractPath               = "../../smart-contracts/artifacts/lp_strategy-aarch64.wasm"
	basicVaultStrategyContractPath       = "../../smart-contracts/artifacts/basic_vault-aarch64.wasm"
	vaultRewardsContractPath             = "../../smart-contracts/artifacts/vault_rewards-aarch64.wasm"
	osmosisPool1Path                     = "scripts/sample_pool1.json"
	osmosisPool2Path                     = "scripts/sample_pool2.json"
	osmosisPool3Path                     = "scripts/sample_pool3.json"
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
	s.OsmosisDenomInQuasar = ibcDenomFromChannel(s.Quasar2OsmosisTransferChan, s.Osmosis().Config().Denom)
	s.QuasarDenomInOsmosis = ibcDenomFromChannelCounterparty(s.Quasar2OsmosisTransferChan, s.Quasar().Config().Denom)

	// Setup an account in quasar chain for contract deployment
	s.ContractsDeploymentWallet = s.CreateUserAndFund(ctx, s.Quasar(), StartingTokenAmount)

	// Send tokens "uayy" and "uqsr" from Quasar to Osmosis account
	s.SendTokensToRespectiveAccounts(ctx)

	// Send tokens to the respective account and create the required pools
	s.CreatePools(ctx)

	// Deploy the lp strategy contract
	s.deployPrimitives(ctx, s.ContractsDeploymentWallet, lpStrategyContractPath, "lp_strategy_test", init1, init2, init3)

	// Deploy reward contract
	s.deployRewardsContract(ctx, s.ContractsDeploymentWallet, vaultRewardsContractPath)

	// deploy basic_vault contract
	s.BasicVaultContractAddress = s.deployVault(ctx, s.ContractsDeploymentWallet, basicVaultStrategyContractPath, "basic_vault",
		map[string]any{
			"total_cap":                     "100000000000000000",
			"thesis":                        "e2e",
			"vault_rewards_code_id":         s.RewardsStoreID,
			"reward_token":                  map[string]any{"native": "uqsr"},
			"reward_distribution_schedules": []string{},
			"decimals":                      6,
			"symbol":                        "ORN",
			"min_withdrawal":                "1",
			"name":                          "ORION",
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

func (s *WasmdTestSuite) TestLockBond() {
	t := s.T()
	ctx := context.Background()

	type tc struct {
		Account                  ibc.Wallet // necessary field
		BondAmount               sdk.Coins  // necessary in case of bonds
		UnbondAmount             string     // only in case of Action is "unbond"
		Action                   string     // necessary to provide action, 3 possibilities "bond", "unbond" or "claim"
		expectedShares           int64      // only needed in case of "bond"
		expectedDeviation        float64    // only needed in case of "bond"
		expectedNumberOfUnbonds  int64      // only needed in case of "unbond"
		expectedBalanceChange    uint64     // only needed in case of "claim"
		expectedBalanceDeviation float64    // only needed in case of "claim"
	}
	var testCases []tc

	rand.Seed(time.Now().UnixNano())
	loweLimit := 99_999
	upperLimit := 100_000_000_000
	max := 100_000_000_000_000
	sum := 0
	for i := 1; i <= 10000; i++ {
		j := rand.Intn(upperLimit-loweLimit) + loweLimit
		sum += j
		if sum >= max {
			break
		}
		testCases = append(testCases, tc{
			Account:    s.E2EBuilder.QuasarAccounts.BondTest,
			Action:     "bond",
			BondAmount: sdk.NewCoins(sdk.NewInt64Coin("ibc/ED07A3391A112B175915CD8FAF43A2DA8E4790EDE12566649D0C2F97716B8518", int64(j))),
		})
		testCases = append(testCases, tc{
			Account:    s.E2EBuilder.QuasarAccounts.BondTest1,
			Action:     "bond",
			BondAmount: sdk.NewCoins(sdk.NewInt64Coin("ibc/ED07A3391A112B175915CD8FAF43A2DA8E4790EDE12566649D0C2F97716B8518", int64(j))),
		})
		testCases = append(testCases, tc{
			Account:    s.E2EBuilder.QuasarAccounts.BondTest2,
			Action:     "bond",
			BondAmount: sdk.NewCoins(sdk.NewInt64Coin("ibc/ED07A3391A112B175915CD8FAF43A2DA8E4790EDE12566649D0C2F97716B8518", int64(j))),
		})
		testCases = append(testCases, tc{
			Account:    s.E2EBuilder.QuasarAccounts.BondTest3,
			Action:     "bond",
			BondAmount: sdk.NewCoins(sdk.NewInt64Coin("ibc/ED07A3391A112B175915CD8FAF43A2DA8E4790EDE12566649D0C2F97716B8518", int64(j))),
		})
	}

	for i, tc := range testCases {
		switch tc.Action {
		case "bond":
			// execute bond transaction
			t.Logf("doing a bond for %s with amount %s", s.E2EBuilder.QuasarAccounts.BondTest.KeyName, tc.BondAmount.String())
			s.ExecuteContract(
				ctx,
				s.Quasar(),
				tc.Account.KeyName,
				s.BasicVaultContractAddress,
				tc.BondAmount,
				map[string]any{"bond": map[string]any{}},
				nil,
			)

			// print the bond state of all the contracts
			s.GetStates(ctx)
		default:
			t.Log("yay")
		}

		// unbond half of the balance from the last 4 test cases
		if i%20 == 0 && i != 0 {
			balance3 := s.GetAccountBalanceInVault(ctx, testCases[i-3].Account.Address)
			t.Logf("doing a unbond for %s with amount %s", testCases[i-3].Account.KeyName, strconv.FormatInt(balance3, 10))
			s.ExecuteContract(
				ctx,
				s.Quasar(),
				testCases[i-3].Account.KeyName,
				s.BasicVaultContractAddress,
				sdk.NewCoins(),
				map[string]any{"unbond": map[string]any{"amount": strconv.FormatInt(balance3/2, 10)}},
				nil,
			)

			balance2 := s.GetAccountBalanceInVault(ctx, testCases[i-2].Account.Address)
			t.Logf("doing a unbond for %s with amount %s", testCases[i-2].Account.KeyName, strconv.FormatInt(balance2, 10))
			s.ExecuteContract(
				ctx,
				s.Quasar(),
				testCases[i-2].Account.KeyName,
				s.BasicVaultContractAddress,
				sdk.NewCoins(),
				map[string]any{"unbond": map[string]any{"amount": strconv.FormatInt(balance2/2, 10)}},
				nil,
			)

			balance1 := s.GetAccountBalanceInVault(ctx, testCases[i-1].Account.Address)
			t.Logf("doing a unbond for %s with amount %s", testCases[i-1].Account.KeyName, strconv.FormatInt(balance1, 10))
			s.ExecuteContract(
				ctx,
				s.Quasar(),
				testCases[i-1].Account.KeyName,
				s.BasicVaultContractAddress,
				sdk.NewCoins(),
				map[string]any{"unbond": map[string]any{"amount": strconv.FormatInt(balance1/2, 10)}},
				nil,
			)

			balance := s.GetAccountBalanceInVault(ctx, testCases[i].Account.Address)
			t.Logf("doing a unbond for %s with amount %s", testCases[i].Account.KeyName, strconv.FormatInt(balance, 10))
			s.ExecuteContract(
				ctx,
				s.Quasar(),
				testCases[i].Account.KeyName,
				s.BasicVaultContractAddress,
				sdk.NewCoins(),
				map[string]any{"unbond": map[string]any{"amount": strconv.FormatInt(balance/2, 10)}},
				nil,
			)

			// print the bond state of all the contracts
			s.GetStates(ctx)
		}

		//claim from all the last 4 test cases
		if i%30 == 0 && i != 0 {
			t.Logf("doing a claim for %s", testCases[i-3].Account.KeyName)
			s.ExecuteContract(
				ctx,
				s.Quasar(),
				testCases[i-3].Account.KeyName,
				s.BasicVaultContractAddress,
				sdk.NewCoins(),
				map[string]any{"claim": map[string]any{}},
				nil,
			)

			t.Logf("doing a claim for %s", testCases[i-2].Account.KeyName)
			s.ExecuteContract(
				ctx,
				s.Quasar(),
				testCases[i-2].Account.KeyName,
				s.BasicVaultContractAddress,
				sdk.NewCoins(),
				map[string]any{"claim": map[string]any{}},
				nil,
			)

			t.Logf("doing a claim for %s", testCases[i-1].Account.KeyName)
			s.ExecuteContract(
				ctx,
				s.Quasar(),
				testCases[i-1].Account.KeyName,
				s.BasicVaultContractAddress,
				sdk.NewCoins(),
				map[string]any{"claim": map[string]any{}},
				nil,
			)

			t.Logf("doing a claim for %s", testCases[i].Account.KeyName)
			s.ExecuteContract(
				ctx,
				s.Quasar(),
				testCases[i].Account.KeyName,
				s.BasicVaultContractAddress,
				sdk.NewCoins(),
				map[string]any{"claim": map[string]any{}},
				nil,
			)
			// print the bond state of all the contracts
			s.GetStates(ctx)
		}

		// clear cache on contracts after every 20 test cases
		if i%20 == 0 {
			t.Log("Wait for quasar and osmosis to settle up ICA packet transfer and the ibc transfer")
			err := testutil.WaitForBlocks(ctx, 5, s.Quasar(), s.Osmosis())
			s.Require().NoError(err)

			// print the bond state of all the contracts
			s.GetStates(ctx)

			s.ExecuteContract(
				ctx,
				s.Quasar(),
				s.ContractsDeploymentWallet.KeyName,
				s.BasicVaultContractAddress,
				sdk.Coins{},
				map[string]any{"clear_cache": map[string]any{}},
				nil,
			)

			t.Log("Wait for quasar to clear cache and settle up ICA packet transfer and the ibc transfer")
			err = testutil.WaitForBlocks(ctx, 10, s.Quasar(), s.Osmosis())
			s.Require().NoError(err)

			// print the bond state of all the contracts
			s.GetStates(ctx)
		}
	}
}

// TestLpStrategyContract_SuccessfulDeposit tests the lp strategy contract creating an ICA channel between the contract and osmosis
// and depositing 1000uqsr tokens to the contract which it must ibc transfer to its ICA account at osmosis.
func (s *WasmdTestSuite) TestLpStrategyContract_SuccessfulDeposit() {
	t := s.T()
	ctx := context.Background()

	testCases := []struct {
		Account                  ibc.Wallet // necessary field
		BondAmount               sdk.Coins  // necessary in case of bonds
		UnbondAmount             string     // only in case of Action is "unbond"
		Action                   string     // necessary to provide action, 3 possibilities "bond", "unbond" or "claim"
		expectedShares           int64      // only needed in case of "bond"
		expectedDeviation        float64    // only needed in case of "bond"
		expectedNumberOfUnbonds  int64      // only needed in case of "unbond"
		expectedBalanceChange    uint64     // only needed in case of "claim"
		expectedBalanceDeviation float64    // only needed in case of "claim"
	}{
		{
			Account:           s.E2EBuilder.QuasarAccounts.BondTest,
			Action:            "bond",
			BondAmount:        sdk.NewCoins(sdk.NewInt64Coin("ibc/ED07A3391A112B175915CD8FAF43A2DA8E4790EDE12566649D0C2F97716B8518", 10000000)),
			expectedShares:    9999999,
			expectedDeviation: 0.01,
		},
		{
			Account:           s.E2EBuilder.QuasarAccounts.BondTest1,
			Action:            "bond",
			BondAmount:        sdk.NewCoins(sdk.NewInt64Coin("ibc/ED07A3391A112B175915CD8FAF43A2DA8E4790EDE12566649D0C2F97716B8518", 1000000)),
			expectedShares:    1015176,
			expectedDeviation: 0.01,
		},
		{
			Account:                 s.E2EBuilder.QuasarAccounts.BondTest,
			Action:                  "unbond",
			BondAmount:              sdk.NewCoins(),
			UnbondAmount:            "1000",
			expectedNumberOfUnbonds: 1,
		},
		{
			Account:                 s.E2EBuilder.QuasarAccounts.BondTest,
			Action:                  "unbond",
			BondAmount:              sdk.NewCoins(),
			UnbondAmount:            "2000",
			expectedNumberOfUnbonds: 2,
		},
		{
			Account:                  s.E2EBuilder.QuasarAccounts.BondTest,
			Action:                   "claim",
			expectedBalanceChange:    1000,
			expectedBalanceDeviation: 0.1,
		},
		{
			Account:           s.E2EBuilder.QuasarAccounts.BondTest7,
			Action:            "bond",
			BondAmount:        sdk.NewCoins(sdk.NewInt64Coin("ibc/ED07A3391A112B175915CD8FAF43A2DA8E4790EDE12566649D0C2F97716B8518", 1000000)),
			expectedShares:    1015176,
			expectedDeviation: 0.01,
		},
	}

	for _, tc := range testCases {
		switch tc.Action {
		case "bond":
			// execute bond transaction
			s.ExecuteContract(
				ctx,
				s.Quasar(),
				tc.Account.KeyName,
				s.BasicVaultContractAddress,
				tc.BondAmount,
				map[string]any{"bond": map[string]any{}},
				nil,
			)

			t.Log("Wait for quasar and osmosis to settle up ICA packet transfer and the ibc transfer")
			err := testutil.WaitForBlocks(ctx, 5, s.Quasar(), s.Osmosis())
			s.Require().NoError(err)

			s.ExecuteContract(
				ctx,
				s.Quasar(),
				s.ContractsDeploymentWallet.KeyName,
				s.BasicVaultContractAddress,
				sdk.Coins{},
				map[string]any{"clear_cache": map[string]any{}},
				nil,
			)

			t.Log("Wait for quasar to clear cache and settle up ICA packet transfer and the ibc transfer")
			err = testutil.WaitForBlocks(ctx, 15, s.Quasar(), s.Osmosis())
			s.Require().NoError(err)

			var data testsuite.ContractBalanceData
			balanceBytes := s.ExecuteContractQuery(
				ctx,
				s.Quasar(),
				s.BasicVaultContractAddress,
				map[string]any{
					"balance": map[string]any{
						"address": tc.Account.Address,
					},
				},
			)

			err = json.Unmarshal(balanceBytes, &data)
			s.Require().NoError(err)

			balance, err := strconv.ParseInt(data.Data.Balance, 10, 64)
			s.Require().NoError(err)

			s.Require().True(int64(float64(tc.expectedShares)*(1-tc.expectedDeviation)) <= balance)
			s.Require().True(balance <= int64(float64(tc.expectedShares)*(1+tc.expectedDeviation)))

		case "unbond":
			s.ExecuteContract(
				ctx,
				s.Quasar(),
				tc.Account.KeyName,
				s.BasicVaultContractAddress,
				sdk.NewCoins(),
				map[string]any{"unbond": map[string]any{"amount": tc.UnbondAmount}},
				nil,
			)

			t.Log("Wait for quasar and osmosis to settle up ICA packet transfer and the ibc transfer")
			err := testutil.WaitForBlocks(ctx, 5, s.Quasar(), s.Osmosis())
			s.Require().NoError(err)

			s.ExecuteContract(
				ctx,
				s.Quasar(),
				s.ContractsDeploymentWallet.KeyName,
				s.BasicVaultContractAddress,
				sdk.Coins{},
				map[string]any{"clear_cache": map[string]any{}},
				nil,
			)

			t.Log("Wait for quasar to clear cache and settle up ICA packet transfer and the ibc transfer")
			err = testutil.WaitForBlocks(ctx, 15, s.Quasar(), s.Osmosis())
			s.Require().NoError(err)

			var pendingUnbondsData testsuite.PendingUnbondsData
			pendingUnbondsBytes := s.ExecuteContractQuery(
				ctx,
				s.Quasar(),
				s.BasicVaultContractAddress,
				map[string]any{
					"pending_unbonds": map[string]any{
						"address": tc.Account.Address,
					},
				},
			)

			err = json.Unmarshal(pendingUnbondsBytes, &pendingUnbondsData)
			s.Require().NoError(err)

			// verify if the unbonded amount and expected number of unbonds matches their respective conditions or not
			s.Require().Equal(tc.expectedNumberOfUnbonds, int64(len(pendingUnbondsData.Data.PendingUnbonds)))
			s.Require().Equal(tc.UnbondAmount, pendingUnbondsData.Data.PendingUnbonds[tc.expectedNumberOfUnbonds-1].Shares)
		case "claim":
			tn := testsuite.GetFullNode(s.Quasar())
			cmds := []string{"bank", "balances", s.E2EBuilder.QuasarAccounts.BondTest.Address,
				"--output", "json",
			}

			res, _, err := tn.ExecQuery(ctx, cmds...)
			s.Require().NoError(err)

			var balanceBefore testsuite.QueryAllBalancesResponse
			err = json.Unmarshal(res, &balanceBefore)
			s.Require().NoError(err)

			s.ExecuteContract(
				ctx,
				s.Quasar(),
				tc.Account.KeyName,
				s.BasicVaultContractAddress,
				sdk.NewCoins(),
				map[string]any{"claim": map[string]any{}},
				nil,
			)

			t.Log("Wait for quasar to clear cache and settle up ICA packet transfer and the ibc transfer")
			err = testutil.WaitForBlocks(ctx, 5, s.Quasar(), s.Osmosis())
			s.Require().NoError(err)

			s.ExecuteContract(
				ctx,
				s.Quasar(),
				s.ContractsDeploymentWallet.KeyName,
				s.BasicVaultContractAddress,
				sdk.Coins{},
				map[string]any{"clear_cache": map[string]any{}},
				nil,
			)

			t.Log("Wait for quasar to clear cache and settle up ICA packet transfer and the ibc transfer")
			err = testutil.WaitForBlocks(ctx, 15, s.Quasar(), s.Osmosis())
			s.Require().NoError(err)

			tn = testsuite.GetFullNode(s.Quasar())
			res, _, err = tn.ExecQuery(ctx, cmds...)
			s.Require().NoError(err)

			var balanceAfter testsuite.QueryAllBalancesResponse
			err = json.Unmarshal(res, &balanceAfter)
			s.Require().NoError(err)

			balanceChange := balanceAfter.Balances.AmountOf(s.OsmosisDenomInQuasar).Sub(balanceBefore.Balances.AmountOf(s.OsmosisDenomInQuasar)).Int64()
			s.Require().True(int64(float64(tc.expectedBalanceChange)*(1-tc.expectedBalanceDeviation)) <= balanceChange)
			s.Require().True(balanceChange <= int64(float64(tc.expectedBalanceChange)*(1+tc.expectedBalanceDeviation)))

		default:
			t.Log("This testCase does not contain any transaction type")
		}
	}
}
