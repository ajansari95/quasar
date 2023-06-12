use cosmwasm_schema::cw_serde;
use cosmwasm_std::{Decimal, Order, StdError, Storage, Uint128};
use cw_storage_plus::Map;
use multihop_router::route::Destination;

use crate::{ContractError, assets::{Asset, AssetInfo}};

/// ASSETS is the map of assets known to the index, if there is no entry in the map for an asset
/// the contract will not be able to handle the asset. It is a mapping of denom to the Asset.
/// An entry in Assets does not mean
pub const ASSETS: Map<String, Asset> = Map::new("assets");

/// USED_ASSETS is the map of assets being used for actual collateral by the contract.
/// Any normalization or other calcultations are done over the entire map.
pub const USED_ASSETS: Map<&str, AssetInfo> = Map::new("used_assets");

/// BONDING_FUNDS keeps tracks outstanding funds that are being bonded through Quicksilver.
/// Entries in the BONDING_FUNDS are not direct user deposits but are first swapped according
/// to the ratio in USED_ASSETS.
pub const BONDING_FUNDS: Map<&str, Uint128> = Map::new("bonding_funds");
