use cosmwasm_std::testing::{mock_env, mock_info, MockApi, MockStorage};
use cosmwasm_std::{Addr, Env, MessageInfo, OwnedDeps};

use crate::tests::mock_querier::{mock_dependencies, CustomMockWasmQuerier};

pub mod instantiate;
pub mod mock_querier;
pub mod prepare;
pub mod qualify;

type MockDeps = OwnedDeps<MockStorage, MockApi, CustomMockWasmQuerier>;

fn mock_deps() -> MockDeps {
    mock_dependencies(&[])
}

const QUALIFIER: &str = "Qualifier";
const QUALIFIER_CREATOR: &str = "QualifierCreator";

const TESTER: &str = "terra199vw7724lzkwz6lf2hsx04lrxfkz09tg8dlp6r";

const GOV: &str = "terra1x46rqay4d3cssq8gxxvqz8xt6nwlz4td20k38v";
const POOL: &str = "terra17lmam6zguazs5q5u6z5mmx76uj63gldnse2pdp";
const CAMPAIGN: &str = "terra1757tkx08n0cqrw7p86ny9lnxsqeth0wgp0em95";

fn qualifier_env() -> Env {
    let mut env = mock_env();

    env.contract.address = Addr::unchecked(QUALIFIER);

    env
}

fn qualifier_creator_sender() -> MessageInfo {
    mock_info(QUALIFIER_CREATOR, &[])
}
