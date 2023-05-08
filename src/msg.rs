use cosmwasm_schema::{cw_serde, QueryResponses};

use cosmwasm_std::{Addr, Api, Coin, StdResult};

use cw20::{Cw20Coin, Cw20ReceiveMsg};

#[cw_serde]
pub struct InstantiateMsg {}

#[cw_serde]
pub enum ExecuteMsg {
    Create(CreateMsg),
    /// Adds all sent native tokens to the contract
    TopUp {
        id: String,
    },
    /// Set the recipient of the given insurance
    SetRecipient {
        id: String,
        recipient: String,
    },
    /// Approve sends all tokens to the recipient.
    /// Only the arbiter can do this
    Approve {
        /// id is a human-readable name for the insurance from create
        id: String,
    },
    /// Refund returns all remaining tokens to the original sender,
    /// The arbiter can do this any time, or anyone can do this after a timeout
    Refund {
        /// id is a human-readable name for the insurance from create
        id: String,
    },
    /// This accepts a properly-encoded ReceiveMsg from a cw20 contract
    Receive(Cw20ReceiveMsg),
    /// Claim puts a message for claim request
    Claim {
        /// id is a human-readable name for the insurance from create
        id: String,
    },
    ProvideCoverage {},
}

#[cw_serde]
pub enum ReceiveMsg {
    Create(CreateMsg),
    /// Adds all sent native tokens to the contract
    TopUp {
        id: String,
    },
    ProvideCoverage {},
}

#[cw_serde]
pub struct CreateMsg {
    /// id is a human-readable name for the insurance to use later
    /// 3-20 bytes of utf-8 text
    pub id: String,
    /// arbiter can decide to approve or refund the insurance
    ///pub arbiter: String,
    /// if approved, funds go to the recipient
    pub recipient: Option<String>,
    /// Title of the insurance
    pub title: String,
    /// Longer description of the insurance, e.g. what conditions should be met
    pub description: String,
    /// When end height set and block height exceeds this value, the insurance is expired.
    /// Once an insurance is expired, it can be returned to the original funder (via "refund").
    pub end_height: Option<u64>,
    /// When end time (in seconds since epoch 00:00:00 UTC on 1 January 1970) is set and
    /// block time exceeds this value, the insurance is expired.
    /// Once an insurance is expired, it can be returned to the original funder (via "refund").
    pub end_time: Option<u64>,
    /// Besides any possible tokens sent with the CreateMsg, this is a list of all cw20 token addresses
    /// that are accepted by the insurance during a top-up. This is required to avoid a DoS attack by topping-up
    /// with an invalid cw20 contract. See https://github.com/CosmWasm/cosmwasm-plus/issues/19
    pub cw20_whitelist: Option<Vec<String>>,
}

impl CreateMsg {
    pub fn addr_whitelist(&self, api: &dyn Api) -> StdResult<Vec<Addr>> {
        match self.cw20_whitelist.as_ref() {
            Some(v) => v.iter().map(|h| api.addr_validate(h)).collect(),
            None => Ok(vec![]),
        }
    }
}

pub fn is_valid_name(name: &str) -> bool {
    let bytes = name.as_bytes();
    if bytes.len() < 3 || bytes.len() > 20 {
        return false;
    }
    true
}

#[cw_serde]
#[derive(QueryResponses)]
pub enum QueryMsg {
    /// Show all open insurances. Return type is ListResponse.
    #[returns(ListResponse)]
    List {},
    /// Returns the details of the named insurance, error if not created
    /// Return type: DetailsResponse.
    #[returns(DetailsResponse)]
    Details { id: String },
    #[returns(ClaimRequestsResponse)]
    ListClaims {},
    #[returns(CoveragePoolViewResponse)]
    ListCoveragePool {},
}

#[cw_serde]
pub struct ListResponse {
    /// list all registered ids
    pub insurances: Vec<String>,
}

#[cw_serde]
pub struct ClaimRequestsResponse {
    /// List all claims that are made
    pub insurances: Vec<String>,
}

#[cw_serde]
pub struct CoveragePoolViewResponse {
    pub pool: Vec<Coin>,
}

#[cw_serde]
pub struct DetailsResponse {
    /// id of this insurance
    pub id: String,
    /// arbiter can decide to approve or refund the insurance
    pub arbiter: String,
    /// if approved, funds go to the recipient
    pub recipient: Option<String>,
    /// if refunded, funds go to the source
    pub source: String,
    /// Title of the insurance
    pub title: String,
    /// Longer description of the insurance, e.g. what conditions should be met
    pub description: String,
    /// When end height set and block height exceeds this value, the insurance is expired.
    /// Once an insurance is expired, it can be returned to the original funder (via "refund").
    pub end_height: Option<u64>,
    /// When end time (in seconds since epoch 00:00:00 UTC on 1 January 1970) is set and
    /// block time exceeds this value, the insurance is expired.
    /// Once an insurance is expired, it can be returned to the original funder (via "refund").
    pub end_time: Option<u64>,
    /// Balance in native tokens
    pub native_balance: Vec<Coin>,
    /// Balance in cw20 tokens
    pub cw20_balance: Vec<Cw20Coin>,
    /// Whitelisted cw20 tokens
    pub cw20_whitelist: Vec<String>,
}
