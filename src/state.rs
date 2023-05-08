use cosmwasm_schema::cw_serde;

use cosmwasm_std::{Addr, Coin, Env, Order, StdResult, Storage, Timestamp};
use cw_storage_plus::Map;

use cw20::{Balance, Cw20CoinVerified};

#[cw_serde]
#[derive(Default)]
pub struct GenericBalance {
    pub native: Vec<Coin>,
    pub cw20: Vec<Cw20CoinVerified>,
}

impl GenericBalance {
    pub fn add_tokens(&mut self, add: Balance) {
        match add {
            Balance::Native(balance) => {
                for token in balance.0 {
                    let index = self.native.iter().enumerate().find_map(|(i, exist)| {
                        if exist.denom == token.denom {
                            Some(i)
                        } else {
                            None
                        }
                    });
                    match index {
                        Some(idx) => self.native[idx].amount += token.amount,
                        None => self.native.push(token),
                    }
                }
            }
            Balance::Cw20(token) => {
                let index = self.cw20.iter().enumerate().find_map(|(i, exist)| {
                    if exist.address == token.address {
                        Some(i)
                    } else {
                        None
                    }
                });
                match index {
                    Some(idx) => self.cw20[idx].amount += token.amount,
                    None => self.cw20.push(token),
                }
            }
        };
    }
}

#[cw_serde]
pub struct Insurance {
    /// arbiter can decide to approve or refund the insurance
    pub arbiter: Addr,
    /// if approved, funds go to the recipient, cannot approve if recipient is none
    pub recipient: Option<Addr>,
    /// if refunded, funds go to the source
    pub source: Addr,
    /// Title of the insurance, for example for a bug bounty "Fix issue in contract.rs"
    pub title: String,
    /// Description of the insurance, a more in depth description of how to meet the insurance condition
    pub description: String,
    /// When end height set and block height exceeds this value, the insurance is expired.
    /// Once an insurance is expired, it can be returned to the original funder (via "refund").
    pub end_height: Option<u64>,
    /// When end time (in seconds since epoch 00:00:00 UTC on 1 January 1970) is set and
    /// block time exceeds this value, the insurance is expired.
    /// Once an insurance is expired, it can be returned to the original funder (via "refund").
    pub end_time: Option<u64>,
    /// Balance in Native and Cw20 tokens
    pub balance: GenericBalance,
    /// All possible contracts that we accept tokens from
    pub cw20_whitelist: Vec<Addr>,
}

impl Insurance {
    pub fn is_expired(&self, env: &Env) -> bool {
        if let Some(end_height) = self.end_height {
            if env.block.height > end_height {
                return true;
            }
        }

        if let Some(end_time) = self.end_time {
            if env.block.time > Timestamp::from_seconds(end_time) {
                return true;
            }
        }

        false
    }

    pub fn human_whitelist(&self) -> Vec<String> {
        self.cw20_whitelist.iter().map(|a| a.to_string()).collect()
    }
}

pub const INSURANCES: Map<&str, Insurance> = Map::new("Insurance");

/// This returns the list of ids for all registered insurances
pub fn all_insurance_ids(storage: &dyn Storage) -> StdResult<Vec<String>> {
    INSURANCES
        .keys(storage, None, None, Order::Ascending)
        .collect()
}
