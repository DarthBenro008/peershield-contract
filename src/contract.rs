use std::sync::mpsc::Sender;

#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{
    coins, from_binary, to_binary, Addr, BankMsg, Binary, Deps, DepsMut, Env, MessageInfo,
    Response, StdResult, SubMsg, WasmMsg,
};

use cw2::set_contract_version;
use cw20::{Balance, Cw20Coin, Cw20CoinVerified, Cw20ExecuteMsg, Cw20ReceiveMsg};
use cw_utils::NativeBalance;

use crate::error::ContractError;
use crate::msg::{
    ClaimRequestsResponse, CoveragePoolViewResponse, CreateMsg, DetailsResponse, ExecuteMsg,
    InstantiateMsg, ListResponse, QueryMsg, ReceiveMsg,
};
use crate::state::{
    all_claim_requests, all_insurance_ids, CoveragePool, GenericBalance, Insurance,
    CLAIMS_REQUESTS, COVERAGE_POOL, INSURANCES,
};

// version info for migration info
const CONTRACT_NAME: &str = "peershield";
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    _msg: InstantiateMsg,
) -> StdResult<Response> {
    set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;
    // no setup
    Ok(Response::default())
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    match msg {
        ExecuteMsg::Create(msg) => {
            execute_create(deps, msg, Balance::from(info.funds), &info.sender)
        }
        ExecuteMsg::SetRecipient { id, recipient } => {
            execute_set_recipient(deps, env, info, id, recipient)
        }
        ExecuteMsg::Approve { id } => execute_approve(deps, env, info, id),
        ExecuteMsg::TopUp { id } => execute_top_up(deps, id, Balance::from(info.funds)),
        ExecuteMsg::Refund { id } => execute_refund(deps, env, info, id),
        ExecuteMsg::Receive(msg) => execute_receive(deps, info, msg),
        ExecuteMsg::Claim { id } => execute_claim_request(deps, env, info, id),
        ExecuteMsg::ProvideCoverage {} => execute_provide_coverage(deps, Balance::from(info.funds)),
    }
}

pub fn execute_claim_request(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    id: String,
) -> Result<Response, ContractError> {
    let insurance = INSURANCES.load(deps.storage, &id)?;

    if info.sender != insurance.arbiter {
        return Err(ContractError::Unauthorized {});
    }
    if insurance.is_expired(&env) {
        return Err(ContractError::Expired {});
    }

    CLAIMS_REQUESTS.update(deps.storage, &id, |existing| match existing {
        None => Ok(String::from("pending_approval")),
        Some(_) => Err(ContractError::AlreadyInUse {}),
    })?;
    let res = Response::new().add_attributes(vec![("action", "claim"), ("id", id.as_str())]);
    Ok(res)
}
pub fn execute_receive(
    deps: DepsMut,
    info: MessageInfo,
    wrapper: Cw20ReceiveMsg,
) -> Result<Response, ContractError> {
    let msg: ReceiveMsg = from_binary(&wrapper.msg)?;
    let balance = Balance::Cw20(Cw20CoinVerified {
        address: info.sender,
        amount: wrapper.amount,
    });
    let api = deps.api;
    match msg {
        ReceiveMsg::Create(msg) => {
            execute_create(deps, msg, balance, &api.addr_validate(&wrapper.sender)?)
        }
        ReceiveMsg::TopUp { id } => execute_top_up(deps, id, balance),
        ReceiveMsg::ProvideCoverage {} => execute_provide_coverage(deps, balance),
    }
}

pub fn execute_provide_coverage(
    deps: DepsMut,
    balance: Balance,
) -> Result<Response, ContractError> {
    if balance.is_empty() {
        return Err(ContractError::EmptyBalance {});
    }
    let cp_pool = COVERAGE_POOL.may_load(deps.storage)?;
    match cp_pool {
        None => {
            let pool_balance = match balance {
                Balance::Native(balance) => GenericBalance {
                    native: balance.0,
                    cw20: vec![],
                },
                Balance::Cw20(token) => GenericBalance {
                    native: vec![],
                    cw20: vec![token],
                },
            };
            let cp = CoveragePool { pool: pool_balance };
            COVERAGE_POOL.save(deps.storage, &cp)?;
        }
        Some(mut cp) => {
            cp.pool.add_tokens(balance);
            COVERAGE_POOL.save(deps.storage, &cp)?;
        }
    }
    let res = Response::new().add_attributes(vec![("action", "provide_coverage")]);
    Ok(res)
}

