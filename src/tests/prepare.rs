use cosmwasm_bignumber::Uint256;
use cosmwasm_std::{from_binary, to_binary, Env, MessageInfo, Response};

use crate::executions::{prepare, ExecuteResult};
use crate::msgs::{PoolBalanceOfResponse, PoolQueryMsg};
use crate::states::load_prepare_status;
use crate::tests::{mock_deps, MockDeps, POOL};

const DEPOSIT_AMOUNT: u64 = 1000000u64;

pub fn exec(deps: &mut MockDeps, env: Env, info: MessageInfo) -> ExecuteResult {
    prepare(deps.as_mut(), env, info)
}

#[test]
fn succeed() {
    let mut deps = mock_deps();

    deps.querier.register_wasm_smart_query_handler(
        POOL.to_string(),
        Box::new(|x| match from_binary::<PoolQueryMsg>(x).unwrap() {
            PoolQueryMsg::BalanceOf { .. } => to_binary(&PoolBalanceOfResponse {
                amount: Uint256::from(DEPOSIT_AMOUNT),
            }),
        }),
    );

    let (env, info, _) = super::instantiate::default(&mut deps);

    let response = exec(&mut deps, env.clone(), info.clone()).unwrap();
    assert_eq!(
        response,
        Response::default().add_attribute("action", "prepare")
    );

    let prepare_status =
        load_prepare_status(deps.as_ref().storage, &env.block.height, &info.sender).unwrap();
    assert_eq!(prepare_status, Uint256::from(DEPOSIT_AMOUNT))
}
