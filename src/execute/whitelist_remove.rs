use crate::{constants::ADMIN_ADDRESS, error::ContractError, state::WHITELIST};
use cosmwasm_std::{attr, Addr, DepsMut, Env, MessageInfo, Response};

pub fn whitelist_remove(
  deps: DepsMut,
  _env: Env,
  info: MessageInfo,
  addr: Addr,
) -> Result<Response, ContractError> {
  // only allow admin to do this
  if info.sender != ADMIN_ADDRESS {
    return Err(ContractError::NotAuthorized {});
  }

  WHITELIST.remove(deps.storage, addr.clone());

  Ok(Response::new().add_attributes(vec![
    attr("action", "add_address_to_whitelist"),
    attr("addr", addr.clone().to_string()),
  ]))
}
