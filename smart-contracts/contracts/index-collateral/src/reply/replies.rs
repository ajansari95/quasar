use cosmwasm_schema::cw_serde;
use cosmwasm_std::{DepsMut, Env, Reply, Response};

use crate::execute::deposit::handle_swap_reply;
use crate::ContractError;
use num_enum::{FromPrimitive, IntoPrimitive};

// TODO write some tests to see if this replies structure works
#[cw_serde]
#[derive(FromPrimitive, IntoPrimitive)]
#[repr(u64)]
pub enum Replies {
    DepositSwap = 1,
    MintShare,
    #[default]
    Unknown,
}

pub fn handle_reply(deps: DepsMut, env: Env, msg: Reply) -> Result<Response, ContractError> {
    match msg.id.into() {
        Replies::DepositSwap => handle_swap_reply(deps, env, msg.result),
        Replies::MintShare => todo!(),
        Replies::Unknown => todo!(),
    }
}
