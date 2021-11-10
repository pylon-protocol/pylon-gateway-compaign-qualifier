use cosmwasm_std::{OwnedDeps, Env, Addr, MessageInfo};
use cosmwasm_std::testing::{MockStorage, MockApi, MockQuerier, mock_env, mock_info, mock_dependencies};

pub mod instantiate;
pub mod qualify;

type MockDeps = OwnedDeps<MockStorage, MockApi, MockQuerier>;

fn mock_deps() -> MockDeps {
    mock_dependencies(&[])
}

const QUALIFIER: &str = "Qualifier";
const QUALIFIER_CREATOR: &str = "QualifierCreator";

fn qualifier_env() -> Env {
    let mut env = mock_env();

    env.contract.address = Addr::unchecked(QUALIFIER);

    env
}

fn qualifier_creator_sender() -> MessageInfo {
    mock_info(QUALIFIER_CREATOR, &[])
}
