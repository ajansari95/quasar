use cosmwasm_std::{Addr, Deps, StdResult};

use crate::{
    msg::{GetAdminResponse, GetConfigResponse, GetValidatorsResponse},
    state::{Config, ADMIN, CONFIG, VALIDATORS},
};

pub fn query_admin(deps: Deps) -> StdResult<GetAdminResponse> {
    let admin: Addr = ADMIN.load(deps.storage)?;
    Ok(GetAdminResponse { admin })
}

pub fn query_config(deps: Deps) -> StdResult<GetConfigResponse> {
    let config: Config = CONFIG.load(deps.storage)?;
    Ok(GetConfigResponse { config })
}

pub fn query_validators(deps: Deps) -> StdResult<GetValidatorsResponse> {
    let validators: Vec<Addr> = VALIDATORS.load(deps.storage)?;
    Ok(GetValidatorsResponse { validators })
}

// TODO: QueryProposal
// TODO: QueryProposals
