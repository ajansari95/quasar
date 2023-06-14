use crate::{error::ContractError, assets::UsedAssets, state::SWAP_CONFIG};
use cosmwasm_std::{Coin, Decimal};
use swaprouter::msg::ExecuteMsg as SwapRouterExecute;

pub struct SwapConfig {
    twap_window: Option<u64>,
    slippage_percentage: Decimal
}

pub fn batch_swap(config: SwapConfig, swap_coin: Coin, ratio: UsedAssets) -> Result<Vec<SwapRouterExecute>, ContractError> {
    // inputs is a vec of amount of swap_coin to swap and the output denom
    let inputs: Result<Vec<SwapRouterExecute>, ContractError> = ratio.into_assets().into_iter().map(|val| -> Result<SwapRouterExecute, ContractError> {
        let amount = val.ratio.checked_mul(Decimal::new(swap_coin.amount))?;
        Ok(SwapRouterExecute::Swap {
            input_coin: Coin { denom: swap_coin.denom.clone(), amount: amount.to_uint_floor() },
            output_denom: val.asset.denom,
            // TODO make window_seconds and slippage_percentage configurable
            slippage: swaprouter::Slippage::Twap { window_seconds: config.twap_window, slippage_percentage: config.slippage_percentage },
        })
    }).collect();
    
    inputs
}