use cosmwasm_schema::cw_serde;
use cosmwasm_std::{DepsMut, Env, Reply, Response};

use crate::ContractError;

// TODO write some tests to see if this replies structure works
#[cw_serde]
pub enum Replies {
    Swap,
}

pub fn handle_reply(deps: DepsMut, _env: Env, msg: Reply) -> Result<Response, ContractError> {
    if msg.id == Replies::Swap as u64 {
        todo!()
    }
    todo!()
}
