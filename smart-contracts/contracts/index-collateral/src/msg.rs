use cosmwasm_schema::{cw_serde, QueryResponses};
use multihop_router::msg::ExecuteMsg as RouterExecuteMsg;

#[cw_serde]
pub struct InstantiateMsg {
    pub collateral_denom: String,
}

#[cw_serde]
pub enum ExecuteMsg {
    AcceptCollateral {},
    AcceptRewards {},
    // TODO finalize the deposit interface, depending on minting of share etc
    Deposit {},
    Redeem {},
    Withdraw {},
    // execute messages for the underlying router
    Router(RouterExecuteMsg),
}

#[cw_serde]
#[derive(QueryResponses)]
pub enum QueryMsg {}