pub fn execute_create(
    deps: DepsMut,
    msg: CreateMsg,
    balance: Balance,
    sender: &Addr,
) -> Result<Response, ContractError> {
    if balance.is_empty() {
        return Err(ContractError::EmptyBalance {});
    }

    let mut cw20_whitelist = msg.addr_whitelist(deps.api)?;

    let insurance_balance = match balance {
        Balance::Native(balance) => GenericBalance {
            native: balance.0,
            cw20: vec![],
        },
        Balance::Cw20(token) => {
            // make sure the token sent is on the whitelist by default
            if !cw20_whitelist.iter().any(|t| t == &token.address) {
                cw20_whitelist.push(token.address.clone())
            }
            GenericBalance {
                native: vec![],
                cw20: vec![token],
            }
        }
    };

    let recipient: Option<Addr> = msg
        .recipient
        .and_then(|addr| deps.api.addr_validate(&addr).ok());

    let insurance = Insurance {
        arbiter: deps
            .api
            .addr_validate(&"osmo1q66vtupgt30926k3nsujtht2hf5nnmjssu4ugx")?,
        recipient,
        source: sender.clone(),
        title: msg.title,
        description: msg.description,
        end_height: msg.end_height,
        end_time: msg.end_time,
        balance: insurance_balance,
        cw20_whitelist,
    };

    let cp_pool = COVERAGE_POOL.load(deps.storage)?;

    if insurance.balance.native[0].amount > cp_pool.pool.native[0].amount {
        return Err(ContractError::InsufficientCover {});
    }

    // try to store it, fail if the id was already in use
    INSURANCES.update(deps.storage, &msg.id, |existing| match existing {
        None => Ok(insurance),
        Some(_) => Err(ContractError::AlreadyInUse {}),
    })?;

    let res = Response::new().add_attributes(vec![("action", "create"), ("id", msg.id.as_str())]);
    Ok(res)
}

pub fn execute_set_recipient(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    id: String,
    recipient: String,
) -> Result<Response, ContractError> {
    let mut insurance = INSURANCES.load(deps.storage, &id)?;
    if info.sender != insurance.arbiter {
        return Err(ContractError::Unauthorized {});
    }

    let recipient = deps.api.addr_validate(recipient.as_str())?;
    insurance.recipient = Some(recipient.clone());
    INSURANCES.save(deps.storage, &id, &insurance)?;

    Ok(Response::new().add_attributes(vec![
        ("action", "set_recipient"),
        ("id", id.as_str()),
        ("recipient", recipient.as_str()),
    ]))
}

pub fn execute_top_up(
    deps: DepsMut,
    id: String,
    balance: Balance,
) -> Result<Response, ContractError> {
    if balance.is_empty() {
        return Err(ContractError::EmptyBalance {});
    }
    // this fails is no insurance there
    let mut insurance = INSURANCES.load(deps.storage, &id)?;

    if let Balance::Cw20(token) = &balance {
        // ensure the token is on the whitelist
        if !insurance.cw20_whitelist.iter().any(|t| t == &token.address) {
            return Err(ContractError::NotInWhitelist {});
        }
    };

    insurance.balance.add_tokens(balance);

    // and save
    INSURANCES.save(deps.storage, &id, &insurance)?;

    let res = Response::new().add_attributes(vec![("action", "top_up"), ("id", id.as_str())]);
    Ok(res)
}

pub fn execute_approve(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    id: String,
) -> Result<Response, ContractError> {
    // this fails is no insurance there
    let insurance = INSURANCES.load(deps.storage, &id)?;
    let mut cp_pool = COVERAGE_POOL.load(deps.storage)?;

    if info.sender != insurance.arbiter {
        return Err(ContractError::Unauthorized {});
    }
    if insurance.is_expired(&env) {
        return Err(ContractError::Expired {});
    }

    let recipient = insurance
        .recipient
        .ok_or(ContractError::RecipientNotSet {})?;

    cp_pool
        .pool
        .remove_tokens(insurance.balance.native[0].amount);
    COVERAGE_POOL.save(deps.storage, &cp_pool)?;
    // we delete the insurance
    INSURANCES.remove(deps.storage, &id);
    CLAIMS_REQUESTS.remove(deps.storage, &id);

    // send all tokens out
    let messages: Vec<SubMsg> = send_tokens(&recipient, &insurance.balance)?;

    Ok(Response::new()
        .add_attribute("action", "approve")
        .add_attribute("id", id)
        .add_attribute("to", recipient)
        .add_submessages(messages))
}

