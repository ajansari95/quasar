mod concentrated_liquidity;
pub mod contract;
mod error;
pub mod helpers;
mod math;
mod merge;
pub mod msg;
mod query;
mod reply;
mod rewards;
pub mod state;
mod swap;
mod vault;

pub use crate::error::ContractError;

#[cfg(test)]
mod test_tube;

#[macro_export]
macro_rules! debug {
    ($deps: ident, $tag:literal, $($arg:tt)*) => {
        $deps.api.debug(format!(concat!($tag, " :{:?}"), $($arg)*).as_str())
    };
}
