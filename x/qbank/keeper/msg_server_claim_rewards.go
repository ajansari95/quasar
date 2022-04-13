package keeper

import (
	"context"

	oriontypes "github.com/abag/quasarnode/x/orion/types"
	"github.com/abag/quasarnode/x/qbank/types"
	sdk "github.com/cosmos/cosmos-sdk/types"
)

// ClaimRewards Transfer the accumulated rewards to the depositors account.
// If types.MsgClaimRewards.VaultID is orion vault then claim will
// be processed from the orion vault global reward account.
func (k msgServer) ClaimRewards(goCtx context.Context, msg *types.MsgClaimRewards) (*types.MsgClaimRewardsResponse, error) {
	ctx := sdk.UnwrapSDKContext(goCtx)

	depositor := msg.GetCreator()
	vaultId := msg.GetVaultID()

	depositorAddr, err := sdk.AccAddressFromBech32(depositor)
	if err != nil {
		return nil, err
	}

	switch vaultId {
	case oriontypes.ModuleName:
		qcoins, found := k.GetUserClaimAmt(ctx, depositor, vaultId)
		if found {
			rewardAccName := oriontypes.CreateOrionRewardGloablMaccName()
			err := k.bankKeeper.SendCoinsFromModuleToAccount(
				ctx,
				rewardAccName,
				depositorAddr,
				qcoins.Coins,
			)
			if err != nil {
				return nil, err
			}

			k.ClaimAll(ctx, depositor, vaultId)
		}

	default:
		return nil, types.ErrInvalidVaultId
	}

	ctx.EventManager().EmitEvent(
		types.CreateClaimRewardsEvent(ctx, depositorAddr, vaultId),
	)

	k.Logger(ctx).Info(
		"ClaimRewards",
		"Depositor", depositor,
		"VaultId", vaultId,
	)

	// TODO - Define and Emit Events
	return &types.MsgClaimRewardsResponse{}, nil
}
