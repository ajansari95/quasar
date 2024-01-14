use crate::{
    math::tick::tick_to_price,
    query::{query_total_assets, TotalAssetsResponse},
    state::{AutomationConfig, AUTOMATION_CONFIG, POOL_CONFIG, POSITION},
    vault::range::execute_update_range,
    ContractError,
};
use cosmwasm_std::{Coin, Decimal, Decimal256, Deps, DepsMut, Env, MessageInfo, Response, Uint128};
use osmosis_std::types::osmosis::concentratedliquidity::v1beta1::Pool;
use osmosis_std::types::osmosis::{
    concentratedliquidity::v1beta1::{ConcentratedliquidityQuerier, FullPositionBreakdown},
    poolmanager::v1beta1::PoolmanagerQuerier,
};
use std::str::FromStr;

pub fn execute_automation(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    max_slippage: Decimal,
    ratio_of_swappable_funds_to_use: Decimal,
    twap_window_seconds: u64,
) -> Result<Response, ContractError> {
    let (lower_price, upper_price) = get_new_prices(deps.as_ref(), env.clone())?;

    let result = execute_update_range(
        deps,
        env,
        info,
        lower_price,
        upper_price,
        max_slippage,
        ratio_of_swappable_funds_to_use,
        twap_window_seconds,
    )?;

    Ok(Response::new().add_attributes(result.attributes))
}

fn get_new_prices(deps: Deps, env: Env) -> Result<(Decimal, Decimal), ContractError> {
    let automation_config = AUTOMATION_CONFIG.load(deps.storage)?;
    if !automation_config.enabled {
        return Err(ContractError::AutomationDisabled {});
    }

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

    // Get current position
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

    // Determine the amount of idle funds
    let total_assets: TotalAssetsResponse = query_total_assets(deps, env)?;
    let idle_funds: (Decimal, Decimal) =
        calculate_idle_funds(total_assets, full_position_breakdown)?;

    // Decision logic for adjusting range
    if should_adjust_range(relative_position, idle_funds, &automation_config)? {
        // Calculate new lower and upper bounds
        let (new_lower_tick, new_upper_tick) =
            calculate_new_ticks(pool.current_tick, &automation_config)?;
        let new_lower_price = tick_to_price(new_lower_tick)?;
        let new_upper_price = tick_to_price(new_upper_tick)?;

        Ok((
            new_lower_price.try_into().unwrap(),
            new_upper_price.try_into().unwrap(),
        ))
    } else {
        Err(ContractError::AutomationThreshold {})
    }
}

/// Calculates the relative position of the current price within the range defined by the lower and upper prices.
///
/// This function computes the relative position as a ratio within the range, where 0.5 represents the midpoint of the range.
/// A value greater than 0.5 indicates a position closer to the upper bound, and less than 0.5 indicates a position closer to the lower bound.
///
/// # Arguments
///
/// * `current_price` - The current price of the asset, represented as `Decimal256`.
/// * `lower_price` - The lower price bound of the position, represented as `Decimal256`.
/// * `upper_price` - The upper price bound of the position, represented as `Decimal256`.
///
/// # Returns
///
/// A `Result` which, on success, contains the relative position ratio within the range. On failure, it contains a `ContractError`.
pub fn calculate_position_ratio(
    current_price: Decimal256,
    lower_price: Decimal256,
    upper_price: Decimal256,
) -> Result<Decimal256, ContractError> {
    let range_width = upper_price - lower_price;
    if range_width == Decimal256::zero() {
        return Err(ContractError::InvalidRangeWidth {});
    }

    let relative_position = (current_price - lower_price) / range_width;

    Ok(relative_position)
}

