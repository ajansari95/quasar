use crate::helpers::get_unused_balances;
use crate::math::tick::{tick_to_price, verify_tick_exp_cache};
use crate::rewards::CoinList;
use crate::state::AUTOMATION_CONFIG;
use crate::vault::automation::{
    calculate_idle_funds, calculate_position_ratio, should_adjust_range,
};
use crate::vault::concentrated_liquidity::get_position;
use crate::ContractError;
use crate::{
    error::ContractResult,
    state::{
        PoolConfig, ADMIN_ADDRESS, METADATA, POOL_CONFIG, POSITION, SHARES, USER_REWARDS,
        VAULT_DENOM,
    },
};
use cosmwasm_schema::cw_serde;
use cosmwasm_std::{coin, Coin, Decimal, Deps, Env, Uint128};
use cw_vault_multi_standard::VaultInfoResponse;
use osmosis_std::types::cosmos::bank::v1beta1::BankQuerier;
use osmosis_std::types::osmosis::concentratedliquidity::v1beta1::{
    ConcentratedliquidityQuerier, Pool,
};
use osmosis_std::types::osmosis::poolmanager::v1beta1::PoolmanagerQuerier;

#[cw_serde]
pub struct MetadataResponse {
    // thesis -> actual metadata
    pub thesis: String,
    // name -> actual metadata
    pub name: String,
    // total_supply -> from denom
    pub total_supply: Uint128,
    // symbol -> tokenfactory denom
    pub symbol: String,
    // decimals -> hardcoded since native denom
    pub decimals: u8,
    // owner -> admin
    pub admin: String,
}

#[cw_serde]
pub struct PoolResponse {
    pub pool_config: PoolConfig,
}

#[cw_serde]
pub struct PositionResponse {
    pub position_ids: Vec<u64>,
}

#[cw_serde]
pub struct AssetsBalanceResponse {
    pub balances: Vec<Coin>,
}

#[cw_serde]
pub struct UserSharesBalanceResponse {
    pub balance: Uint128,
}

#[cw_serde]
pub struct UserRewardsResponse {
    pub rewards: Vec<Coin>,
}

#[cw_serde]
pub struct TotalAssetsResponse {
    pub token0: Coin,
    pub token1: Coin,
}

#[cw_serde]
pub struct RangeAdminResponse {
    pub address: String,
}

#[cw_serde]
pub struct TotalVaultTokenSupplyResponse {
    pub total: Uint128,
}

#[cw_serde]
pub struct VerifyTickCacheResponse {
    pub result: Result<(), i64>,
}

#[cw_serde]
pub struct AutomationResponse {
    pub relative_position: Decimal,
    pub token0_idle_ratio: Decimal,
    pub token1_idle_ratio: Decimal,
    pub should_adjust_range: bool,
}

pub fn query_verify_tick_cache(deps: Deps) -> Result<VerifyTickCacheResponse, ContractError> {
    verify_tick_exp_cache(deps.storage)
        .err()
        .map(|e| {
            if let ContractError::TickNotFound { tick } = e {
                Ok(VerifyTickCacheResponse { result: Err(tick) })
            } else {
                Err(e)
            }
        })
        .unwrap_or(Ok(VerifyTickCacheResponse { result: Ok(()) }))
}

pub fn query_metadata(deps: Deps) -> ContractResult<MetadataResponse> {
    let metadata = METADATA.load(deps.storage)?;
    let vault_denom = VAULT_DENOM.load(deps.storage)?;
    let total_supply = BankQuerier::new(&deps.querier)
        .supply_of(vault_denom.clone())?
        .amount
        .unwrap()
        .amount
        .parse::<u128>()?
        .into();
    let admin = ADMIN_ADDRESS.load(deps.storage)?.to_string();

    Ok(MetadataResponse {
        thesis: metadata.thesis,
        name: metadata.name,
        total_supply,
        symbol: vault_denom,
        decimals: 6,
        admin,
    })
}

pub fn query_info(deps: Deps) -> ContractResult<VaultInfoResponse> {
    let pool_config = POOL_CONFIG.load(deps.storage)?;
    let vault_token = VAULT_DENOM.load(deps.storage)?;
    Ok(VaultInfoResponse {
        tokens: vec![pool_config.token0, pool_config.token1],
        vault_token,
    })
}

pub fn query_pool(deps: Deps) -> ContractResult<PoolResponse> {
    let pool_config = POOL_CONFIG.load(deps.storage)?;
    Ok(PoolResponse { pool_config })
}

pub fn query_position(deps: Deps) -> ContractResult<PositionResponse> {
    let position_id = POSITION.load(deps.storage)?.position_id;
    Ok(PositionResponse {
        position_ids: vec![position_id],
    })
}

