use crate::{
    state::Config,
    ContractError,
};
use cosmwasm_std::{Addr, DepsMut, MessageInfo, Response};

pub fn execute_update_admin(
    _deps: DepsMut,
    _info: MessageInfo,
    address: Addr,
) -> Result<Response, ContractError> {
    //todo!();

    Ok(Response::new()
        .add_attribute("method", "execute")
        .add_attribute("action", "update_admin")
        .add_attribute("address", address))
}

pub fn execute_update_config(
    _deps: DepsMut,
    _info: MessageInfo,
    config: Config,
) -> Result<Response, ContractError> {
    //todo!();

    Ok(Response::new()
        .add_attribute("method", "execute")
        .add_attribute("action", "update_config"))
    //.add_attribute("config", config))
}

pub fn execute_add_validators(
    _deps: DepsMut,
    _info: MessageInfo,
    addresses: Vec<Addr>,
) -> Result<Response, ContractError> {
    //todo!();

    Ok(Response::new()
        .add_attribute("method", "execute")
        .add_attribute("action", "add_validators"))
    //.add_attribute("addresses", addresses))
}

pub fn execute_remove_validators(
    _deps: DepsMut,
    _info: MessageInfo,
    addresses: Vec<Addr>,
) -> Result<Response, ContractError> {
    //todo!();

    Ok(Response::new()
        .add_attribute("method", "execute")
        .add_attribute("action", "remove_validators"))
    // .add_attribute("addresses", addresses))
}

#[cfg(test)]
mod tests {
    // use crate::msg::InstantiateMsg;

    // use super::*;
    // use cosmwasm_std::testing::{mock_dependencies, mock_env, mock_info};
    // use cosmwasm_std::{coins, from_binary};

    // #[test]
    // fn reset() {
    //     let mut deps = mock_dependencies();

    //     let msg = InstantiateMsg { count: 17 };
    //     let info = mock_info("creator", &coins(2, "token"));
    //     let _res = instantiate(deps.as_mut(), mock_env(), info, msg).unwrap();

    //     // beneficiary can release it
    //     let unauth_info = mock_info("anyone", &coins(2, "token"));
    //     let msg = ExecuteMsg::Reset { count: 5 };
    //     let res = execute(deps.as_mut(), mock_env(), unauth_info, msg);
    //     match res {
    //         Err(ContractError::Unauthorized {}) => {}
    //         _ => panic!("Must return unauthorized error"),
    //     }

    //     // only the original creator can reset the counter
    //     let auth_info = mock_info("creator", &coins(2, "token"));
    //     let msg = ExecuteMsg::Reset { count: 5 };
    //     let _res = execute(deps.as_mut(), mock_env(), auth_info, msg).unwrap();

    //     // should now be 5
    //     let res = query(deps.as_ref(), mock_env(), QueryMsg::GetCount {}).unwrap();
    //     let value: GetCountResponse = from_binary(&res).unwrap();
    //     assert_eq!(5, value.count);
    // }
}
