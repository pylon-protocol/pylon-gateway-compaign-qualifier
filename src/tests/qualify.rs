use cosmwasm_bignumber::Uint256;
use cosmwasm_std::{from_binary, to_binary, Binary, Env, MessageInfo, Response, Uint128};
use valkyrie::campaign::query_msgs::ActorResponse;
use valkyrie_qualifier::{QualificationMsg, QualificationResult, QualifiedContinueOption};

use crate::executions::{instantiate, qualify, ExecuteResult};
use crate::msgs::{
    GovQueryMsg, GovStakerResponse, InstantiateMsg, PoolBalanceOfResponse, PoolQueryMsg,
};
use crate::tests::{
    mock_deps, qualifier_creator_sender, qualifier_env, MockDeps, CAMPAIGN, GOV, POOL, TESTER,
};

const DEPOSIT_AMOUNT: u64 = 1000000u64;

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
    )
    .unwrap();

    (env, info, response)
}

#[test]
fn succeed() {
    let mut deps = mock_deps();

    deps.querier.register_wasm_smart_query_handler(
        GOV.to_string(),
        Box::new(|x| match from_binary::<GovQueryMsg>(x).unwrap() {
            GovQueryMsg::Staker { .. } => to_binary(&GovStakerResponse {
                balance: Uint128::from(DEPOSIT_AMOUNT * 2),
                share: Default::default(),
                claimable_airdrop: vec![],
                locked_balance: vec![],
            }),
        }),
    );

    deps.querier.register_wasm_smart_query_handler(
        POOL.to_string(),
        Box::new(|x| match from_binary::<PoolQueryMsg>(x).unwrap() {
            PoolQueryMsg::BalanceOf { .. } => to_binary(&PoolBalanceOfResponse {
                amount: Uint256::from(0u64),
            }),
        }),
    );

    deps.querier.register_wasm_smart_query_handler(
        CAMPAIGN.to_string(),
        Box::new(
            |x| match from_binary::<valkyrie::campaign::query_msgs::QueryMsg>(x).unwrap() {
                valkyrie::campaign::query_msgs::QueryMsg::Actor { .. } => {
                    to_binary(&ActorResponse {
                        address: "".to_string(),
                        referrer_address: None,
                        participation_reward_amount: Default::default(),
                        referral_reward_amount: Default::default(),
                        participation_reward_amounts: vec![],
                        referral_reward_amounts: vec![],
                        cumulative_participation_reward_amount: Default::default(),
                        cumulative_referral_reward_amount: Default::default(),
                        participation_count: 0,
                        referral_count: 0,
                        last_participated_at: Default::default(),
                    })
                }
                _ => Ok(Binary::default()),
            },
        ),
    );

    let env = qualifier_env();
    let info = qualifier_creator_sender();

    instantiate(
        deps.as_mut(),
        env.clone(),
        info.clone(),
        InstantiateMsg {
            pool: POOL.to_string(),
            gov: GOV.to_string(),
            deposit_delta: Uint256::from(DEPOSIT_AMOUNT),
            min_mine_stake_amount: Uint256::from(DEPOSIT_AMOUNT * 2),
            continue_option_on_fail: QualifiedContinueOption::Eligible,
        },
    )
    .unwrap();

    super::prepare::exec(&mut deps, env.clone(), info.clone()).unwrap();

    // overwrite
    deps.querier.register_wasm_smart_query_handler(
        POOL.to_string(),
        Box::new(|x| match from_binary::<PoolQueryMsg>(x).unwrap() {
            PoolQueryMsg::BalanceOf { .. } => to_binary(&PoolBalanceOfResponse {
                amount: Uint256::from(DEPOSIT_AMOUNT),
            }),
        }),
    );

    let (_, _, response) = will_success(
        &mut deps,
        CAMPAIGN.to_string(),
        TESTER.to_string(),
        TESTER.to_string(),
        None,
    );
    assert_eq!(
        response,
        Response::default()
            .add_attribute("action", "qualify")
            .add_attribute("qualified_continue_option", "eligible")
            .set_data(
                to_binary(&QualificationResult {
                    continue_option: QualifiedContinueOption::Eligible,
                    reason: None
                })
                .unwrap()
            )
    )
}
