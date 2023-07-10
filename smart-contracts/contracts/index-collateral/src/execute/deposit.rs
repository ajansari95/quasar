use std::ops::Index;

use cosmwasm_std::{
    coin, from_binary, to_binary, Binary, Coin, CosmosMsg, Deps, DepsMut, Env, IbcMsg, IbcTimeout,
    MessageInfo, Response, StdError, StdResult, Storage, SubMsg, SubMsgResult, Uint128, WasmMsg, QuerierWrapper, Addr, Empty, Decimal, Fraction,
};
use cw_utils::one_coin;
use multihop_router::contract::handle_get_route;
use multihop_router::route::RouteId;
use osmosis_std::types::osmosis::twap::v1beta1::TwapQuerier;
use osmosis_std::types::{
    cosmos::base::v1beta1::Coin as OsmoCoin, osmosis::tokenfactory::v1beta1::MsgMint,
};
use swaprouter::msg::SwapResponse;
// use cw2::set_contract_version;

use crate::assets::UsedAssets;
use crate::error::ContractError;
use crate::execute::swap::{batch_swap, SwapResult};
use crate::reply::replies::Replies;
use crate::state::{ASSETS, BONDING_FUNDS, IBC_CONFIG, SWAPS, SWAP_CONFIG, USED_ASSETS, COLLATERAL_DENOM, VALUE_DENOM};

pub(crate) fn execute_deposit(deps: DepsMut, info: MessageInfo) -> Result<Response, ContractError> {
    let coin = one_coin(&info)?;
    let assets = UsedAssets::from_state(deps.storage, USED_ASSETS)?;

    // the complete flow works like this:
    // swap the tokens to our desired format, after swapping, the rest of the steps have to be called in the reply
    let swap_conf = SWAP_CONFIG.load(deps.storage)?;
    let swaps = batch_swap(coin, assets)?;

    // save the outgoing swaps so we can handle them
    let swap_results: Vec<SwapResult> = swaps
        .iter()
        .map(|(swap, _)| SwapResult::new(swap.clone(), None))
        .collect();
    SWAPS.save(deps.storage, &swap_results)?;

    let msgs: Result<Vec<SubMsg>, ContractError> = swaps
        .into_iter()
        .map(|(msg, coin)| -> Result<SubMsg, ContractError> {
            Ok(SubMsg::reply_always(
                CosmosMsg::Wasm(WasmMsg::Execute {
                    contract_addr: swap_conf.router_addr.clone().to_string(),
                    msg: to_binary(&msg.into_execute(swaprouter::Slippage::Twap {
                        window_seconds: swap_conf.twap_window,
                        slippage_percentage: swap_conf.slippage_percentage,
                    }))?,
                    funds: vec![coin],
                }),
                Replies::DepositSwap as u64,
            ))
        })
        .collect();

    let swaps = msgs?;

    Ok(Response::new()
        .add_attribute("swaps", swaps.len().to_string())
        .add_submessages(swaps))
}

pub fn handle_swap_reply(
    deps: DepsMut,
    env: Env,
    msg_result: SubMsgResult,
) -> Result<Response, ContractError> {
    let bin = msg_result
        .into_result()
        .map_err(StdError::generic_err)?
        .data
        .ok_or(ContractError::NoSwapReplyData)?;

    let sr: SwapResponse = from_binary(&bin)?;

    let mut swaps = SWAPS.load(deps.storage)?;

    // update our swap status
    swaps
        .iter_mut()
        .find(|s| s.swap.output_denom == sr.token_out_denom)
        .and_then(|val| Some(val.result = Some(sr.amount)))
        .ok_or(ContractError::OutputDenomNotFound)?;

    SWAPS.save(deps.storage, &swaps)?;

    // If all swaps were completed, we can finalize the result and send all funds to the respective ica accounts
    if swaps.iter().all(|swap| swap.result.is_some()) {
        let ibc_config = IBC_CONFIG.load(deps.storage)?;
        let timeout: IbcTimeout =
            IbcTimeout::with_timestamp(env.block.time.plus_seconds(ibc_config.timeout_time));

        // TODO we want to also mint shares here and send those to the Quasar account
        // to mint shares for the user, we need to query the value of every swap converted to a single denom, probably USDC
        let mint = mint_shares(deps.storage, deps.querier, env, &swaps)?;

        let msgs: Result<Vec<IbcMsg>, ContractError> = swaps
            .iter()
            .map(|swap| {
                // TODO set a configurable timestamp somewhere for the timeout
                send_to_ica(
                    deps.storage,
                    swap.swap.output_denom.as_str(),
                    swap.result.unwrap(),
                    timeout.clone(),
                )
            })
            .collect();

        // TODO add some attributes
        Ok(Response::new().add_messages(msgs?).add_submessage(mint))
    } else {
        // save the mutated swaps with the new result
        // TODO set some attributes
        SWAPS.save(deps.storage, &swaps)?;
        Ok(Response::new())
    }
}

