use cosmwasm_std::{
    coin, CosmosMsg, Decimal, DepsMut, Env, MessageInfo, Response, StdError, SubMsg, SubMsgResult,
    Uint128,
};
use osmosis_std::types::osmosis::concentratedliquidity::v1beta1::{
    MsgCreatePositionResponse, Pool,
};
use osmosis_std::types::osmosis::poolmanager::v1beta1::PoolmanagerQuerier;
use osmosis_std::types::osmosis::tokenfactory::v1beta1::{
    MsgCreateDenom, MsgCreateDenomResponse, MsgMint,
};

use crate::helpers::{get_asset0_value, get_spot_price, must_pay_one_or_two};
use crate::msg::InstantiateMsg;
use crate::reply::Replies;
use crate::rewards::CoinList;
use crate::state::{
    Metadata, PoolConfig, Position, ADMIN_ADDRESS, INSTANTIATE_CREATE_POSITION_FUNDS, METADATA,
    POOL_CONFIG, POSITIONS, RANGE_ADMIN, STRATEGIST_REWARDS, VAULT_CONFIG, VAULT_DENOM,
};
use crate::vault::concentrated_liquidity::create_position;
use crate::ContractError;

pub fn handle_instantiate(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    // a performance fee of more than 1 means that the performance fee is more than 100%
    if msg.config.performance_fee > Decimal::one() {
        return Err(ContractError::Std(StdError::generic_err(
            "performance fee cannot be more than 1.0",
        )));
    }

    VAULT_CONFIG.save(deps.storage, &msg.config.into_vault_config(deps.api)?)?;

    let pool: Pool = PoolmanagerQuerier::new(&deps.querier)
        .pool(msg.pool_id)?
        .pool
        .ok_or(ContractError::PoolNotFound {
            pool_id: msg.pool_id,
        })?
        .try_into()
        .unwrap();

    POOL_CONFIG.save(
        deps.storage,
        &PoolConfig {
            pool_id: pool.id,
            token0: pool.token0.clone(),
            token1: pool.token1.clone(),
        },
    )?;

    STRATEGIST_REWARDS.save(deps.storage, &CoinList::new())?;

    METADATA.save(
        deps.storage,
        &Metadata {
            thesis: msg.thesis,
            name: msg.name,
        },
    )?;

    let admin = deps.api.addr_validate(&msg.admin)?;

    ADMIN_ADDRESS.save(deps.storage, &admin)?;
    RANGE_ADMIN.save(deps.storage, &deps.api.addr_validate(&msg.range_admin)?)?;

    let create_denom_msg: CosmosMsg = MsgCreateDenom {
        sender: env.contract.address.to_string(),
        subdenom: msg.vault_token_subdenom,
    }
    .into();

    // in order to create the initial position, we need some funds to throw in there, these funds should be seen as burned
    let (initial0, initial1) = must_pay_one_or_two(&info, (pool.token0, pool.token1))?;

    INSTANTIATE_CREATE_POSITION_FUNDS.save(deps.storage, &(initial0.amount, initial1.amount))?;

    let create_position_msg = create_position(
        deps,
        &env,
        msg.initial_lower_tick,
        msg.initial_upper_tick,
        vec![initial0, initial1],
        Uint128::zero(),
        Uint128::zero(),
    )?;

    Ok(Response::new()
        .add_submessage(SubMsg::reply_on_success(
            create_denom_msg,
            Replies::CreateDenom as u64,
        ))
        .add_submessage(SubMsg::reply_on_success(
            create_position_msg,
            Replies::InstantiateCreatePosition as u64,
        )))
}

pub fn handle_create_denom_reply(
    deps: DepsMut,
    data: SubMsgResult,
) -> Result<Response, ContractError> {
    let response: MsgCreateDenomResponse = data.try_into()?;
    VAULT_DENOM.save(deps.storage, &response.new_token_denom)?;

    Ok(Response::new().add_attribute("vault_denom", response.new_token_denom))
}

pub fn handle_instantiate_create_position_reply(
    deps: DepsMut,
    env: Env,
    data: SubMsgResult,
) -> Result<Response, ContractError> {
    let response: MsgCreatePositionResponse = data.try_into()?;
    POSITIONS.save(
        deps.storage,
        response.position_id,
        &Position {
            position_id: response.position_id,
            ratio: Uint128::one(),
        },
    )?;
    let vault_denom = VAULT_DENOM.load(deps.storage)?;

    // todo the mint amount here should be calculated from the asset_0 value
    // todo do we want to mint the initial mint to the instantiater, or just not care?
    let (initial0, initial1) = INSTANTIATE_CREATE_POSITION_FUNDS.load(deps.storage)?;
    let spot_price = get_spot_price(deps.storage, &deps.querier)?;
    let asset0_value = get_asset0_value(initial0, initial1, spot_price)?;

    let mint_msg = MsgMint {
        sender: env.contract.address.to_string(),
        amount: Some(coin(asset0_value.u128(), vault_denom).into()),
        mint_to_address: env.contract.address.to_string(),
    };

    Ok(Response::new()
        .add_message(mint_msg)
        .add_attribute("initial_position", response.position_id.to_string())
        .add_attribute("initial_liquidity", response.liquidity_created))
}
