use cosmwasm_bignumber::Uint256;
use cosmwasm_std::{Addr, QuerierWrapper, StdResult, Storage, Uint128};
use cw20::{BalanceResponse, Cw20QueryMsg, Denom};
use cw_storage_plus::{Item, Map};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use std::borrow::Borrow;

use crate::msgs::{GovQueryMsg, GovStakerResponse, PoolBalanceOfResponse, PoolQueryMsg};
use valkyrie::campaign::query_msgs::ActorResponse;
use valkyrie_qualifier::QualifiedContinueOption;

const QUALIFIER_CONFIG: Item<QualifierConfig> = Item::new("qualifier_config");

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct QualifierConfig {
    pub admin: Addr,
    pub pool: Addr,
    pub gov: Addr,
    pub continue_option_on_fail: QualifiedContinueOption,
}

impl QualifierConfig {
    pub fn save(&self, storage: &mut dyn Storage) -> StdResult<()> {
        QUALIFIER_CONFIG.save(storage, self)
    }

    pub fn load(storage: &dyn Storage) -> StdResult<QualifierConfig> {
        QUALIFIER_CONFIG.load(storage)
    }

    pub fn is_admin(&self, address: &Addr) -> bool {
        self.admin == *address
    }
}

#[allow(dead_code)]
pub fn is_admin(storage: &dyn Storage, address: &Addr) -> StdResult<bool> {
    QualifierConfig::load(storage).map(|c| c.is_admin(address))
}

const USER_PREPARE_STATUS: Map<(&[u8], &str), Uint256> = Map::new("prepare_status");

pub fn save_prepare_status(
    storage: &mut dyn Storage,
    block_number: &u64,
    address: &Addr,
    amount: &Uint256,
) -> StdResult<()> {
    USER_PREPARE_STATUS.borrow().save(
        storage,
        (&block_number.to_be_bytes(), address.as_str()),
        amount,
    )
}

pub fn load_prepare_status(
    storage: &dyn Storage,
    block_number: &u64,
    address: &Addr,
) -> StdResult<Uint256> {
    Ok(USER_PREPARE_STATUS
        .borrow()
        .may_load(storage, (&block_number.to_be_bytes(), address.as_str()))?
        .unwrap_or_default())
}

const REQUIREMENT: Item<Requirement> = Item::new("requirement");

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct Requirement {
    pub deposit_delta: Uint256,
    pub min_mine_stake_amount: Uint256,
}

impl Requirement {
    pub fn save(&self, storage: &mut dyn Storage) -> StdResult<()> {
        REQUIREMENT.save(storage, self)
    }

    pub fn load(storage: &dyn Storage) -> StdResult<Requirement> {
        REQUIREMENT.load(storage)
    }

    #[allow(clippy::too_many_arguments)]
    pub fn is_satisfy_requirements(
        &self,
        storage: &dyn Storage,
        block_number: &u64,
        querier: &Querier,
        campaign: &Addr,
        sender: &Addr,
        actor: &Addr,
        _referrer: Option<&Addr>,
    ) -> StdResult<(bool, String)> {
        let result = self.is_satisfy_deposit_delta(storage, querier, block_number, sender)?;
        if !result.0 {
            return Ok(result);
        }

        let result = self.is_satisfy_mine_stake_amount(storage, querier, sender)?;
        if !result.0 {
            return Ok(result);
        }

        let result = self.is_satisfy_participation_count(querier, campaign, actor)?;
        if !result.0 {
            return Ok(result);
        }

        Ok((true, String::default()))
    }

