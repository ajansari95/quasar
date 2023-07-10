use cosmwasm_schema::cw_serde;
use cosmwasm_std::{Timestamp, Uint128};
use cw_storage_plus::{Item, Map};

use crate::{
    assets::{Asset, AssetInfo},
    execute::swap::{SwapConfig, SwapResult},
};

pub const SHARE_DENOM: Item<String> = Item::new("collateral_denom");
// TODO rename this
pub const VALUE_DENOM: Item<String> = Item::new("value_denom");

/// ASSETS is the map of assets known to the index, if there is no entry in the map for an asset
/// the contract will not be able to handle the asset. It is a mapping of denom to the Asset.
/// An entry in Assets does not mean the asset is actively used, just that the index has all the data
/// to use it
pub const ASSETS: Map<&str, Asset> = Map::new("assets");

/// USED_ASSETS is the map of assets being used for actual collateral by the contract.
/// Any normalization or other calcultations are done over the entire map.
pub const USED_ASSETS: Map<&str, AssetInfo> = Map::new("used_assets");

/// BONDING_FUNDS keeps tracks outstanding funds that are being bonded through Quicksilver.
/// Entries in the BONDING_FUNDS are not direct user deposits but are first swapped according
/// to the ratio in USED_ASSETS.
pub const BONDING_FUNDS: Map<&str, Uint128> = Map::new("bonding_funds");

pub const SWAP_CONFIG: Item<SwapConfig> = Item::new("swap_config");

/// SWAPS is the currently being executed swap, since the contract executes a set of submsgs in a single call
/// we save them here to get the output after slippage
pub const SWAPS: Item<Vec<SwapResult>> = Item::new("swaps");

pub const IBC_CONFIG: Item<IbcConfig> = Item::new("ibc_config");

#[cw_serde]
pub struct IbcConfig {
    // default timeout_time in seconds
    pub timeout_time: u64,
}
