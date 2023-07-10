use crate::{assets::UsedAssets, error::ContractError};
use cosmwasm_schema::cw_serde;
use cosmwasm_std::{coin, Addr, Coin, Decimal, Fraction, Uint128};
use swaprouter::{msg::ExecuteMsg as SwapRouterExecute, Slippage};

#[cw_serde]
pub struct SwapConfig {
    pub(crate) router_addr: Addr,
    // the twap window used in seconds
    pub(crate) twap_window: u64,
    pub(crate) slippage_percentage: Decimal,
}

pub fn batch_swap(swap_coin: Coin, ratio: UsedAssets) -> Result<Vec<(Swap, Coin)>, ContractError> {
    ratio
        .into_assets()
        .into_iter()
        .map(|val| -> Result<(Swap, Coin), ContractError> {
            let amount = swap_coin
                .amount
                .checked_multiply_ratio(val.ratio.numerator(), val.ratio.denominator())?;
            let swap = Swap::new(
                Coin {
                    denom: swap_coin.denom.clone(),
                    amount,
                },
                val.asset.denom,
            );
            let coin = coin(amount.u128(), swap_coin.denom.clone());
            Ok((swap, coin))
        })
        .collect()
}

#[cw_serde]
pub struct Swap {
    pub input: Coin,
    pub output_denom: String,
}

impl Swap {
    pub fn new(input: Coin, output_denom: String) -> Swap {
        Swap {
            input,
            output_denom,
        }
    }

    /// Transforms the swap into a SwapRouterExecute::Swap msg, to actually use the swaprouter
    /// these can be directly dispatched after wrapping in a CosmosMsg
    pub fn into_execute(self, slippage: Slippage) -> SwapRouterExecute {
        SwapRouterExecute::Swap {
            input_coin: self.input,
            output_denom: self.output_denom,
            slippage: slippage,
        }
    }
}

#[cw_serde]
pub struct SwapResult {
    pub swap: Swap,
    pub result: Option<Uint128>,
}

impl SwapResult {
    pub fn new(swap: Swap, result: Option<Uint128>) -> SwapResult {
        SwapResult { swap, result }
    }
}

#[cfg(test)]
mod tests {
    use crate::assets::{Asset, AssetInfo};

    use super::*;

    use cosmwasm_std::{Fraction, Uint128};
    use multihop_router::route::Destination;
    use proptest::{collection, prelude::*};

    prop_compose! {
        fn arb_swap_coin()(denom in any::<String>(), amount in 1..100 as u64) -> Coin {
            Coin{
                denom ,
                amount: Uint128::new(amount.into()) ,
            }
        }
    }

    prop_compose! {
        fn arb_asset()(denom in any::<String>(), destination in any::<String>(), deposit_ica in any::<String>()) -> Asset {
            Asset::new(denom, destination, deposit_ica)
        }
    }

    prop_compose! {
        fn arb_asset_info()(denom in any::<String>(), destination in any::<String>(), deposit_ica in any::<String>(), raw_ratio in 1..u64::MAX) -> AssetInfo {
            AssetInfo::new(denom, destination, deposit_ica, Uint128::new(raw_ratio.into()))
        }
    }

    prop_compose! {
        fn arb_used_assets()(assets in  collection::vec(arb_asset_info(), 1..100), ) -> UsedAssets {
            UsedAssets::with_assets(assets)
        }
    }

    proptest! {
        #[test]
        fn batch_swap_works(swap_coin in arb_swap_coin(), used_assets in arb_used_assets()) {
            let resp = batch_swap(swap_coin.clone(), used_assets.clone());

            prop_assert!(resp.is_ok());
            let msgs = resp.unwrap();
            prop_assert_eq!(msgs.len(), used_assets.assets().len());
            for (msg, asset) in msgs.iter().zip(used_assets.assets().iter()) {
                prop_assert_eq!(msg.1.amount, swap_coin.amount.multiply_ratio(asset.ratio.numerator(), asset.ratio.denominator()))

            }
        }
    }

    #[test]
    fn single_token_output() {
        let swap_coin = coin(40, "");
        let used_assets = UsedAssets::with_assets(vec![AssetInfo {
            asset: Asset {
                denom: "".to_string(),
                destination: Destination::new(""),
                deposit_ica: "".to_string(),
            },
            ratio: Decimal::new(Uint128::new(1)),
            raw_ratio: Uint128::new(1),
        }]);
        let resp = batch_swap(swap_coin, used_assets).unwrap();
        assert_eq!(resp[0].1.amount, Uint128::new(40))
    }
}
