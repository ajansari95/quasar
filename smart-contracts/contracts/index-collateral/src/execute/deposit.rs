#[cfg(not(feature = "library"))]
use cosmwasm_std::{
    coin, Coin, IbcMsg, IbcTimeout, Response, Storage, Uint128,
};
use multihop_router::contract::handle_get_route;
use multihop_router::route::RouteId;

use crate::error::ContractError;
use crate::state::{ASSETS, BONDING_FUNDS};

pub(crate) fn execute_deposit(coins: Vec<Coin>) -> Result<Response, ContractError> {
    // the complete flow works like this:
    // swap the tokens to our desired format, after swapping, the rest of the steps have to be called in the reply

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
