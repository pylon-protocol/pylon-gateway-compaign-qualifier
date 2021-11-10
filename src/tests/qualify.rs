use std::error::Error;

use cosmwasm_std::{Env, MessageInfo, Response};

use crate::msgs::InstantiateMsg;
use crate::tests::{MockDeps, qualifier_creator_sender, qualifier_env, mock_deps};
use crate::executions::{qualify, ExecuteResult};
use valkyrie_qualifier::QualificationMsg;

pub fn exec(
    deps: &mut MockDeps,
    env: Env,
    info: MessageInfo,
    campaign: String,
    sender: String,
    actor: String,
    referrer: Option<String>,
) -> ExecuteResult {
    let msg = QualificationMsg {
        campaign,
        sender,
        actor,
        referrer,
    };
    qualify(deps.as_mut(), env, info, msg)
}

pub fn will_success(
    deps: &mut MockDeps,
    campaign: String,
    sender: String,
    actor: String,
    referrer: Option<String>,
) -> (Env, MessageInfo, Response) {
    let env = qualifier_env();
    let info = qualifier_creator_sender();

    let response = exec(
        deps,
        env.clone(),
        info.clone(),
        campaign,
        sender,
        actor,
        referrer,
    ).unwrap();

    (env, info, response)
}

#[test]
fn succeed() {
    let mut deps = mock_deps();

    super::instantiate::default(&mut deps);
    //TODO:
}
