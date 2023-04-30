#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{Binary, Deps, DepsMut, Env, MessageInfo, Response, StdResult};
use cw2::set_contract_version;

use crate::error::ContractError;
use crate::msg::{ExecuteMsg, InstantiateMsg, QueryMsg};
use crate::state::{Config, Poll, CONFIG, POLLS};

const CONTRACT_NAME: &str = "crates.io:zero-to-hero";
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;
    let validated_admin_address: cosmwasm_std::Addr = deps.api.addr_validate(&msg.admin_address)?;

    let config = Config {
        admin_address: validated_admin_address,
    };
    CONFIG.save(deps.storage, &config)?;
    Ok(Response::new().add_attribute("action", "instantiated"))
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    match msg {
        ExecuteMsg::CreatePoll { question } => execute_create_poll(deps, env, info, question),
        ExecuteMsg::Vote { question, choice } => execute_vote(deps, env, info, question, choice),
    }
}

fn execute_create_poll(
    deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    question: String,
) -> Result<Response, ContractError> {
    // Does the map have a key of this value
    if POLLS.has(deps.storage, question.clone()) {
        // If it does we want to error
        return Err(ContractError::CustomError {
            val: "Key already taken!".to_string(),
        });
    }
    let poll = Poll {
        question: question.clone(),
        yes_votes: 0,
        no_votes: 0,
    };
    POLLS.save(deps.storage, question, &poll)?;
    Ok(Response::new().add_attribute("action", "create_poll"))
}

fn execute_vote(
    deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    question: String,
    choice: String,
) -> Result<Response, ContractError> {
    // If there is no poll with the key question
    if !POLLS.has(deps.storage, question.clone()) {
        // We want to error and tell the user that the poll does not exits
        return Err(ContractError::CustomError {
            val: "Poll does not exits".to_string(),
        });
    }
    let mut poll = POLLS.load(deps.storage, question.clone())?;
    if choice != "yes" && choice != "no" {
        return Err(ContractError::CustomError {
            val: "Unrecognized choice!".to_string(),
        });
    } else {
        if choice == "yes" {
            poll.yes_votes += 1;
        } else {
            poll.no_votes += 1;
        }
        POLLS.save(deps.storage, question, &poll)?;
        Ok(Response::new().add_attribute("action", "vote"))
    }
}
#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(_deps: Deps, _env: Env, _msg: QueryMsg) -> StdResult<Binary> {
    unimplemented!()
}

#[cfg(test)]
mod tests {
    use cosmwasm_std::{
        attr,
        testing::{mock_dependencies, mock_env, mock_info},
    };

    use crate::msg::{ExecuteMsg, InstantiateMsg};

    use super::{execute, instantiate};

    #[test]
    fn test_instantiate() {
        let mut deps = mock_dependencies();
        let env = mock_env();
        let info = mock_info("addr1", &[]);
        let msg = InstantiateMsg {
            admin_address: "addr1".to_string(),
        };

        let resp = instantiate(deps.as_mut(), env, info, msg).unwrap();
        assert_eq!(resp.attributes, vec![attr("action", "instantiated")]);
    }

    #[test]
    fn test_create_poll() {
        let mut deps = mock_dependencies();
        let env = mock_env();
        let info = mock_info("addr1", &[]);
        let msg = InstantiateMsg {
            admin_address: "addr1".to_string(),
        };

        let _resp = instantiate(deps.as_mut(), env.clone(), info.clone(), msg).unwrap();
        let msg = ExecuteMsg::CreatePoll {
            question: "Do you love Spark IBC".to_string(),
        };
        let resp = execute(deps.as_mut(), env.clone(), info.clone(), msg).unwrap();
        assert_eq!(resp.attributes, vec![attr("action", "create_poll")]);

        let msg = ExecuteMsg::CreatePoll {
            question: "Do you love Spark IBC".to_string(),
        };
        let _resp = execute(deps.as_mut(), env, info, msg).unwrap_err();
    }

    #[test]
    fn test_vote() {
        let mut deps = mock_dependencies();
        let env = mock_env();
        let info = mock_info("addr1", &[]);
        let msg = InstantiateMsg {
            admin_address: "addr1".to_string(),
        };
        let _resp = instantiate(deps.as_mut(), env.clone(), info.clone(), msg).unwrap();

        let msg = ExecuteMsg::CreatePoll {
            question: "Do you love Spark IBC".to_string(),
        };
        let _resp = execute(deps.as_mut(), env.clone(), info.clone(), msg).unwrap();

        //success case, we vote on the poll that exists, with a valid option
        let msg = ExecuteMsg::Vote {
            question: "Do you love Spark IBC".to_string(),
            choice: "yes".to_string(),
        };
        let resp = execute(deps.as_mut(), env.clone(), info.clone(), msg).unwrap();
        assert_eq!(resp.attributes, vec![attr("action", "vote")]);

        // Error Case 1: we vote on a poll that does exists
        let msg = ExecuteMsg::Vote {
            question: "Do you hate Spark IBC".to_string(),
            choice: "yes".to_string(),
        };
        let _resp = execute(deps.as_mut(), env.clone(), info.clone(), msg).unwrap_err();

        // Error Case 2: we vote on a poll that exists but with invalid choices
        let msg = ExecuteMsg::Vote {
            question: "Do you hate Spark IBC".to_string(),
            choice: "yeah baby".to_string(),
        };
        let _resp = execute(deps.as_mut(), env.clone(), info.clone(), msg).unwrap_err();
    }
}
