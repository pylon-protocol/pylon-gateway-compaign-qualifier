use cosmwasm_std::{DepsMut, Env, MessageInfo, Response, Uint128, StdError, to_binary};
use valkyrie_qualifier::{QualificationMsg, QualifiedContinueOption, QualificationResult};
use cw20::Denom;
use crate::msgs::InstantiateMsg;
use crate::states::{Requirement, QualifierConfig, is_admin, Querier};
use crate::errors::ContractError;
use crate::queries;


pub type ExecuteResult = Result<Response, ContractError>;

pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    msg: InstantiateMsg,
) -> ExecuteResult {
    let mut response = Response::new()
        .add_attribute("action", "instantiate");

    QualifierConfig {
        admin: info.sender,
        continue_option_on_fail: msg.continue_option_on_fail,
    }.save(deps.storage)?;

    Requirement {
        //TODO:
    }.save(deps.storage)?;

    Ok(response)
}

pub fn qualify(
    deps: DepsMut,
    env: Env,
    _info: MessageInfo,
    msg: QualificationMsg,
) -> ExecuteResult {
    let mut response = Response::new()
        .add_attribute("action", "qualify");

    let result = queries::qualify(deps.as_ref(), env, msg)?;

    response = response.add_attribute("qualified_continue_option", result.continue_option.to_string())
        .set_data(to_binary(&result)?);

    Ok(response)
}
