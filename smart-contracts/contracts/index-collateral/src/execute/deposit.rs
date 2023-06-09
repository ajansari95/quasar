#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{Binary, Coin, Deps, DepsMut, Env, MessageInfo, Response, StdResult};
// use cw2::set_contract_version;

use crate::error::ContractError;
use crate::msg::{ExecuteMsg, InstantiateMsg, QueryMsg};

pub fn execute_deposit(coins: Vec<Coin>) -> Result<Response, ContractError> {
    // swap the tokens to our desired format

    // for each asset, get the path to the ICA address and the ICA address

    // send the tokens to the ICA addresses

    // mint shares on the Quasar contract
    todo!()
}
