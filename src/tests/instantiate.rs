use std::error::Error;

use cosmwasm_std::{Env, MessageInfo, Response};

use crate::executions::{instantiate, ExecuteResult};
use crate::msgs::InstantiateMsg;
use crate::tests::{MockDeps, qualifier_creator_sender, qualifier_env, mock_deps};

pub fn exec(
    deps: &mut MockDeps,
    env: Env,
    info: MessageInfo,
) -> ExecuteResult {
    let msg = InstantiateMsg {
        //TODO:
    };
    instantiate(deps.as_mut(), env, info, msg)
}

pub fn default(deps: &mut MockDeps) -> (Env, MessageInfo, Response) {
    let env = qualifier_env();
    let info = qualifier_creator_sender();

    let response = exec(
        deps,
        env.clone(),
        info.clone(),
    ).unwrap();

    (env, info, response)
}

#[test]
fn succeed() {
    let mut deps = mock_deps();

    let (env, info, response) = default(&mut deps);
    assert_eq!(response, response)
    //TODO:
}
