use std::str::FromStr;

use cosmwasm_std::{coin, Coin, Decimal, Uint128, assert_approx_eq};

use osmosis_std::types::{
    cosmos::bank::v1beta1::{MsgSend, QueryAllBalancesRequest},
    osmosis::{
        concentratedliquidity::v1beta1::PositionByIdRequest, poolmanager::v1beta1::SpotPriceRequest,
    },
};
use osmosis_test_tube::{Account, Bank, ConcentratedLiquidity, Module, PoolManager, Wasm};

use crate::{
    msg::{ExecuteMsg, ExtensionQueryMsg, QueryMsg},
    query::{PositionResponse, UserBalanceResponse},
    tests::{default_init, helpers::{get_share_price_in_asset0, get_full_positions, get_total_assets}},
};

#[test]
fn multi_position_deposit_works() {
    let (app, contract_address, cl_pool_id, admin) = default_init();
    let alice = app
        .init_account(&[
            Coin::new(1_000_000_000_000, "uatom"),
            Coin::new(1_000_000_000_000, "uosmo"),
        ])
        .unwrap();
    let bob = app
        .init_account(&[
            Coin::new(1_000_000_000_000, "uatom"),
            Coin::new(1_000_000_000_000, "uosmo"),
        ])
        .unwrap();

    let bank = Bank::new(&app);
    // our initial balance, 89874uosmo
    let balances = bank
        .query_all_balances(&QueryAllBalancesRequest {
            address: contract_address.to_string(),
            pagination: None,
        })
        .unwrap();

    // make sure we have some fee uosmo and uatom to create the new position
    bank.send(
        MsgSend {
            from_address: admin.address(),
            to_address: contract_address.to_string(),
            amount: vec![
                Coin::new(1_000_000_000, "uatom").into(),
                Coin::new(1_000_000_000, "uosmo").into(),
            ],
        },
        &admin,
    )
    .unwrap();

    let wasm = Wasm::new(&app);

    let total_assets = get_total_assets(&wasm, contract_address.as_str()).unwrap();

    // create a new position
    let _res = wasm
        .execute(
            contract_address.as_str(),
            &ExecuteMsg::VaultExtension(crate::msg::ExtensionExecuteMsg::ModifyRange(
                crate::msg::ModifyRange::CreatePosition {
                    lower_price: Decimal::from_str("0.90").unwrap(),
                    upper_price: Decimal::from_str("1.1").unwrap(),
                    ratio: Uint128::one(),
                },
            )),
            &vec![],
            &admin,
        )
        .unwrap();

    let current_assets = get_total_assets(&wasm, contract_address.as_str()).unwrap();
    assert_approx_eq!(total_assets.0.amount, current_assets.0.amount, "0.000001");
    assert_approx_eq!(total_assets.1.amount, current_assets.1.amount, "0.000001");


    // create a new position
    // this introduction should not introduce new funds as long as we free up some funds first
    let positions = get_full_positions(&wasm, contract_address.as_str()).unwrap();
    let fp = positions.get(1).unwrap().full_breakdown.position.clone().unwrap();

    let _res = wasm
        .execute(
            contract_address.as_str(),
            &ExecuteMsg::VaultExtension(crate::msg::ExtensionExecuteMsg::ModifyRange(
                crate::msg::ModifyRange::DecreaseFunds {
                    position_id: fp.position_id,
                    liquidity: (Decimal::from_str(fp.liquidity.as_str()).unwrap() / Decimal::from_ratio(2_u128, 1_u128)).into(),
                },
            )),
            &vec![],
            &admin,
        )
        .unwrap();

    let current_assets = get_total_assets(&wasm, contract_address.as_str()).unwrap();
    assert_approx_eq!(total_assets.0.amount, current_assets.0.amount, "0.000001");
    assert_approx_eq!(total_assets.1.amount, current_assets.1.amount, "0.000001");

    let _res = wasm
        .execute(
            contract_address.as_str(),
            &ExecuteMsg::VaultExtension(crate::msg::ExtensionExecuteMsg::ModifyRange(
                crate::msg::ModifyRange::CreatePosition {
                    lower_price: Decimal::from_str("0.80").unwrap(),
                    upper_price: Decimal::from_str("1.2").unwrap(),
                    ratio: Uint128::one(),
                },
            )),
            &vec![],
            &admin,
        )
        .unwrap();

    let current_assets = get_total_assets(&wasm, contract_address.as_str()).unwrap();
    assert_approx_eq!(total_assets.0.amount, current_assets.0.amount, "0.000001");
    assert_approx_eq!(total_assets.1.amount, current_assets.1.amount, "0.000001");

    let positions = get_full_positions(&wasm, contract_address.as_str()).unwrap();
    let fp = positions.get(0).unwrap().full_breakdown.position.clone().unwrap();
    let _res = wasm
    .execute(
        contract_address.as_str(),
        &ExecuteMsg::VaultExtension(crate::msg::ExtensionExecuteMsg::ModifyRange(
            crate::msg::ModifyRange::DecreaseFunds {
                position_id: fp.position_id,
                liquidity: (Decimal::from_str(fp.liquidity.as_str()).unwrap() / Decimal::from_ratio(2_u128, 1_u128)).into(),
            },
        )),
        &vec![],
        &admin,
    )
    .unwrap();

let current_assets = get_total_assets(&wasm, contract_address.as_str()).unwrap();
assert_approx_eq!(total_assets.0.amount, current_assets.0.amount, "0.000001");
assert_approx_eq!(total_assets.1.amount, current_assets.1.amount, "0.000001");

let positions = get_full_positions(&wasm, contract_address.as_str()).unwrap();
    let fp = positions.get(0).unwrap().full_breakdown.position.clone().unwrap();
    let _res = wasm
    .execute(
        contract_address.as_str(),
        &ExecuteMsg::VaultExtension(crate::msg::ExtensionExecuteMsg::ModifyRange(
            crate::msg::ModifyRange::DeletePosition { position_id: fp.position_id },
        )),
        &vec![],
        &admin,
    )
    .unwrap();
let current_assets = get_total_assets(&wasm, contract_address.as_str()).unwrap();
assert_approx_eq!(total_assets.0.amount, current_assets.0.amount, "0.000001");
assert_approx_eq!(total_assets.1.amount, current_assets.1.amount, "0.000001");

}
