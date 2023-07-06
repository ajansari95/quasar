use cosmwasm_std::{CheckedMultiplyRatioError, OverflowError, StdError};
use cw_utils::PaymentError;
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

    #[error("No swap reply data")]
    NoSwapReplyData,

    #[error("Output Denom not found")]
    OutputDenomNotFound,

    #[error("{0}")]
    OverflowError(#[from] OverflowError),

    #[error("{0}")]
    PaymentError(#[from] PaymentError),

    #[error("{0}")]
    CheckedMultiplyRatio(#[from] CheckedMultiplyRatioError),
}
