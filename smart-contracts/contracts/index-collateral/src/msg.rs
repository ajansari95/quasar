use cosmwasm_schema::{cw_serde, QueryResponses};

#[cw_serde]
pub struct InstantiateMsg {}

#[cw_serde]
pub enum ExecuteMsg {
    AcceptCollateral {},
    AcceptRewards {},
    // TODO finalize the deposit interface, depending on minting of share etc
    Deposit {},
    Redeem {},
    Withdraw {},
}

#[cw_serde]
#[derive(QueryResponses)]
pub enum QueryMsg {}
