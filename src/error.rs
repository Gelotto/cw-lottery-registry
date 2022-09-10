use cosmwasm_std::{Addr, StdError};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum ContractError {
  #[error("{0}")]
  Std(#[from] StdError),

  #[error("AlreadyRegistered")]
  AlreadyRegistered { addr: Addr },

  #[error("ValidationError")]
  ValidationError {},

  #[error("NotAuthorized")]
  NotAuthorized {},

  #[error("NotActive")]
  NotActive {},

  #[error("AlreadyEnded")]
  AlreadyEnded {},
}
