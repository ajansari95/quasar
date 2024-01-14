use cosmwasm_std::{Decimal, Decimal256, Deps, DepsMut, Env, MessageInfo, Response};

use crate::{
    math::tick::tick_to_price,
    query::AutomationResponse,
    state::{AutomationConfig, AUTOMATION_CONFIG, POOL_CONFIG, POSITION},
    vault::range::execute_update_range,
    ContractError,
};
use osmosis_std::types::osmosis::{
    concentratedliquidity::v1beta1::ConcentratedliquidityQuerier,
    poolmanager::v1beta1::PoolmanagerQuerier,
};

pub fn execute_automation(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    max_slippage: Decimal,
    ratio_of_swappable_funds_to_use: Decimal,
    twap_window_seconds: u64,
) -> Result<Response, ContractError> {
    let (lower_price, upper_price) = get_new_prices(deps.as_ref())?;

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

fn get_new_prices(deps: Deps) -> Result<(Decimal, Decimal), ContractError> {
    let automation_config = AUTOMATION_CONFIG.load(deps.storage)?;
    if !automation_config.enabled {
        return Err(ContractError::AutomationDisabled {});
    }

    // Prepare queriers
    let pmq = PoolmanagerQuerier::new(&deps.querier);
    let clq = ConcentratedliquidityQuerier::new(&deps.querier);

    // Get pool config to query pool information for current_tick
    let pool_config = POOL_CONFIG.load(deps.storage)?;
    let pool = pmq.pool(pool_config.pool_id)?.pool.unwrap().value;

    // Get current position
    let position = POSITION.load(deps.storage)?;
    let position_by_id = clq
        .position_by_id(position.position_id)?
        .position
        .unwrap()
        .position
        .unwrap();

    // Convert position's current ticks to prices
    let current_price = tick_to_price(pool.current_tick)?;
    let current_lower_price = tick_to_price(position_by_id.lower_tick)?;
    let current_upper_price = tick_to_price(position_by_id.upper_tick)?;

    // Calculate the distance ratio from lower and upper bounds
    let relative_position =
        calculate_position_ratio(current_price, current_lower_price, current_upper_price)?;

    // Determine the amount of idle funds
    let idle_funds = calculate_idle_funds(deps)?;

    // Decision logic for adjusting range
    if should_adjust_range(relative_position, idle_funds, &automation_config) {
        // Calculate new lower and upper bounds
        let (new_lower_tick, new_upper_tick) =
            calculate_new_ticks(current_price, &automation_config)?;
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
///
/// # Errors
///
/// Returns `ContractError::InvalidInput` if the range width (difference between upper and lower prices) is zero, to prevent division by zero.
fn calculate_position_ratio(
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

fn calculate_idle_funds(deps: Deps) -> Result<Decimal, ContractError> {
    // Placeholder logic: Calculate the amount of funds that are idle and not currently being used in the position.
    // This might involve querying the contract's state or the pool's state.
    todo!()
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
fn should_adjust_range(
    relative_position: Decimal256,
    idle_funds: Decimal,
    automation_config: &AutomationConfig,
) -> bool {
    // Check if the relative position is outside the configured bounds
    let is_outside_bounds = relative_position <= automation_config.lower_bound_threshold
        || relative_position >= automation_config.upper_bound_threshold;

    // Check if idle funds are above the configured threshold
    let are_idle_funds_above_threshold = idle_funds >= automation_config.idle_funds_threshold;

    // Decide if the range should be adjusted
    is_outside_bounds || are_idle_funds_above_threshold
}

fn calculate_new_ticks(
    current_price: Decimal256,
    automation_config: &AutomationConfig,
) -> Result<(i64, i64), ContractError> {
    // Placeholder logic: Calculate the new lower and upper tick positions based on the current price and the ticks setting in the automation configuration.
    // You will need to determine how to adjust the ticks based on the current market conditions and the desired position size.
    todo!()
}

pub fn query_automation(deps: Deps) -> Result<AutomationResponse, ContractError> {
    // TODO: Move query to query.rs
    todo!()
}
