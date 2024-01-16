use cosmwasm_schema::cw_serde;
use cosmwasm_std::{Addr, Decimal, Decimal256, Uint128};
use cw_storage_plus::{Deque, Item, Map};

use crate::rewards::CoinList;
use crate::vault::merge::CurrentMergeWithdraw;
use crate::vault::range::move_position::SwapDirection;

/// metadata useful for display purposes
#[cw_serde]
pub struct Metadata {
    /// the underlying thesis of the vault's positions, eg aggresive
    pub thesis: String,
    /// the name of the vault
    pub name: String,
}

pub const METADATA: Item<Metadata> = Item::new("metadata");

pub const ADMIN_ADDRESS: Item<Addr> = Item::new("admin_address");
pub const RANGE_ADMIN: Item<Addr> = Item::new("range_admin");

/// VAULT_CONFIG: Base config struct for the contract.
#[cw_serde]
pub struct VaultConfig {
    /// Percentage of profit to be charged as performance fee
    pub performance_fee: Decimal,
    /// Account to receive fee payments
    pub treasury: Addr,
    /// swap max slippage
    pub swap_max_slippage: Decimal,
}

pub const VAULT_CONFIG: Item<VaultConfig> = Item::new("vault_config");

pub const VAULT_DENOM: Item<String> = Item::new("vault_denom");

/// POOL_CONFIG
#[cw_serde]
pub struct PoolConfig {
    pub pool_id: u64,
    pub token0: String,
    // todo: Verify in instantiate message
    pub token1: String, // todo: Verify in instantiate message
}

impl PoolConfig {
    pub fn pool_contains_token(&self, token: impl Into<String>) -> bool {
        vec![&self.token0, &self.token1].contains(&&token.into())
    }
}

pub const POOL_CONFIG: Item<PoolConfig> = Item::new("pool_config");

/// a position managed by the contract
#[cw_serde]
pub struct Position {
    pub position_id: u64,
    pub ratio: Uint128,
}

// positions in the contract, the key should be the same as the position's id in Osmosis
pub const POSITIONS: Map<u64, Position> = Map::new("positions");
pub const SHARES: Map<Addr, Uint128> = Map::new("shares");

/// The merge of positions currently being executed
pub const CURRENT_MERGE: Deque<CurrentMergeWithdraw> = Deque::new("current_merge");

#[cw_serde]
pub struct CurrentMergePosition {
    pub lower_tick: i64,
    pub upper_tick: i64,
}

pub const CURRENT_MERGE_POSITION: Item<CurrentMergePosition> = Item::new("current_merge_position");

#[cw_serde]
pub struct CurrentDeposit {
    pub token0_in: Uint128,
    pub token1_in: Uint128,
    pub original_id: u64,
    pub refund0: Option<Uint128>,
    pub refund1: Option<Uint128>,
    pub liquidity_out: Option<Decimal256>,
}

pub const CURRENT_DEPOSIT: Deque<CurrentDeposit> = Deque::new("current_deposit");
pub const CURRENT_DEPOSITOR: Item<Addr> = Item::new("current_depositor");
pub const CURRENT_DEPOSIT_LEFTOVER: Item<(Uint128, Uint128)> =
    Item::new("current_deposit_leftover");

/// REWARDS: Current rewards are the rewards being gathered, these can be both spread rewards as well as incentives
pub const CURRENT_REWARDS: Item<CoinList> = Item::new("current_rewards");
pub const USER_REWARDS: Map<Addr, CoinList> = Map::new("user_rewards");
pub const STRATEGIST_REWARDS: Item<CoinList> = Item::new("strategist_rewards");

/// CURRENT_REMAINDERS is a tuple of Uin128 containing the current remainder amount before performing a swap
pub const CURRENT_REMAINDERS: Item<(Uint128, Uint128)> = Item::new("current_remainders");
pub const CURRENT_BALANCE: Item<(Uint128, Uint128)> = Item::new("current_balance");
pub const CURRENT_SWAP: Item<(SwapDirection, Uint128)> = Item::new("current_swap");

pub const INSTANTIATE_CREATE_POSITION_FUNDS: Item<(Uint128, Uint128)> =
    Item::new("instantiate_create_position_funds");

pub const CURRENT_RATIO: Item<Uint128> = Item::new("current_ratio");

pub const CURRENT_POSITION_ID: Item<u64> = Item::new("current_position_id");

#[cw_serde]
pub struct ModifyRangeState {
    // pre-withdraw state items
    pub lower_tick: i64,
    pub upper_tick: i64,
    // the max slippage for modifying the range
    pub max_slippage: Decimal,
    // pre-deposit state items
    pub new_range_position_ids: Vec<u64>,
    // the ratio of the position we are moving, and thus also of the final position
    pub position_ratio: Uint128,
}

pub const MODIFY_RANGE_STATE: Item<Option<ModifyRangeState>> = Item::new("modify_range_state");

#[cw_serde]
pub struct SwapDepositMergeState {
    pub target_lower_tick: i64,
    pub target_upper_tick: i64,
    pub target_range_position_ids: Vec<u64>,
}

pub const SWAP_DEPOSIT_MERGE_STATE: Item<SwapDepositMergeState> =
    Item::new("swap_deposit_merge_state");

#[cw_serde]
pub struct TickExpIndexData {
    pub initial_price: Decimal256,
    pub max_price: Decimal256,
    pub additive_increment_per_tick: Decimal256,
    pub initial_tick: i64,
}

pub const TICK_EXP_CACHE: Map<i64, TickExpIndexData> = Map::new("tick_exp_cache");
pub const CURRENT_WITHDRAWER: Item<Addr> = Item::new("current_withdrawer");
pub const CURRENT_WITHDRAWER_DUST: Item<(Uint128, Uint128)> = Item::new("current_withdrawer_dust");

#[cfg(test)]
mod tests {
    use super::PoolConfig;

    #[test]
    fn test_pool_contains_token() {
        let pool_config = PoolConfig {
            pool_id: 1,
            token0: "token1".to_string(),
            token1: "token2".to_string(),
        };

        assert!(pool_config.pool_contains_token("token1"));
        assert!(pool_config.pool_contains_token("token2"));
        assert!(!pool_config.pool_contains_token("token3"));
    }
}
