use cosmwasm_std::Uint256;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use valkyrie_qualifier::{QualificationMsg, QualifiedContinueOption};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct InstantiateMsg {
    pub pool: String,
    pub deposit_delta: Uint256,
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
