#[cfg(test)]
mod tests {
    use crate::msg::ExecuteMsg;
    use crate::test_tube::initialize::initialize::default_init;
    use cosmwasm_std::Coin;
    use osmosis_std::types::cosmos::base::v1beta1::Coin as OsmoCoin;
    use osmosis_std::types::osmosis::poolmanager::v1beta1::{
        MsgSwapExactAmountIn, SwapAmountInRoute,
    };
    use osmosis_test_tube::{Account, Module, PoolManager, Wasm};

    const _ADMIN_BALANCE_AMOUNT: u128 = 340282366920938463463374607431768211455u128;
    const _TOKENS_PROVIDED_AMOUNT: &str = "1000000000000";
    const DENOM_BASE: &str = "uatom";
    const DENOM_QUOTE: &str = "uosmo";

    #[test]
    #[ignore]
    fn test_rewards_single_distribute_claim() {
        let (app, contract_address, cl_pool_id, _admin) = default_init();
        let alice = app
            .init_account(&[
                Coin::new(1_000_000_000_000, DENOM_BASE),
                Coin::new(1_000_000_000_000, DENOM_QUOTE),
            ])
            .unwrap();

        let bob = app
            .init_account(&[
                Coin::new(1_000_000_000_000, DENOM_BASE),
                Coin::new(1_000_000_000_000, DENOM_QUOTE),
            ])
            .unwrap();

        let wasm = Wasm::new(&app);

        let _ = wasm
            .execute(
                contract_address.as_str(),
                &ExecuteMsg::ExactDeposit { recipient: None },
                &[
                    Coin::new(5_000_000, DENOM_BASE),
                    Coin::new(5_000_000, DENOM_QUOTE),
                ],
                &alice,
            )
            .unwrap();

        // do a bunch of swaps to get some swap fees
        PoolManager::new(&app)
            .swap_exact_amount_in(
                MsgSwapExactAmountIn {
                    sender: bob.address(),
                    routes: vec![SwapAmountInRoute {
                        pool_id: cl_pool_id,
                        token_out_denom: DENOM_BASE.to_string(),
                    }],
                    token_in: Some(OsmoCoin {
                        denom: DENOM_QUOTE.to_string(),
                        amount: "100".to_string(),
                    }),
                    token_out_min_amount: "1".to_string(),
                },
                &bob,
            )
            .unwrap();

        PoolManager::new(&app)
            .swap_exact_amount_in(
                MsgSwapExactAmountIn {
                    sender: bob.address(),
                    routes: vec![SwapAmountInRoute {
                        pool_id: cl_pool_id,
                        token_out_denom: DENOM_BASE.to_string(),
                    }],
                    token_in: Some(OsmoCoin {
                        denom: DENOM_QUOTE.to_string(),
                        amount: "100".to_string(),
                    }),
                    token_out_min_amount: "1".to_string(),
                },
                &bob,
            )
            .unwrap();

        PoolManager::new(&app)
            .swap_exact_amount_in(
                MsgSwapExactAmountIn {
                    sender: bob.address(),
                    routes: vec![SwapAmountInRoute {
                        pool_id: cl_pool_id,
                        token_out_denom: DENOM_BASE.to_string(),
                    }],
                    token_in: Some(OsmoCoin {
                        denom: DENOM_QUOTE.to_string(),
                        amount: "100".to_string(),
                    }),
                    token_out_min_amount: "1".to_string(),
                },
                &bob,
            )
            .unwrap();

        PoolManager::new(&app)
            .swap_exact_amount_in(
                MsgSwapExactAmountIn {
                    sender: bob.address(),
                    routes: vec![SwapAmountInRoute {
                        pool_id: cl_pool_id,
                        token_out_denom: DENOM_QUOTE.to_string(),
                    }],
                    token_in: Some(OsmoCoin {
                        denom: DENOM_BASE.to_string(),
                        amount: "100".to_string(),
                    }),
                    token_out_min_amount: "1".to_string(),
                },
                &bob,
            )
            .unwrap();

        PoolManager::new(&app)
            .swap_exact_amount_in(
                MsgSwapExactAmountIn {
                    sender: bob.address(),
                    routes: vec![SwapAmountInRoute {
                        pool_id: cl_pool_id,
                        token_out_denom: DENOM_QUOTE.to_string(),
                    }],
                    token_in: Some(OsmoCoin {
                        denom: DENOM_BASE.to_string(),
                        amount: "100".to_string(),
                    }),
                    token_out_min_amount: "1".to_string(),
                },
                &bob,
            )
            .unwrap();

        PoolManager::new(&app)
            .swap_exact_amount_in(
                MsgSwapExactAmountIn {
                    sender: bob.address(),
                    routes: vec![SwapAmountInRoute {
                        pool_id: cl_pool_id,
                        token_out_denom: DENOM_QUOTE.to_string(),
                    }],
                    token_in: Some(OsmoCoin {
                        denom: DENOM_BASE.to_string(),
                        amount: "100".to_string(),
                    }),
                    token_out_min_amount: "1".to_string(),
                },
                &bob,
            )
            .unwrap();

        let _res = wasm
            .execute(
                contract_address.as_str(),
                &ExecuteMsg::VaultExtension(crate::msg::ExtensionExecuteMsg::DistributeRewards {}),
                &[],
                &alice,
            )
            .unwrap();
    }
}
