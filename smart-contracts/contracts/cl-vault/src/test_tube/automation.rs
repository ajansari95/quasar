#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use crate::msg::{ExecuteMsg, ExtensionQueryMsg, QueryMsg};
    use crate::query::AutomationResponse;
    use crate::test_tube::helpers::get_event_attributes_by_ty_and_key;
    use crate::test_tube::initialize::initialize::default_init;
    use cosmwasm_std::{Coin, Decimal};
    use osmosis_std::types::cosmos::base::v1beta1::Coin as OsmoCoin;
    use osmosis_std::types::osmosis::poolmanager::v1beta1::{
        MsgSwapExactAmountIn, SwapAmountInRoute,
    };
    use osmosis_test_tube::{Account, Module, PoolManager, Wasm};

    const DENOM_BASE: &str = "uatom";
    const DENOM_QUOTE: &str = "uosmo";
    const ACCOUNTS_NUM: u64 = 10;
    const ACCOUNTS_INIT_BALANCE: u128 = 1_000_000_000_000_000_000_000_000;
    const DEPOSIT_AMOUNT: u128 = 5_000_000;
    const SWAPS_AMOUNT: &str = "10000000000";
    const AUTOMATION_CYCLES: usize = 1000;

    #[test]
    #[ignore]
    fn test_execute_automation_works() {
        let (app, contract_address, cl_pool_id, admin) = default_init();
        let wasm = Wasm::new(&app);

        // Initialize accounts
        let accounts = app
            .init_accounts(
                &[
                    Coin::new(ACCOUNTS_INIT_BALANCE, DENOM_BASE),
                    Coin::new(ACCOUNTS_INIT_BALANCE, DENOM_QUOTE),
                ],
                ACCOUNTS_NUM,
            )
            .unwrap();

        // Depositing with users
        for account in &accounts {
            let _ = wasm
                .execute(
                    contract_address.as_str(),
                    &ExecuteMsg::ExactDeposit { recipient: None },
                    &[
                        Coin::new(DEPOSIT_AMOUNT, DENOM_BASE),
                        Coin::new(DEPOSIT_AMOUNT, DENOM_QUOTE),
                    ],
                    account,
                )
                .unwrap();
        }

        // Declare swapper account
        let swapper = app
            .init_account(&[
                Coin::new(ACCOUNTS_INIT_BALANCE, DENOM_BASE),
                Coin::new(ACCOUNTS_INIT_BALANCE, DENOM_QUOTE),
            ])
            .unwrap();
        println!("iii {:?}", swapper.address());

        for _ in 0..AUTOMATION_CYCLES {
            println!("iii {:?}", 1);

            // Swap to generate move range on previously created user positions
            PoolManager::new(&app)
                .swap_exact_amount_in(
                    MsgSwapExactAmountIn {
                        sender: swapper.address(),
                        routes: vec![SwapAmountInRoute {
                            pool_id: cl_pool_id,
                            token_out_denom: DENOM_BASE.to_string(),
                        }],
                        token_in: Some(OsmoCoin {
                            denom: DENOM_QUOTE.to_string(),
                            amount: SWAPS_AMOUNT.to_string(),
                        }),
                        token_out_min_amount: "1".to_string(),
                    },
                    &swapper,
                )
                .unwrap();

            // TODO: Query contract query_automation() to check if conditions are met based on AutomationConfig
            let query_automation: AutomationResponse = wasm
                .query(
                    contract_address.as_str(),
                    &QueryMsg::VaultExtension(ExtensionQueryMsg::Automation {}),
                )
                .unwrap();

            println!("query_automation {:?}", query_automation);

            // Only if
            if query_automation.should_adjust_range {
                let ratio_steps = vec![
                    Decimal::from_str("0.0000000001").unwrap(),
                    Decimal::from_str("0.00001").unwrap(),
                    Decimal::from_str("0.001").unwrap(),
                    Decimal::from_str("0.01").unwrap(),
                    Decimal::from_str("0.1").unwrap(),
                    Decimal::from_str("0.25").unwrap(),
                    Decimal::from_str("0.33").unwrap(),
                    Decimal::from_str("0.5").unwrap(),
                    Decimal::from_str("0.75").unwrap(),
                    Decimal::one(),
                ];

                for ratio in ratio_steps {
                    let result = wasm
                        .execute(
                            contract_address.as_str(),
                            &ExecuteMsg::VaultExtension(
                                crate::msg::ExtensionExecuteMsg::Automation {
                                    max_slippage: Decimal::from_str("0.995").unwrap(),
                                    ratio_of_swappable_funds_to_use: ratio,
                                    twap_window_seconds: 24 as u64,
                                },
                            ),
                            &[],
                            &admin,
                        )
                        .unwrap();
                    println!("execution {:?}", result);

                    // Extract the 'todo' attribute from the 'wasm' event
                    let todo = get_event_attributes_by_ty_and_key(&result, "wasm", vec!["todo"]);
                    assert_eq!(todo[0].value, "false".to_string());
                }
            } else {
                // TODO: Try to execute_automation() and unwrap_err() asserting the right error is retrieven
            }
        }
    }
}
