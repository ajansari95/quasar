use cosmwasm_schema::{cw_serde, QueryResponses};
use cosmwasm_std::Addr;

use crate::state::{Config, Proposal};

#[cw_serde]
pub struct InstantiateMsg {
    pub admin: Addr,
    pub config: Config,
    pub validators: Vec<Addr>,
}

#[cw_serde]
pub enum ExecuteMsg {
    UpdateAdmin { address: Addr },
    UpdateConfig { config: Config },
    AddValidators { addresses: Vec<Addr> },
    RemoveValidators { addresses: Vec<Addr> },
    Proposal { proposal: Proposal },
}

#[cw_serde]
pub enum ReplyMsg<'a> {
    Proposal {
        proposal: Proposal,
        valid: Vec<&'a [u8]>,
        invalid: Vec<&'a [u8]>,
    },
}

#[cw_serde]
#[derive(QueryResponses)]
pub enum QueryMsg {
    #[returns(GetAdminResponse)]
    GetAdmin {},
    #[returns(GetConfigResponse)]
    GetConfig {},
    #[returns(GetValidatorsResponse)]
    GetValidators {},
    // #[returns(GetProposalResponse)]
    // GetProposal {},
    // #[returns(GetProposalsResponse)]
    // GetProposals {},
}

// We define a custom struct for each query response
#[cw_serde]
pub struct GetAdminResponse {
    pub admin: Addr,
}

#[cw_serde]
pub struct GetConfigResponse {
    pub config: Config,
}

#[cw_serde]
pub struct GetValidatorsResponse {
    pub validators: Vec<Addr>,
}
