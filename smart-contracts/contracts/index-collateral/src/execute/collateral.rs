use cosmwasm_std::{Coin, Response, Storage, Uint128};

use crate::{state::BONDING_FUNDS, ContractError};

pub(crate) fn execute_accept_collateral(
    storage: &mut dyn Storage,
    coin: Coin,
) -> Result<Response, ContractError> {
    // decrement the amount from the amount of assets from bonding funds
    BONDING_FUNDS.update(
        storage,
        coin.denom.as_str(),
        |old| -> Result<Uint128, ContractError> {
            Ok(old.unwrap_or(Uint128::zero()).checked_sub(coin.amount)?)
        },
    )?;

    Ok(Response::new()
        .add_attribute("action", "accept_collateral")
        .add_attribute("denom", coin.denom)
        .add_attribute("amount", coin.amount.to_string()))
}
