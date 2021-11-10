use cosmwasm_std::{Deps, Env};
use valkyrie_qualifier::{QualificationMsg, QualificationResult, QualifiedContinueOption};

use crate::errors::ContractError;
use crate::states::{QualifierConfig, Querier, Requirement};

pub type QueryResult<T> = Result<T, ContractError>;

#[allow(dead_code)]
pub fn config(deps: Deps, _env: Env) -> QueryResult<QualifierConfig> {
    Ok(QualifierConfig::load(deps.storage)?)
}

pub fn requirement(deps: Deps, _env: Env) -> QueryResult<Requirement> {
    Ok(Requirement::load(deps.storage)?)
}

pub fn qualify(deps: Deps, env: Env, msg: QualificationMsg) -> QueryResult<QualificationResult> {
    let campaign = deps.api.addr_validate(msg.campaign.as_str())?;
    let sender = deps.api.addr_validate(msg.sender.as_str())?;
    let actor = deps.api.addr_validate(msg.actor.as_str())?;
    let referrer = msg
        .referrer
        .map(|r| deps.api.addr_validate(r.as_str()))
        .transpose()?;

    let requirement = Requirement::load(deps.storage)?;
    let querier = Querier::new(&deps.querier);

    let (is_valid, error_msg) = requirement.is_satisfy_requirements(
        deps.storage,
        &env.block.height,
        &querier,
        &campaign,
        &sender,
        &actor,
        referrer.as_ref(),
    )?;

    if is_valid {
        Ok(QualificationResult {
            continue_option: QualifiedContinueOption::Eligible,
            reason: None,
        })
    } else {
        let config = QualifierConfig::load(deps.storage)?;

        Ok(QualificationResult {
            continue_option: config.continue_option_on_fail,
            reason: Some(error_msg),
        })
    }
}