    fn is_satisfy_deposit_delta(
        &self,
        storage: &dyn Storage,
        querier: &Querier,
        block_number: &u64,
        sender: &Addr,
    ) -> StdResult<(bool, String)> {
        // it's because before amount is zero
        let config = QualifierConfig::load(storage)?;
        let prepare_status = load_prepare_status(storage, block_number, sender)?;
        let pool_deposit_after = querier.load_pool_deposit(&config.pool, sender)?;
        if pool_deposit_after - prepare_status < self.deposit_delta {
            return Ok((
                false,
                format!(
                    "Delta does not satisfy condition(required: {}, delta: {})",
                    self.deposit_delta.to_string(),
                    pool_deposit_after.to_string(),
                ),
            ));
        }

        Ok((true, String::default()))
    }

    fn is_satisfy_mine_stake_amount(
        &self,
        storage: &dyn Storage,
        querier: &Querier,
        sender: &Addr,
    ) -> StdResult<(bool, String)> {
        let config = QualifierConfig::load(storage)?;
        let stake_amount = querier.load_gov_stake_amount(&config.gov, sender)?;
        if Uint256::from(stake_amount) < self.min_mine_stake_amount {
            return Ok((
                false,
                format!(
                    "Minimum MINE stake amount does not satisfy condition(required: {}, amount: {})",
                    stake_amount.to_string(),
                    self.min_mine_stake_amount.to_string(),
                ),
            ));
        }

        Ok((true, String::default()))
    }

    fn is_satisfy_participation_count(
        &self,
        querier: &Querier,
        campaign: &Addr,
        actor: &Addr,
    ) -> StdResult<(bool, String)> {
        let participation_count = querier.load_participation_count(campaign, actor)?;
        if participation_count != 0 {
            return Ok((false, "Already participated".to_string()));
        }

        Ok((true, String::default()))
    }
}

pub struct Querier<'a> {
    querier: &'a QuerierWrapper<'a>,
}

impl Querier<'_> {
    pub fn new<'a>(querier: &'a QuerierWrapper<'a>) -> Querier<'a> {
        Querier { querier }
    }

    #[allow(dead_code)]
    pub fn load_balance(&self, denom: &Denom, address: &Addr) -> StdResult<Uint128> {
        match denom {
            Denom::Native(denom) => self.load_native_balance(denom, address),
            Denom::Cw20(token_contract) => self.load_cw20_balance(token_contract, address),
        }
    }

    fn load_native_balance(&self, denom: &str, address: &Addr) -> StdResult<Uint128> {
        Ok(self.querier.query_balance(address, denom)?.amount)
    }

    fn load_cw20_balance(&self, token_contract: &Addr, address: &Addr) -> StdResult<Uint128> {
        let balance: BalanceResponse = self.querier.query_wasm_smart(
            token_contract,
            &Cw20QueryMsg::Balance {
                address: address.to_string(),
            },
        )?;

        Ok(balance.balance)
    }

    pub fn load_pool_deposit(&self, pool: &Addr, staker: &Addr) -> StdResult<Uint256> {
        let balance: PoolBalanceOfResponse = self.querier.query_wasm_smart(
            pool,
            &PoolQueryMsg::BalanceOf {
                owner: staker.to_string(),
            },
        )?;

        Ok(balance.amount)
    }

    pub fn load_gov_stake_amount(&self, gov: &Addr, staker: &Addr) -> StdResult<Uint128> {
        let staker: GovStakerResponse = self.querier.query_wasm_smart(
            gov,
            &GovQueryMsg::Staker {
                address: staker.to_string(),
            },
        )?;

        Ok(staker.balance)
    }

    pub fn load_participation_count(&self, campaign: &Addr, address: &Addr) -> StdResult<u64> {
        let actor: ActorResponse = self.querier.query_wasm_smart(
            campaign,
            &valkyrie::campaign::query_msgs::QueryMsg::Actor {
                address: address.to_string(),
            },
        )?;

        Ok(actor.participation_count)
    }
}

#[allow(dead_code)]
fn denom_to_string(denom: &Denom) -> String {
    match denom {
        Denom::Native(denom) => denom.to_string(),
        Denom::Cw20(address) => address.to_string(),
    }
}
