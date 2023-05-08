use cosmwasm_std::StdError;
use thiserror::Error;

#[derive(Error, Debug, PartialEq)]
pub enum ContractError {
    #[error("{0}")]
    Std(#[from] StdError),

    #[error("Unauthorized")]
    Unauthorized {},

    #[error("Only accepts tokens in the cw20_whitelist")]
    NotInWhitelist {},

    #[error("Insurance is expired")]
    Expired {},

    #[error("Send some coins to create an Insurance")]
    EmptyBalance {},

    #[error("Insurance id already in use")]
    AlreadyInUse {},

    #[error("Recipient is not set")]
    RecipientNotSet {},

    #[error("Coverage Pool doesn't have funds to cover")]
    InsufficientCover {},
}
