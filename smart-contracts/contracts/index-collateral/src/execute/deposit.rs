#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{Binary, Coin, Deps, DepsMut, Env, MessageInfo, Response, StdResult, CosmosMsg, IbcMsg, Storage, Uint128, IbcTimeout, coin};
use multihop_router::contract::handle_get_route;
use multihop_router::route::RouteId;
// use cw2::set_contract_version;

use crate::error::ContractError;
use crate::msg::{ExecuteMsg, InstantiateMsg, QueryMsg};
use crate::state::ASSETS;

pub fn execute_deposit(coins: Vec<Coin>) -> Result<Response, ContractError> {
    // the complete flow works like this:
    // swap the tokens to our desired format, after swapping, the rest of the steps have to be called in the reply
    

    todo!()
}

// for an asset and an amount, get the path to the ICA address and the ICA address.
// send the tokens to the ICA addresses, no st  ate tracking is done here
fn send_to_ica(storage: &mut dyn Storage, asset: String, amount: Uint128, timeout: IbcTimeout) -> Result<IbcMsg, ContractError> {
    // lookup the destination from assets
    let asset = ASSETS.load(storage, asset)?;
    // lookup the route to the destination from the router
    let route = handle_get_route(storage, RouteId::new(asset.destination, asset.denom.clone()))?.route;
    
    Ok(IbcMsg::Transfer { channel_id: route.channel, to_address: asset.deposit_ica, amount: coin(amount.u128(), asset.denom), timeout })
}
