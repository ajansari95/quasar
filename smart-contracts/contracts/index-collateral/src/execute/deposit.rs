#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{
    coin, Binary, Coin, CosmosMsg, Deps, DepsMut, Env, IbcMsg, IbcTimeout, MessageInfo, Response,
    StdResult, Storage, Uint128,
};
use cw_utils::one_coin;
use multihop_router::contract::handle_get_route;
use multihop_router::route::RouteId;
// use cw2::set_contract_version;

use crate::assets::UsedAssets;
use crate::error::ContractError;
use crate::execute::swap::batch_swap;
use crate::msg::{ExecuteMsg, InstantiateMsg, QueryMsg};
use crate::state::{ASSETS, BONDING_FUNDS, SWAP_CONFIG, USED_ASSETS};

pub(crate) fn execute_deposit(deps: DepsMut, info: MessageInfo) -> Result<Response, ContractError> {
    let coin = one_coin(&info)?;
    let assets = UsedAssets::from_state(deps.storage, USED_ASSETS)?;
    // the complete flow works like this:
    // swap the tokens to our desired format, after swapping, the rest of the steps have to be called in the reply
    let swap_conf = SWAP_CONFIG.load(deps.storage)?;
    let swap_executes = batch_swap(swap_conf, coin, assets);
    todo!()
}

// for a denom and an amount, get the path to the ICA address and the ICA address.
// send the tokens to the ICA addresses, no st  ate tracking is done here
pub(crate) fn send_to_ica(
    storage: &mut dyn Storage,
    denom: &str,
    amount: Uint128,
    timeout: IbcTimeout,
) -> Result<IbcMsg, ContractError> {
    // lookup the destination from assets
    let asset = ASSETS.load(storage, denom)?;

    // lookup the route to the destination from the router
    let route = handle_get_route(
        storage,
        RouteId::new(asset.destination, asset.denom.clone()),
    )?
    .route;

    BONDING_FUNDS.update(storage, denom, |old| -> Result<Uint128, ContractError> {
        Ok(old.unwrap_or(Uint128::zero()).checked_add(amount)?)
    })?;

    Ok(IbcMsg::Transfer {
        channel_id: route.channel,
        to_address: asset.deposit_ica,
        amount: coin(amount.u128(), asset.denom),
        timeout,
    })
}
