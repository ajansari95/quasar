use std::str::FromStr;

use cosmwasm_std::{coin, Coin, Decimal, Uint128};
use osmosis_std::types::{
    cosmos::base::v1beta1,
    osmosis::{
        concentratedliquidity::{
            poolmodel::concentrated::v1beta1::MsgCreateConcentratedPool,
            v1beta1::{MsgCreatePosition, Pool, PoolsRequest},
        },
        poolmanager::v1beta1::{MsgSwapExactAmountIn, SwapAmountInRoute},
    },
};
use osmosis_test_tube::{Account, ConcentratedLiquidity, Module, PoolManager, Wasm};

use crate::{
    msg::{ExecuteMsg, ModifyRange, QueryMsg},
    query::PositionsResponse,
    tests::initialize::init_test_contract,
};

use prost::Message;

#[test]
#[ignore]
fn move_range_works() {
    let (app, contract, cl_pool_id, admin) = init_test_contract(
        "./test-tube-build/wasm32-unknown-unknown/release/cl_vault.wasm",
        &[
            Coin::new(1_000_000_000_000, "uatom"),
            Coin::new(1_000_000_000_000, "uosmo"),
        ],
        MsgCreateConcentratedPool {
            sender: "overwritten".to_string(),
            denom0: "uatom".to_string(),
            denom1: "uosmo".to_string(),
            tick_spacing: 100,
            spread_factor: Decimal::from_str("0.0001").unwrap().atomics().to_string(),
        },
        21205000,
        27448000,
        vec![
            v1beta1::Coin {
                denom: "uatom".to_string(),
                amount: "10000000000".to_string(),
            },
            v1beta1::Coin {
                denom: "uosmo".to_string(),
                amount: "10000000000".to_string(),
            },
        ],
        Uint128::zero(),
        Uint128::zero(),
    );
    let alice = app
        .init_account(&[
            Coin::new(1_000_000_000_000, "uatom"),
            Coin::new(1_000_000_000_000, "uosmo"),
        ])
        .unwrap();

    let wasm = Wasm::new(&app);
    let cl = ConcentratedLiquidity::new(&app);

    // do a swap to move the cur tick
    let pm = PoolManager::new(&app);
    pm.swap_exact_amount_in(
        MsgSwapExactAmountIn {
            sender: alice.address(),
            routes: vec![SwapAmountInRoute {
                pool_id: cl_pool_id,
                token_out_denom: "uatom".to_string(),
            }],
            token_in: Some(v1beta1::Coin {
                denom: "uosmo".to_string(),
                amount: "1000".to_string(),
            }),
            token_out_min_amount: "1".to_string(),
        },
        &alice,
    )
    .unwrap();

    let pools = cl.query_pools(&PoolsRequest { pagination: None }).unwrap();
    let _pool = Pool::decode(pools.pools[0].value.as_slice()).unwrap();

    let before_position: PositionsResponse = wasm
        .query(
            contract.as_str(),
            &QueryMsg::VaultExtension(crate::msg::ExtensionQueryMsg::ConcentratedLiquidity(
                crate::msg::ClQueryMsg::Positions {},
            )),
        )
        .unwrap();

    let _result = wasm
        .execute(
            contract.as_str(),
            &ExecuteMsg::VaultExtension(crate::msg::ExtensionExecuteMsg::ModifyRange(
                ModifyRange::MovePosition {
                    old_position_id: before_position.positions[0].position_id,
                    new_lower_price: Decimal::from_str("400").unwrap(),
                    new_upper_price: Decimal::from_str("1466").unwrap(),
                    max_slippage: Decimal::permille(5),
                },
            )),
            &[],
            &admin,
        )
        .unwrap();

    let _after_position: PositionsResponse = wasm
        .query(
            contract.as_str(),
            &QueryMsg::VaultExtension(crate::msg::ExtensionQueryMsg::ConcentratedLiquidity(
                crate::msg::ClQueryMsg::Positions {},
            )),
        )
        .unwrap();
}