pub fn query_assets_from_shares(
    deps: Deps,
    env: Env,
    shares: Uint128,
) -> ContractResult<AssetsBalanceResponse> {
    let vault_supply = query_total_vault_token_supply(deps)?.total;
    let vault_assets = query_total_assets(deps, env)?;

    let vault_balance = CoinList::from_coins(vec![vault_assets.token0, vault_assets.token1]);

    let assets_from_shares = vault_balance.mul_ratio(Decimal::from_ratio(shares, vault_supply));

    Ok(AssetsBalanceResponse {
        balances: assets_from_shares.coins(),
    })
}

/// User assets is the users assets EXCLUDING any rewards claimable by that user
pub fn query_user_assets(
    deps: Deps,
    env: Env,
    user: String,
) -> ContractResult<AssetsBalanceResponse> {
    let user_shares = query_user_balance(deps, user)?.balance;
    let user_assets = query_assets_from_shares(deps, env, user_shares)?;

    Ok(user_assets)
}

pub fn query_user_balance(deps: Deps, user: String) -> ContractResult<UserSharesBalanceResponse> {
    let balance = SHARES
        .may_load(deps.storage, deps.api.addr_validate(&user)?)?
        .unwrap_or(Uint128::zero());
    Ok(UserSharesBalanceResponse { balance })
}

pub fn query_user_rewards(deps: Deps, user: String) -> ContractResult<UserRewardsResponse> {
    let rewards = USER_REWARDS
        .load(deps.storage, deps.api.addr_validate(&user)?)?
        .coins();
    Ok(UserRewardsResponse { rewards })
}

/// Vault base assets is the vault assets EXCLUDING any rewards claimable by strategist or users
pub fn query_total_assets(deps: Deps, env: Env) -> ContractResult<TotalAssetsResponse> {
    let position = get_position(deps.storage, &deps.querier)?;
    let pool = POOL_CONFIG.load(deps.storage)?;
    let unused_balance = get_unused_balances(deps.storage, &deps.querier, &env)?;

    // add token0 unused balance to what's in the position
    let mut token0 = position
        .asset0
        .map(|c| c.try_into().unwrap())
        .unwrap_or(coin(0, pool.token0));

    token0 = Coin {
        denom: token0.denom.clone(),
        amount: token0
            .amount
            .checked_add(unused_balance.find_coin(token0.denom).amount)?,
    };

    let mut token1 = position
        .asset1
        .map(|c| c.try_into().unwrap())
        .unwrap_or(coin(0, pool.token1));

    token1 = Coin {
        denom: token1.denom.clone(),
        amount: token1
            .amount
            .checked_add(unused_balance.find_coin(token1.denom).amount)?,
    };

    Ok(TotalAssetsResponse { token0, token1 })
}

pub fn query_total_vault_token_supply(deps: Deps) -> ContractResult<TotalVaultTokenSupplyResponse> {
    let bq = BankQuerier::new(&deps.querier);
    let vault_denom = VAULT_DENOM.load(deps.storage)?;
    let total = bq
        .supply_of(vault_denom)?
        .amount
        .unwrap()
        .amount
        .parse::<u128>()?
        .into();
    Ok(TotalVaultTokenSupplyResponse { total })
}

pub fn query_automation(deps: Deps, env: Env) -> ContractResult<AutomationResponse> {
    let automation_config = AUTOMATION_CONFIG.load(deps.storage)?;

    // Prepare queriers
    let pmq = PoolmanagerQuerier::new(&deps.querier);
    let clq = ConcentratedliquidityQuerier::new(&deps.querier);

    // Get pool config to query pool information for current_tick
    let pool_config = POOL_CONFIG.load(deps.storage)?;
    let pool: Pool = pmq
        .pool(pool_config.pool_id)?
        .pool
        .ok_or(ContractError::PoolNotFound {
            pool_id: pool_config.pool_id,
        })?
        .try_into()
        .unwrap();

    // Get position information
    let position = POSITION.load(deps.storage)?;
    let full_position_breakdown = clq.position_by_id(position.position_id)?.position.unwrap();

    // Convert position's current ticks to prices
    let current_price = tick_to_price(pool.current_tick)?;
    let current_lower_price = tick_to_price(
        full_position_breakdown
            .position
            .as_ref()
            .unwrap()
            .lower_tick,
    )?;
    let current_upper_price = tick_to_price(
        full_position_breakdown
            .position
            .as_ref()
            .unwrap()
            .upper_tick,
    )?;

    // Calculate the distance ratio from lower and upper bounds
    let relative_position =
        calculate_position_ratio(current_price, current_lower_price, current_upper_price)?;

    // TODO:
    let total_assets = query_total_assets(deps, env)?;
    let idle_funds = calculate_idle_funds(total_assets, full_position_breakdown)?;
    let should_adjust_range =
        should_adjust_range(relative_position, idle_funds, &automation_config)?;

    Ok(AutomationResponse {
        relative_position: relative_position.try_into().unwrap(),
        token0_idle_ratio: idle_funds.0,
        token1_idle_ratio: idle_funds.1,
        should_adjust_range,
    })
}