fn mint_shares(storage: &mut dyn Storage, querier: QuerierWrapper, env: Env, swaps: &Vec<SwapResult>) -> Result<SubMsg, ContractError> {
    let amount = calc_share_amount(storage, querier, swaps)?;

    let denom = COLLATERAL_DENOM.load(storage)?;

    let mint = MsgMint {
        sender: env.contract.address.to_string(),
        amount: Some(OsmoCoin {
            denom: denom,
            amount: amount.to_string(),
        }),
        mint_to_address: env.contract.address.to_string(),
    };

    Ok(SubMsg::reply_always(mint, Replies::MintShare.into()))
}

fn calc_share_amount(storage: &mut dyn Storage, querier: QuerierWrapper, swaps: &Vec<SwapResult>) -> Result<Uint128, ContractError> {
    let value_denom = VALUE_DENOM.load(storage)?;

    swaps.iter().fold(Uint128::zero(), |swap| {
    })
    todo!()
}

fn calc_asset_value(querier: QuerierWrapper<Empty>, env: Env, twap_window: u64, router_address: Addr, swap: &SwapResult, value_denom: String) -> Result<Uint128, ContractError> {
    let twap_querier = TwapQuerier::new(&querier);

    // get the route from the swaprouter
    let routes: swaprouter::msg::GetRouteResponse = querier.query_wasm_smart(router_address, &swaprouter::msg::QueryMsg::GetRoute { input_denom: swap.swap.output_denom, output_denom: value_denom })?;
    // calculate the asset value to usdc over the pool route
    // for each value in the pool route, we keep an "input value", query the twap to change that input value to an intermediate value
    // finally, we should endup with the amount in value denom
    let mut quote_asset = swap.swap.output_denom;
    let mut amount = swap.result.unwrap();
    let result = Uint128::zero();
    for (i, route) in routes.pool_route.into_iter().enumerate() {
        // get the 5min twap to calculate the on the route to calculate the final value
        // we use a geometric twap since it's more robust for our case, see https://delphilabs.medium.com/which-one-should-you-use-arithmetic-or-geometric-mean-twap-ded01532bf49#:~:text=The%20AM%20TWAP%20can%20be,spot%20price%20at%20block%20i.
        // 
        let base_asset = route.token_out_denom;
        let pool = route.pool_id;
        let start_time = env.block.time.minus_seconds(twap_window);

        // When swapping from input to output, we need to quote the price in the input token
        // For example when seling osmo to buy atom:
        //  price of <out> is X<in> (i.e.: price of atom is Xosmo)
        let twap: Decimal = twap_querier.geometric_twap_to_now(pool, base_asset, quote_asset, Some(osmosis_std::shim::Timestamp { seconds: start_time.seconds() as i64, nanos: 0 }))?.geometric_twap.parse()?;

        // calculate the intermediate amount
        amount = amount.checked_multiply_ratio(twap.numerator(), twap.denominator())?;

        // set the denoms for the next item
        if let Some(next) = routes.pool_route.get(i) {
            quote_asset = base_asset;
        }
    }
    // after our iteration step, 
    todo!()
}


// for a denom and an amount, get the path to the ICA address and the ICA address.
// send the tokens to the ICA addresses, this is called after the reply of execute deposit
// TODO no state tracking is done here right now, we need to track the funds as outstanding collateral returning soon
fn send_to_ica(
    storage: &mut dyn Storage,
    denom: &str,
    amount: Uint128,
    timeout: IbcTimeout,
) -> Result<IbcMsg, ContractError> {
    // lookup the destination from assets
    let asset = ASSETS.load(storage, denom)?;

    // lookup the route to the destination from the router
    let route = handle_get_route(
        storage,
        RouteId::new(asset.destination, asset.denom.clone()),
    )?
    .route;

    BONDING_FUNDS.update(storage, denom, |old| -> Result<Uint128, ContractError> {
        Ok(old.unwrap_or(Uint128::zero()).checked_add(amount)?)
    })?;

    Ok(IbcMsg::Transfer {
        channel_id: route.channel,
        to_address: asset.deposit_ica,
        amount: coin(amount.u128(), asset.denom),
        timeout,
    })
}

#[cfg(test)]
mod tests {
    use cosmwasm_std::{
        testing::{mock_dependencies, mock_env},
        SubMsgResponse, Timestamp,
    };
    use multihop_router::{
        route::{Destination, Route},
        state::ROUTES,
    };

    use super::*;
    use crate::{assets::Asset, execute::swap::Swap, state::ASSETS};
    use proptest::{collection, prelude::*};

    prop_compose! {
        // arb_swap is a strategy to return mock swaps and a a mocked result of the swap
        fn arb_swap_amount_in(input_denom: String, output_denom: String)(amount_in in 1..u128::MAX) -> Swap {
            Swap::new(coin(amount_in, input_denom.clone()), output_denom.clone())
        }
    }

    prop_compose! {
        fn arb_swap(input_denom: String)(swap in arb_swap_amount_in(input_denom, any::<String>().to_string()), amount_out in 1..u128::MAX) -> (Swap, Uint128) {
            (swap, Uint128::new(amount_out))
        }
    }

