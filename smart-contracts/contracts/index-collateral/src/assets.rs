use cosmwasm_schema::cw_serde;
use cosmwasm_std::{Decimal, Order, StdError, Storage, Uint128};
use cw_storage_plus::Map;
use multihop_router::route::Destination;

use crate::ContractError;

/// UsedAssets is the struct representation of USED_ASSETS, a new assets should either be createed through new
/// in combination with add_asset(), or preferably, with_assets(). It can then be saved to state with to_state().
/// UsedAssets can be directly created from USED_ASSETS with from_state
pub struct UsedAssets {
    assets: Vec<AssetInfo>,
}

impl UsedAssets {
    pub fn new() -> UsedAssets {
        UsedAssets::default()
    }

    pub fn from_state(
        storage: &mut dyn Storage,
        map: Map<&str, AssetInfo>,
    ) -> Result<UsedAssets, ContractError> {
        let items: Result<Vec<(String, AssetInfo)>, StdError> =
            map.range(storage, None, None, Order::Ascending).collect();
        Ok(UsedAssets {
            assets: items?.into_iter().map(|(_, v)| v).collect(),
        })
    }

    pub fn to_state(
        &self,
        storage: &mut dyn Storage,
        map: Map<&str, AssetInfo>,
    ) -> Result<(), ContractError> {
        for val in self.assets.iter() {
            map.save(storage, val.asset.denom.as_str(), val)?;
        }
        todo!()
    }

    /// Instantiate assets from a vec of assets
    pub fn with_assets(assets: Vec<AssetInfo>) -> UsedAssets {
        let mut assets = UsedAssets { assets };
        assets.normalize();
        assets
    }

    pub fn add_asset(&mut self, asset: AssetInfo) -> &Self {
        self.assets.push(asset);
        // after we add an asset, we need to normalize our ratio for the new asset again
        self.normalize();
        self
    }

    /// normalize normalizes the assets of each ratio according to raw_ratio
    fn normalize(&mut self) {
        let total = self
            .assets
            .iter()
            .fold(Uint128::zero(), |acc, b| acc + b.raw_ratio);
        for asset in self.assets.iter_mut() {
            asset.ratio = Decimal::from_ratio(asset.raw_ratio, total);
        }
    }
}

impl Default for UsedAssets {
    fn default() -> Self {
        Self { assets: vec![] }
    }
}

#[cw_serde]
pub struct AssetInfo {
    asset: Asset,
    ratio: Decimal,
    raw_ratio: Uint128,
}

impl AssetInfo {
    pub fn new(
        denom: impl Into<String>,
        destination: impl Into<String>,
        deposit_ica: impl Into<String>,
        raw_ratio: Uint128,
    ) -> AssetInfo {
        AssetInfo {
            asset: Asset::new(denom, destination.into(), deposit_ica),
            // we always set ratio to 0, since we only want ration to be calculated in comparison to other ratios
            ratio: Decimal::zero(),
            raw_ratio,
        }
    }
}

#[cw_serde]
pub struct Asset {
    pub denom: String,
    pub destination: Destination,
    pub deposit_ica: String,
}

impl Asset {
    pub fn new(
        denom: impl Into<String>,
        destination: impl Into<Destination>,
        deposit_ica: impl Into<String>,
    ) -> Asset {
        Asset {
            denom: denom.into(),
            destination: destination.into(),
            deposit_ica: deposit_ica.into(),
        }
    }
}
