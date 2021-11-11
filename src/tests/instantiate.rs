use cosmwasm_std::{Addr, Api, Env, MessageInfo, Response};
use valkyrie_qualifier::QualifiedContinueOption;

use crate::executions::{instantiate, ExecuteResult};
use crate::msgs::InstantiateMsg;
use crate::states::QualifierConfig;
use crate::tests::{
    mock_deps, qualifier_creator_sender, qualifier_env, MockDeps, GOV, POOL, QUALIFIER_CREATOR,
};

pub fn exec(deps: &mut MockDeps, env: Env, info: MessageInfo) -> ExecuteResult {
    let msg = InstantiateMsg {
        pool: POOL.to_string(),
        gov: GOV.to_string(),
        deposit_delta: Default::default(),
        min_mine_stake_amount: Default::default(),
        continue_option_on_fail: QualifiedContinueOption::Eligible,
    };
    instantiate(deps.as_mut(), env, info, msg)
}

pub fn default(deps: &mut MockDeps) -> (Env, MessageInfo, Response) {
    let env = qualifier_env();
    let info = qualifier_creator_sender();

    let response = exec(deps, env.clone(), info.clone()).unwrap();

    (env, info, response)
}

#[test]
fn succeed() {
    let mut deps = mock_deps();

    let (_, _, response) = default(&mut deps);
    assert_eq!(
        response,
        Response::default().add_attribute("action", "instantiate")
    );

    let config = QualifierConfig::load(deps.as_ref().storage).unwrap();
    assert_eq!(
        config,
        QualifierConfig {
            admin: Addr::unchecked(QUALIFIER_CREATOR),
            pool: deps.api.addr_validate(POOL).unwrap(),
            gov: deps.api.addr_validate(GOV).unwrap(),
            continue_option_on_fail: QualifiedContinueOption::Eligible
        }
    )
}
