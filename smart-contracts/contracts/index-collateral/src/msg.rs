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
    // execute messages for the underlying router
    Router(),
}

#[cw_serde]
#[derive(QueryResponses)]
pub enum QueryMsg {}
