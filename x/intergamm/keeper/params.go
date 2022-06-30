package keeper

import (
	"github.com/abag/quasarnode/x/intergamm/types"
	sdk "github.com/cosmos/cosmos-sdk/types"
)

// GetParams get all parameters as types.Params
func (k Keeper) GetParams(ctx sdk.Context) types.Params {
	return types.NewParams(k.OsmoTokenTransferChannels(ctx))
}

// SetParams set the params
func (k Keeper) SetParams(ctx sdk.Context, params types.Params) {
	k.paramstore.SetParamSet(ctx, &params)
}

// OsmoTokenTransferChannels returns the  other chains token transfer channel to osmosis
func (k Keeper) OsmoTokenTransferChannels(ctx sdk.Context) (res map[string]string) {
	k.paramstore.Get(ctx, types.KeyOsmoTokenTransferChannels, &res)
	return
}
