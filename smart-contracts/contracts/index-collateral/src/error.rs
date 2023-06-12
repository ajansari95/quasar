use cosmwasm_std::{OverflowError, StdError};
use thiserror::Error;

use multihop_router::ContractError as RouterError;

#[derive(Error, Debug)]
pub enum ContractError {
    #[error("{0}")]
    Std(#[from] StdError),

    #[error("Unauthorized")]
    Unauthorized {},

    #[error("{0}")]
    Router(#[from] RouterError),

    #[error("{0}")]
    OverflowError(#[from] OverflowError),
}
