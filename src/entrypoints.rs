use cosmwasm_std::{entry_point, to_binary, Binary, Deps, DepsMut, Env, MessageInfo, Response};
use valkyrie_qualifier::query_msgs::QueryMsg;

use crate::errors::ContractError;
use crate::executions::ExecuteResult;
use crate::msgs::{ExecuteMsg, InstantiateMsg};
use crate::{executions, queries};

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
pub fn execute(deps: DepsMut, env: Env, info: MessageInfo, msg: ExecuteMsg) -> ExecuteResult {
    match msg {
        ExecuteMsg::Configure {
            admin,
            pool,
            gov,
            continue_option_on_fail,
        } => executions::configure(deps, env, info, admin, pool, gov, continue_option_on_fail),
        ExecuteMsg::Prepare {} => executions::prepare(deps, env, info),
        ExecuteMsg::Qualify(msg) => executions::qualify(deps, env, info, msg),
    }
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, env: Env, msg: QueryMsg) -> Result<Binary, ContractError> {
    let result = match msg {
        QueryMsg::Qualify(msg) => {
            to_binary(&queries::qualify_without_checking_deposit(deps, env, msg)?)
        }
        QueryMsg::Requirement {} => to_binary(&queries::requirement(deps, env)?),
    }?;

    Ok(result)
}
