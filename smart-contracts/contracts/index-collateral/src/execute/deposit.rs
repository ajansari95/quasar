use cosmwasm_std::{
    coin, from_binary, to_binary, Binary, Coin, CosmosMsg, Deps, DepsMut, Env, IbcMsg, IbcTimeout,
    MessageInfo, Response, StdError, StdResult, Storage, SubMsg, SubMsgResult, Uint128, WasmMsg,
};
use cw_utils::one_coin;
use multihop_router::contract::handle_get_route;
use multihop_router::route::RouteId;
use swaprouter::msg::SwapResponse;
// use cw2::set_contract_version;

use crate::assets::UsedAssets;
use crate::error::ContractError;
use crate::execute::swap::{batch_swap, SwapResult};
use crate::reply::replies::Replies;
use crate::state::{ASSETS, BONDING_FUNDS, IBC_CONFIG, SWAPS, SWAP_CONFIG, USED_ASSETS};

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

    // If all swaps were completed, we can finalize the result and send all funds to the respective ica accounts
    if swaps.iter().all(|swap| swap.result.is_some()) {
        let ibc_config = IBC_CONFIG.load(deps.storage)?;
        let timeout: IbcTimeout =
            IbcTimeout::with_timestamp(env.block.time.plus_seconds(ibc_config.timeout_time));

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

        Ok(Response::new().add_messages(msgs?))
    } else {
        // save the mutated swaps with the new result
        SWAPS.save(deps.storage, &swaps)?;
        Ok(Response::new())
    }
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
    use cosmwasm_std::{testing::{mock_dependencies, mock_env}, SubMsgResponse, Timestamp};
    use multihop_router::{
        route::{Destination, Route},
        state::ROUTES,
    };

    use super::*;
    use crate::{assets::Asset, state::ASSETS, execute::swap::Swap};

    #[test]
    fn handle_swap_reply_works() {
        let mut deps = mock_dependencies();
        let env = mock_env();

        let token_out_denom = "ibc/uqsr";
        let token_out_amount = Uint128::new(100);

        let swaps = vec![SwapResult::new(Swap::new(coin(100, "uosmo"), token_out_denom.to_string()), None)];
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
        IBC_CONFIG.save(deps.as_mut().storage, &crate::state::IbcConfig { timeout_time: 100 }).unwrap();

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