#[test]
#[ignore]
fn move_range_same_single_side_works() {
    let (app, contract, cl_pool_id, admin) = init_test_contract(
        "./test-tube-build/wasm32-unknown-unknown/release/cl_vault.wasm",
        &[
            Coin::new(1_000_000_000_000, "uatom"),
            Coin::new(1_000_000_000_000, "uosmo"),
        ],
        MsgCreateConcentratedPool {
            sender: "overwritten".to_string(),
            denom0: "uatom".to_string(),
            denom1: "uosmo".to_string(),
            tick_spacing: 100,
            spread_factor: Decimal::from_str("0.0001").unwrap().atomics().to_string(),
        },
        21205000,
        27448000,
        vec![
            v1beta1::Coin {
                denom: "uatom".to_string(),
                amount: "10000000000".to_string(),
            },
            v1beta1::Coin {
                denom: "uosmo".to_string(),
                amount: "10000000000".to_string(),
            },
        ],
        Uint128::zero(),
        Uint128::zero(),
    );
    let alice = app
        .init_account(&[
            Coin::new(1_000_000_000_000, "uatom"),
            Coin::new(1_000_000_000_000, "uosmo"),
        ])
        .unwrap();

    let wasm = Wasm::new(&app);
    let cl = ConcentratedLiquidity::new(&app);

    // do a swap to move the cur tick
    let pm = PoolManager::new(&app);
    pm.swap_exact_amount_in(
        MsgSwapExactAmountIn {
            sender: alice.address(),
            routes: vec![SwapAmountInRoute {
                pool_id: cl_pool_id,
                token_out_denom: "uatom".to_string(),
            }],
            token_in: Some(v1beta1::Coin {
                denom: "uosmo".to_string(),
                amount: "1000".to_string(),
            }),
            token_out_min_amount: "1".to_string(),
        },
        &alice,
    )
    .unwrap();

    let pools = cl.query_pools(&PoolsRequest { pagination: None }).unwrap();
    let pool = Pool::decode(pools.pools[0].value.as_slice()).unwrap();

    let before_position: PositionsResponse = wasm
        .query(
            contract.as_str(),
            &QueryMsg::VaultExtension(crate::msg::ExtensionQueryMsg::ConcentratedLiquidity(
                crate::msg::ClQueryMsg::Positions {},
            )),
        )
        .unwrap();

    let _result = wasm
        .execute(
            contract.as_str(),
            &ExecuteMsg::VaultExtension(crate::msg::ExtensionExecuteMsg::ModifyRange(
                ModifyRange::MovePosition {
                    old_position_id: before_position.positions[0].position_id,
                    new_lower_price: Decimal::from_str("20.71").unwrap(),
                    new_upper_price: Decimal::from_str("45").unwrap(),
                    max_slippage: Decimal::permille(5),
                },
            )),
            &[],
            &admin,
        )
        .unwrap();
}

/*
we try the following position from https://docs.google.com/spreadsheets/d/1xPsKsQkM0apTZQPBBwVlEyB5Sk31sw6eE8U0FgnTWUQ/edit?usp=sharing
lower_price:   4500
current_price: 4692.937
upper_price:   5500

the spreadsheet says we need to leave 42806.28569 in token x and swap over 157193.7143
157193.7143 / 4692.937 = 33.49580749
both token amounts are used in 5 decimals, since the leftover amount is in 5 decimals
so we want to deposit 4280628569 and 3349580
*/
#[test]
#[ignore]
fn test_swap_math_poc() {
    let (app, _contract, _cl_pool_id, _admin) = init_test_contract(
        "./test-tube-build/wasm32-unknown-unknown/release/cl_vault.wasm",
        &[
            Coin::new(1_000_000_000_000, "uatom"),
            Coin::new(1_000_000_000_000, "uosmo"),
        ],
        MsgCreateConcentratedPool {
            sender: "overwritten".to_string(),
            denom0: "uatom".to_string(), //token0 is uatom
            denom1: "uosmo".to_string(), //token1 is uosmo
            tick_spacing: 100,
            spread_factor: Decimal::from_str("0.0001").unwrap().atomics().to_string(),
        },
        30500000, // 4500
        31500000, // 5500
        vec![
            v1beta1::Coin {
                denom: "uatom".to_string(),
                amount: "1000000".to_string(),
            },
            v1beta1::Coin {
                denom: "uosmo".to_string(),
                amount: "1000000".to_string(),
            },
        ],
        Uint128::zero(),
        Uint128::zero(),
    );
    let alice = app
        .init_account(&[
            Coin::new(1_000_000_000_000, "uatom"),
            Coin::new(1_000_000_000_000, "uosmo"),
        ])
        .unwrap();

    let cl = ConcentratedLiquidity::new(&app);

    let pools = cl.query_pools(&PoolsRequest { pagination: None }).unwrap();
    let pool: Pool = Pool::decode(pools.pools[0].value.as_slice()).unwrap();

    // from the spreadsheet
    // create a basic position on the pool
    let initial_position = MsgCreatePosition {
        pool_id: pool.id,
        sender: alice.address(),
        lower_tick: 30500000,
        upper_tick: 31500000,
        tokens_provided: vec![
            coin(3349580, "uatom").into(),
            coin(4280628569, "uosmo").into(),
        ],
        token_min_amount0: "0".to_string(),
        token_min_amount1: "0".to_string(),
    };
    let position = cl.create_position(initial_position, &alice).unwrap();
}
