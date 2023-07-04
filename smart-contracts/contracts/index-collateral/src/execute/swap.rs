use crate::{assets::UsedAssets, error::ContractError};
use cosmwasm_schema::cw_serde;
use cosmwasm_std::{coin, Addr, Coin, Decimal, Fraction};
use swaprouter::msg::ExecuteMsg as SwapRouterExecute;

#[cw_serde]
pub struct SwapConfig {
    pub(crate) router_addr: Addr,
    twap_window: Option<u64>,
    slippage_percentage: Decimal,
}

pub fn batch_swap(
    config: &SwapConfig,
    swap_coin: Coin,
    ratio: UsedAssets,
) -> Result<Vec<(SwapRouterExecute, Coin)>, ContractError> {
    // inputs is a vec of amount of swap_coin to swap and the output denom
    let inputs: Result<Vec<(SwapRouterExecute, Coin)>, ContractError> = ratio
        .into_assets()
        .into_iter()
        .map(|val| -> Result<(SwapRouterExecute, Coin), ContractError> {
            let amount = swap_coin.amount.checked_multiply_ratio(val.ratio.numerator(), val.ratio.denominator())?;
            let msg = SwapRouterExecute::Swap {
                input_coin: Coin {
                    denom: swap_coin.denom.clone(),
                    amount,
                },
                output_denom: val.asset.denom,
                // TODO make window_seconds and slippage_percentage configurable
                slippage: swaprouter::Slippage::Twap {
                    window_seconds: config.twap_window,
                    slippage_percentage: config.slippage_percentage,
                },
            };
            let coin = coin(amount.u128(), swap_coin.denom.clone());
            Ok((msg, coin))
        })
        .collect();

    inputs
}

#[cfg(test)]
mod tests {
    use crate::assets::{Asset, AssetInfo};

    use super::*;

    use cosmwasm_std::{Uint128, Fraction};
    use multihop_router::route::Destination;
    use proptest::{prelude::*, collection};

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

    proptest!{
        #[test]
        fn batch_swap_works(swap_coin in arb_swap_coin(), used_assets in arb_used_assets()) {
            let config = SwapConfig { router_addr: Addr::unchecked("addy"), twap_window: Some(5), slippage_percentage: Decimal::percent(5) };

            let resp = batch_swap(&config, swap_coin.clone(), used_assets.clone());

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
        let config = SwapConfig { router_addr: Addr::unchecked("addy"), twap_window: Some(5), slippage_percentage: Decimal::percent(5) };
        let swap_coin = coin(40, "");
        let used_assets = UsedAssets::with_assets(vec![AssetInfo { asset: Asset { denom: "".to_string(), destination: Destination::new(""), deposit_ica: "".to_string() }, ratio: Decimal::new(Uint128::new(1)), raw_ratio: Uint128::new(1) }]);
        let resp = batch_swap(&config, swap_coin, used_assets).unwrap();
        assert_eq!(resp[0].1.amount, Uint128::new(40))
    }
}