pub fn execute_refund(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    id: String,
) -> Result<Response, ContractError> {
    // this fails is no insurance there
    let insurance = INSURANCES.load(deps.storage, &id)?;

    // the arbiter can send anytime OR anyone can send after expiration
    if !insurance.is_expired(&env) && info.sender != insurance.arbiter {
        Err(ContractError::Unauthorized {})
    } else {
        // we delete the insurance
        INSURANCES.remove(deps.storage, &id);

        // send all tokens out
        let messages = send_tokens(&insurance.source, &insurance.balance)?;

        Ok(Response::new()
            .add_attribute("action", "refund")
            .add_attribute("id", id)
            .add_attribute("to", insurance.source)
            .add_submessages(messages))
    }
}

fn send_from_cp(to: &Addr, amount: u128) -> StdResult<Vec<SubMsg>> {
    Ok(vec![SubMsg::new(BankMsg::Send {
        to_address: to.into(),
        amount: coins(amount, "osmo"),
    })])
}

fn send_tokens(to: &Addr, balance: &GenericBalance) -> StdResult<Vec<SubMsg>> {
    let native_balance = &balance.native;
    let mut msgs: Vec<SubMsg> = if native_balance.is_empty() {
        vec![]
    } else {
        vec![SubMsg::new(BankMsg::Send {
            to_address: to.into(),
            amount: native_balance.to_vec(),
        })]
    };

    let cw20_balance = &balance.cw20;
    let cw20_msgs: StdResult<Vec<_>> = cw20_balance
        .iter()
        .map(|c| {
            let msg = Cw20ExecuteMsg::Transfer {
                recipient: to.into(),
                amount: c.amount,
            };
            let exec = SubMsg::new(WasmMsg::Execute {
                contract_addr: c.address.to_string(),
                msg: to_binary(&msg)?,
                funds: vec![],
            });
            Ok(exec)
        })
        .collect();
    msgs.append(&mut cw20_msgs?);
    Ok(msgs)
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::List {} => to_binary(&query_list(deps)?),
        QueryMsg::Details { id } => to_binary(&query_details(deps, id)?),
        QueryMsg::ListClaims {} => to_binary(&query_claims_request(deps)?),
        QueryMsg::ListCoveragePool {} => to_binary(&query_coverage_pool(deps)?),
    }
}

fn query_coverage_pool(deps: Deps) -> StdResult<CoveragePoolViewResponse> {
    let cp_pool = COVERAGE_POOL.load(deps.storage)?;
    Ok(CoveragePoolViewResponse {
        pool: cp_pool.pool.native,
    })
}
fn query_claims_request(deps: Deps) -> StdResult<ClaimRequestsResponse> {
    Ok(ClaimRequestsResponse {
        insurances: all_claim_requests(deps.storage)?,
    })
}

fn query_details(deps: Deps, id: String) -> StdResult<DetailsResponse> {
    let insurance = INSURANCES.load(deps.storage, &id)?;

    let cw20_whitelist = insurance.human_whitelist();

    // transform tokens
    let native_balance = insurance.balance.native;

    let cw20_balance: StdResult<Vec<_>> = insurance
        .balance
        .cw20
        .into_iter()
        .map(|token| {
            Ok(Cw20Coin {
                address: token.address.into(),
                amount: token.amount,
            })
        })
        .collect();

    let recipient = insurance.recipient.map(|addr| addr.into_string());

    let details = DetailsResponse {
        id,
        arbiter: insurance.arbiter.into(),
        recipient,
        source: insurance.source.into(),
        title: insurance.title,
        description: insurance.description,
        end_height: insurance.end_height,
        end_time: insurance.end_time,
        native_balance,
        cw20_balance: cw20_balance?,
        cw20_whitelist,
    };
    Ok(details)
}

fn query_list(deps: Deps) -> StdResult<ListResponse> {
    Ok(ListResponse {
        insurances: all_insurance_ids(deps.storage)?,
    })
}
