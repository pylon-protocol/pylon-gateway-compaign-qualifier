use cosmwasm_std::{Uint128, Uint256};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use std::fmt;
use valkyrie_qualifier::{QualificationMsg, QualifiedContinueOption};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct InstantiateMsg {
    pub pool: String,
    pub gov: String,
    pub deposit_delta: Uint256,
    pub min_mine_stake_amount: Uint256,
    pub continue_option_on_fail: QualifiedContinueOption,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum ExecuteMsg {
    Configure {
        admin: Option<String>,
        pool: Option<String>,
        continue_option_on_fail: Option<QualifiedContinueOption>,
    },

    // 1. Prepare
    Prepare {},
    // 2. Deposit
    // 3. Qualify
    Qualify(QualificationMsg),
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum PoolQueryMsg {
    BalanceOf { owner: String },
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum GovQueryMsg {
    Staker { address: String },
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, JsonSchema)]
pub struct GovStakerResponse {
    pub balance: Uint128,
    pub share: Uint128,
    pub claimable_airdrop: Vec<(String, Uint128)>,
    pub locked_balance: Vec<(u64, VoterInfo)>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct VoterInfo {
    pub vote: VoteOption,
    pub balance: Uint128,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum VoteOption {
    Yes,
    No,
}

impl fmt::Display for VoteOption {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if *self == VoteOption::Yes {
            write!(f, "yes")
        } else {
            write!(f, "no")
        }
    }
}