/// Calculates the idle fund ratios for each asset in a position.
///
/// This function computes the percentage of funds that are idle (not actively used in a position) for each asset in a pair.
/// It compares the total amounts of each asset (active + idle) with the amounts currently active in the position to determine the idle funds.
/// The idle fund ratio is calculated as a percentage for each asset.
///
/// # Arguments
///
/// * `total_assets` - A `TotalAssetsResponse` struct containing the total (active + idle) amounts of each asset in the pair.
/// * `full_position_breakdown` - A `FullPositionBreakdown` struct containing the amounts of each asset currently active in the position.
///
/// # Returns
///
/// A `Result` which, on success, contains a tuple of two `Decimal` values representing the idle fund ratios for each asset in the pair.
/// Each ratio is expressed as a percentage. On failure, it contains a `ContractError`.
pub fn calculate_idle_funds(
    total_assets: TotalAssetsResponse,
    full_position_breakdown: FullPositionBreakdown,
) -> Result<(Decimal, Decimal), ContractError> {
    // Assuming CoinList is a Vec<Coin>
    let total_assets_coin_list = vec![total_assets.token0, total_assets.token1];
    let position_active_funds = vec![
        Coin {
            denom: full_position_breakdown.asset0.clone().unwrap().denom,
            amount: Uint128::from_str(full_position_breakdown.asset0.unwrap().amount.as_str())?,
        },
        Coin {
            denom: full_position_breakdown.asset1.clone().unwrap().denom,
            amount: Uint128::from_str(full_position_breakdown.asset1.unwrap().amount.as_str())?,
        },
    ];

    // Calculate the idle funds for each asset
    let mut idle_funds = total_assets_coin_list.clone();
    for idle_fund in &mut idle_funds {
        if let Some(active_fund) = position_active_funds
            .iter()
            .find(|&a| a.denom == idle_fund.denom)
        {
            idle_fund.amount = idle_fund.amount.checked_sub(active_fund.amount)?;
        } else {
            return Err(ContractError::IncorrectAmountFunds {}); // TODO: more specific one
        }
    }

    // Calculate the idle fund ratio for each asset
    let idle_ratios = idle_funds
        .iter()
        .map(|idle_fund| {
            if let Some(total_asset) = total_assets_coin_list
                .iter()
                .find(|&a| a.denom == idle_fund.denom)
            {
                if total_asset.amount.is_zero() {
                    Decimal::zero()
                } else {
                    Decimal::from_ratio(idle_fund.amount, total_asset.amount)
                        * Decimal::percent(100)
                }
            } else {
                Decimal::zero()
            }
        })
        .collect::<Vec<Decimal>>();

    if idle_ratios.len() == 2 {
        Ok((idle_ratios[0], idle_ratios[1]))
    } else {
        Err(ContractError::CalculationError {})
    }
}

/// Determines if the range should be adjusted based on the relative position, idle funds, and automation configuration.
///
/// # Arguments
///
/// * `relative_position` - The relative position of the current price within the range, represented as `Decimal256`.
/// * `idle_funds` - The amount of idle funds, represented as `Decimal`.
/// * `automation_config` - A reference to the automation configuration settings.
///
/// # Returns
///
/// Returns `true` if the range should be adjusted, `false` otherwise.
pub fn should_adjust_range(
    relative_position: Decimal256,
    idle_funds: (Decimal, Decimal),
    automation_config: &AutomationConfig,
) -> Result<bool, ContractError> {
    // Check if the relative position is outside the configured bounds
    let is_outside_bounds = relative_position <= automation_config.lower_bound_threshold
        || relative_position >= automation_config.upper_bound_threshold;

    // Check if idle funds are above the configured threshold
    let are_idle_funds_above_threshold_0 = idle_funds.0 >= automation_config.idle_funds_threshold;
    let are_idle_funds_above_threshold_1 = idle_funds.1 >= automation_config.idle_funds_threshold;

    // Decide if the range should be adjusted
    Ok(is_outside_bounds || (are_idle_funds_above_threshold_0 || are_idle_funds_above_threshold_1))
}

fn calculate_new_ticks(
    current_tick: i64,
    automation_config: &AutomationConfig,
) -> Result<(i64, i64), ContractError> {
    // Placeholder logic: Calculate the new lower and upper tick positions based on the current price and the ticks setting in the automation configuration.
    // You will need to determine how to adjust the ticks based on the current market conditions and the desired position size.
    todo!()
}

pub fn validate_automation_config(
    tick_spacing: u64,
    automation_config: &AutomationConfig,
) -> Result<bool, ContractError> {
    // Validate lower and upper bound thresholds
    if automation_config.lower_bound_threshold < Decimal256::zero()
        || automation_config.lower_bound_threshold > Decimal256::one()
        || automation_config.upper_bound_threshold < Decimal256::zero()
        || automation_config.upper_bound_threshold > Decimal256::one()
        || automation_config.upper_bound_threshold < automation_config.lower_bound_threshold
    {
        return Err(ContractError::IncorrectAutomationConfig {});
    }

    // Validate balance
    if automation_config.balance.is_zero() {
        return Err(ContractError::IncorrectAutomationConfig {});
    }

    // Validate ticks as a multiple of tick_spacing
    if automation_config.ticks == 0 || automation_config.ticks % tick_spacing != 0 {
        return Err(ContractError::IncorrectAutomationConfig {});
    }

    Ok(true)
}
