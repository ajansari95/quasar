use crate::{
    msg::ReplyMsg,
    state::{Proposal, CONFIG, VALIDATORS},
    ContractError, helpers::pubkey_to_address,
};
use cosmwasm_crypto::secp256k1_recover_pubkey;
use cosmwasm_std::{DepsMut, MessageInfo, Response, SubMsg, Addr};

pub fn execute_proposal(
    deps: DepsMut,
    _info: MessageInfo,
    proposal: Proposal,
) -> Result<Response, ContractError> {
    // assert_admin()

    let config = CONFIG.load(deps.storage)?;
    let validators = VALIDATORS.load(deps.storage)?;

    let mut valid: Vec<Addr> = vec![];
    let mut invalid: Vec<Addr> = vec![];

    for signature in proposal.signatures {
        let pubkey_result = secp256k1_recover_pubkey(&proposal.message_hash, &signature, 1u8); // TODO: remove hardcoded 1u8

        match pubkey_result {
            Ok(pubkey) => {
                let address = pubkey_to_address(&pubkey);

                if validators.contains(&address) {
                    valid.push(address);
                } else {
                    invalid.push(address);
                }
            }
            Err(_) => {
                // Handle error, log or push to invalid
            }
        }
    }

    // Create new response object
    let mut response = Response::new();

    if valid.len() >= config.threshold as usize {
        // TODO: MsgExecuteContract to middleware sending "proposal.payload"
        let msg = SubMsg::reply_on_success(
            create_denom_msg,
            ReplyMsg::Proposal {
                proposal,
                valid,
                invalid,
            },
        );
        response = response.add_submessage(msg);
    } else {
        // TODO: do something
    }

    Ok(response
        .add_attribute("method", "execute")
        .add_attribute("action", "proposal"))
    // .add_attribute("proposal", proposal))
}

// TODO: Reply
pub fn reply_proposal(
    deps: DepsMut,
    _info: MessageInfo,
    proposal: Proposal,
    valid: Vec<&[u8]>,
    invalid: Vec<&[u8]>,
) -> Result<Response, ContractError> {

    Ok(Response::new()
        .add_attribute("method", "reply")
        .add_attribute("action", "proposal"))
    // .add_attribute("proposal", proposal))
}

#[cfg(test)]
mod tests {
    // use crate::msg::InstantiateMsg;

    // use super::*;
    // use cosmwasm_std::testing::{mock_dependencies, mock_env, mock_info};
    // use cosmwasm_std::{coins, from_binary};

    // #[test]
    // fn reset() {
    //     let mut deps = mock_dependencies();

    //     let msg = InstantiateMsg { count: 17 };
    //     let info = mock_info("creator", &coins(2, "token"));
    //     let _res = instantiate(deps.as_mut(), mock_env(), info, msg).unwrap();

    //     // beneficiary can release it
    //     let unauth_info = mock_info("anyone", &coins(2, "token"));
    //     let msg = ExecuteMsg::Reset { count: 5 };
    //     let res = execute(deps.as_mut(), mock_env(), unauth_info, msg);
    //     match res {
    //         Err(ContractError::Unauthorized {}) => {}
    //         _ => panic!("Must return unauthorized error"),
    //     }

    //     // only the original creator can reset the counter
    //     let auth_info = mock_info("creator", &coins(2, "token"));
    //     let msg = ExecuteMsg::Reset { count: 5 };
    //     let _res = execute(deps.as_mut(), mock_env(), auth_info, msg).unwrap();

    //     // should now be 5
    //     let res = query(deps.as_ref(), mock_env(), QueryMsg::GetCount {}).unwrap();
    //     let value: GetCountResponse = from_binary(&res).unwrap();
    //     assert_eq!(5, value.count);
    // }
}
