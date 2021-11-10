use cosmwasm_std::{Binary, Deps, DepsMut, entry_point, Env, MessageInfo, Response, to_binary};
use valkyrie_qualifier::execute_msgs::ExecuteMsg;
use valkyrie_qualifier::query_msgs::QueryMsg;

use crate::{executions, queries};
use crate::errors::ContractError;
use crate::executions::ExecuteResult;
use crate::msgs::InstantiateMsg;

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: InstantiateMsg,
) -> ExecuteResult {
    executions::instantiate(deps, env, info, msg)?;

    Ok(Response::default())
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> ExecuteResult {
    match msg {
        ExecuteMsg::Qualify(msg) => executions::qualify(deps, env, info, msg),
    }
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(
    deps: Deps,
    env: Env,
    msg: QueryMsg,
) -> Result<Binary, ContractError> {
    let result = match msg {
        QueryMsg::Qualify(msg) => to_binary(&queries::qualify(deps, env, msg)?),
        QueryMsg::Requirement {} => to_binary(&queries::requirement(deps, env)?),
    }?;

    Ok(result)
}