    proptest! {
        #[test]
        fn handle_swap_reply_works(swaps in collection::vec(arb_swap(any::<String>().to_string()), 1..100)){
            let mut deps = mock_dependencies();
            let env = mock_env();

            SWAPS.save(deps.as_mut().storage, &swaps.iter().map(|(v, _)| SwapResult::new(v.clone(), None)).collect()).unwrap();

            // mock stuff for send_to_ica
            IBC_CONFIG.save(deps.as_mut().storage, &crate::state::IbcConfig { timeout_time: 100 }).unwrap();

            swaps.into_iter().for_each(|val| {
                let destination = Destination::new("quasar");
                let token_out_denom = val.0.output_denom;
                let token_out_amount = val.1;

                let asset = Asset::new(token_out_denom.clone(), destination.clone(), "quasaraddress1");

                ASSETS.save(deps.as_mut().storage, token_out_denom.as_str(), &asset).unwrap();
                ROUTES
                    .save(
                        deps.as_mut().storage,
                        &RouteId::new(destination, token_out_denom.to_string()),
                        &Route::new("channel-1", "port-1", None),
                    )
                    .unwrap();

                // our submsg handling doesn't rely on events, so we can leave that empty
                let sub_msg_result = SubMsgResult::Ok(SubMsgResponse {
                    events: vec![],
                    data: Some(
                        to_binary(&SwapResponse {
                            original_sender: "me".to_string(),
                            token_out_denom: token_out_denom.clone(),
                            amount: token_out_amount.clone(),
                        })
                        .unwrap(),
                    ),
                });

                // we can verifiy the response by checking the attributes, we expect to encounter swaps.len()-1 saves and 1 with swaps.len() amount of IBC transfers
                let response = handle_swap_reply(deps.as_mut(), env.clone(), sub_msg_result).unwrap();
                // We can also load swaps from SWAPS and verifify that the out_amount of this swap is set to on the loaded swaps
                let saved_swaps = SWAPS.load(deps.as_mut().storage).unwrap();
                assert!(saved_swaps.iter().find(|s| {
                    if s.swap.output_denom == token_out_denom {
                        s.result.unwrap_or(Uint128::zero()) == token_out_amount
                    } else {
                        false
                    }
                }).is_some(), "no output found with correct token_out_amount");
            })
        }
    }

    #[test]
    fn handle_single_swap_reply_works() {
        let mut deps = mock_dependencies();
        let env = mock_env();

        let token_out_denom = "ibc/uqsr";
        let token_out_amount = Uint128::new(100);

        let swaps = vec![SwapResult::new(
            Swap::new(coin(100, "uosmo"), token_out_denom.to_string()),
            None,
        )];
        SWAPS.save(deps.as_mut().storage, &swaps).unwrap();

        // our submsg handling doesn't rely on events, so we can leave that empty
        let sub_msg_result = SubMsgResult::Ok(SubMsgResponse {
            events: vec![],
            data: Some(
                to_binary(&SwapResponse {
                    original_sender: "me".to_string(),
                    token_out_denom: token_out_denom.to_string(),
                    amount: token_out_amount,
                })
                .unwrap(),
            ),
        });

        // mock stuff for send_to_ica
        IBC_CONFIG
            .save(
                deps.as_mut().storage,
                &crate::state::IbcConfig { timeout_time: 100 },
            )
            .unwrap();

        let denom = "ibc/uqsr";
        let destination = Destination::new("quasar");
        let asset = Asset::new(denom, destination.clone(), "quasaraddress1");

        ASSETS.save(deps.as_mut().storage, denom, &asset).unwrap();
        ROUTES
            .save(
                deps.as_mut().storage,
                &RouteId::new(destination, denom.to_string()),
                &Route::new("channel-1", "port-1", None),
            )
            .unwrap();

        let response = handle_swap_reply(deps.as_mut(), env, sub_msg_result).unwrap();
        assert!(
            SWAPS
                .load(deps.as_mut().storage)
                .unwrap()
                .first()
                .unwrap()
                .result
                .is_some(),
            "only swap had no result"
        )
    }

    #[test]
    fn send_to_ica_works() {
        let mut deps = mock_dependencies();

        let denom = "ibc/uqsr";
        let destination = Destination::new("quasar");
        let asset = Asset::new(denom, destination.clone(), "quasaraddress1");

        ASSETS.save(deps.as_mut().storage, denom, &asset).unwrap();
        ROUTES
            .save(
                deps.as_mut().storage,
                &RouteId::new(destination, denom.to_string()),
                &Route::new("channel-1", "port-1", None),
            )
            .unwrap();

        let amount = Uint128::new(1000);
        let timeout = IbcTimeout::with_timestamp(Timestamp::from_seconds(100));
        let res = send_to_ica(deps.as_mut().storage, denom, amount, timeout.clone()).unwrap();

        let expected = IbcMsg::Transfer {
            channel_id: "channel-1".to_string(),
            to_address: "quasaraddress1".to_string(),
            amount: coin(amount.u128(), asset.denom),
            timeout,
        };
        assert_eq!(expected, res)
    }
}
