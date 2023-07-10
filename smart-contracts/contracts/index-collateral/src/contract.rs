#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{
    Binary, CosmosMsg, Deps, DepsMut, Env, MessageInfo, Reply, Response, StdResult,
};
use osmosis_std::types::osmosis::tokenfactory::v1beta1::MsgCreateDenom;
// use cw2::set_contract_version;

use crate::error::ContractError;
use crate::msg::{ExecuteMsg, InstantiateMsg, QueryMsg};
use crate::reply::replies::handle_reply;
use crate::state::SHARE_DENOM;

use multihop_router::contract::execute as router_execute;

/*
// version info for migration info
const CONTRACT_NAME: &str = "crates.io:index-collateral";
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");
*/

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut,
    env: Env,
    _info: MessageInfo,
    msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    SHARE_DENOM.save(deps.storage, &msg.collateral_denom)?;

    // construct message and convert them into cosmos message
    // (notice `CosmosMsg` type and `.into()`)
    let sender = env.contract.address.into();
    let msg_create_denom: CosmosMsg = MsgCreateDenom {
        sender,
        subdenom: msg.collateral_denom,
    }
    .into();

    Ok(Response::new().add_message(msg_create_denom))
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    match msg {
        ExecuteMsg::AcceptCollateral {} => todo!(),
        ExecuteMsg::AcceptRewards {} => todo!(),
        ExecuteMsg::Deposit {} => todo!(),
        ExecuteMsg::Redeem {} => todo!(),
        ExecuteMsg::Withdraw {} => todo!(),
        // TODO see if we make the collateral contract admin of the router or keep the sender here as sender
        ExecuteMsg::Router(msg) => Ok(router_execute(deps, env, info, msg)?),
    }
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(_deps: Deps, _env: Env, _msg: QueryMsg) -> StdResult<Binary> {
    unimplemented!()
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn reply(deps: DepsMut, env: Env, msg: Reply) -> Result<Response, ContractError> {
    handle_reply(deps, env, msg)
}

#[cfg(test)]
mod tests {}
