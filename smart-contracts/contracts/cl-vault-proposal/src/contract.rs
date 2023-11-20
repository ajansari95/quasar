#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{to_json_binary, Binary, Deps, DepsMut, Env, MessageInfo, Response, StdResult};
use cw2::set_contract_version;

use crate::error::ContractError;
use crate::execute::{
    execute_add_validators, execute_proposal, execute_remove_validators, execute_update_admin,
    execute_update_config,
};
use crate::msg::{ExecuteMsg, InstantiateMsg, QueryMsg, ReplyMsg};
use crate::query::{query_admin, query_config, query_validators};

// version info for migration info
const CONTRACT_NAME: &str = "crates.io:cl-vault-proposal";
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;

    Ok(Response::new()
        .add_attribute("method", "instantiate")
        .add_attribute("owner", info.sender))
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    match msg {
        ExecuteMsg::UpdateAdmin { address } => execute_update_admin(deps, info, address),
        ExecuteMsg::UpdateConfig { config } => execute_update_config(deps, info, config),
        ExecuteMsg::AddValidators { addresses } => execute_add_validators(deps, info, addresses),
        ExecuteMsg::RemoveValidators { addresses } => {
            execute_remove_validators(deps, info, addresses)
        }
        ExecuteMsg::Proposal { proposal } => execute_proposal(deps, info, proposal),
    }
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn reply(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    msg: ReplyMsg,
) -> Result<Response, ContractError> {
    match msg {
        ReplyMsg::Proposal { proposal, valid, invalid } => execute_proposal(deps, info, proposal, valid, invalid),
    }
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::GetAdmin {} => to_json_binary(&query_admin(deps)?),
        QueryMsg::GetConfig {} => to_json_binary(&query_config(deps)?),
        QueryMsg::GetValidators {} => to_json_binary(&query_validators(deps)?),
    }
}

#[cfg(test)]
mod tests {
    // use crate::msg::GetCountResponse;

    // use super::*;
    // use cosmwasm_std::testing::{mock_dependencies, mock_env, mock_info};
    // use cosmwasm_std::{coins, to_json_binary};

    // #[test]
    // fn proper_initialization() {
    //     let mut deps = mock_dependencies();

    //     let msg = InstantiateMsg { count: 17 };
    //     let info = mock_info("creator", &coins(1000, "earth"));

    //     // we can just call .unwrap() to assert this was a success
    //     let res = instantiate(deps.as_mut(), mock_env(), info, msg).unwrap();
    //     assert_eq!(0, res.messages.len());

    //     // it worked, let's query the state
    //     let res = query(deps.as_ref(), mock_env(), QueryMsg::GetCount {}).unwrap();
    //     let value: GetCountResponse = to_json_binary(&res).unwrap();
    //     assert_eq!(17, value.count);
    // }
}
