use cosmwasm_std::{to_binary, DepsMut, Env, MessageInfo, Response};
use valkyrie_qualifier::{QualificationMsg, QualifiedContinueOption};

use crate::errors::ContractError;
use crate::msgs::InstantiateMsg;
use crate::queries;
use crate::states::{save_prepare_status, QualifierConfig, Querier, Requirement};

pub type ExecuteResult = Result<Response, ContractError>;

pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    msg: InstantiateMsg,
) -> ExecuteResult {
    let response = Response::new().add_attribute("action", "instantiate");

    QualifierConfig {
        admin: info.sender,
        pool: deps.api.addr_validate(msg.pool.as_str())?,
        gov: deps.api.addr_validate(msg.gov.as_str())?,
        continue_option_on_fail: msg.continue_option_on_fail,
    }
    .save(deps.storage)?;

    Requirement {
        deposit_delta: msg.deposit_delta,
        min_mine_stake_amount: msg.min_mine_stake_amount,
    }
    .save(deps.storage)?;

    Ok(response)
}

pub fn configure(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    admin: Option<String>,
    pool: Option<String>,
    continue_option_on_fail: Option<QualifiedContinueOption>,
) -> ExecuteResult {
    let response = Response::new().add_attribute("action", "configure");

    let mut config = QualifierConfig::load(deps.storage)?;
    if !config.is_admin(&info.sender) {
        return Err(ContractError::Unauthorized {});
    }

    if let Some(v) = admin {
        config.admin = deps.api.addr_validate(v.as_str())?;
    }
    if let Some(v) = pool {
        config.pool = deps.api.addr_validate(v.as_str())?;
    }
    if let Some(v) = continue_option_on_fail {
        config.continue_option_on_fail = v;
    }

    config.save(deps.storage)?;

    Ok(response)
}

pub fn prepare(deps: DepsMut, env: Env, info: MessageInfo) -> ExecuteResult {
    let response = Response::new().add_attribute("action", "prepare");

    let config = QualifierConfig::load(deps.storage)?;
    let querier = Querier::new(&deps.querier);

    let pool_deposit = querier.load_pool_deposit(&config.pool, &info.sender)?;
    if !pool_deposit.is_zero() {
        return Err(ContractError::Unauthorized {});
    }

    save_prepare_status(deps.storage, &env.block.height, &info.sender)?;

    Ok(response)
}

pub fn qualify(
    deps: DepsMut,
    env: Env,
    _info: MessageInfo,
    msg: QualificationMsg,
) -> ExecuteResult {
    let mut response = Response::new().add_attribute("action", "qualify");

    let result = queries::qualify(deps.as_ref(), env, msg)?;

    response = response
        .add_attribute(
            "qualified_continue_option",
            result.continue_option.to_string(),
        )
        .set_data(to_binary(&result)?);

    Ok(response)
}
