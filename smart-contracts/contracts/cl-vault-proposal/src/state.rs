use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use cosmwasm_std::Addr;
use cw_storage_plus::Item;

// Admin
pub const ADMIN: Item<Addr> = Item::new("admin");

// Config
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema)]
pub struct Config {
    pub threshold: i64,
}
pub const CONFIG: Item<Config> = Item::new("config");

// Validators
pub const VALIDATORS: Item<Vec<Addr>> = Item::new("validators");

// Proposals
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema)]
pub struct Proposal<'a> {
    pub payload: ProposalPayload,
    pub message_hash: &'a [u8],
    pub signatures: Vec<&'a [u8]>,
}
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema)]
pub struct ProposalPayload {
    pub lower_price: String,
    pub upper_price: String,
    pub ratio_of_swappable_funds_to_use: String,
}
pub const PROPOSALS: Item<Vec<Proposal>> = Item::new("state");
