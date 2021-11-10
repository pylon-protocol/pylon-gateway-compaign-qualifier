use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use valkyrie_qualifier::QualifiedContinueOption;

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct InstantiateMsg {
    pub continue_option_on_fail: QualifiedContinueOption,
}
