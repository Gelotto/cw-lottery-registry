use crate::error::ContractError;
use crate::state::{ACTIVE_SET, CLOSED_SET, METADATA};
use cosmwasm_std::{attr, DepsMut, Env, MessageInfo, Response};

pub fn on_end_lottery(
  deps: DepsMut,
  _env: Env,
  info: MessageInfo,
) -> Result<Response, ContractError> {
  if !ACTIVE_SET.has(deps.storage, info.sender.clone()) {
    return Err(ContractError::NotActive {});
  }

  if CLOSED_SET.has(deps.storage, info.sender.clone()) {
    return Err(ContractError::AlreadyEnded {});
  }

  let lottery = ACTIVE_SET.load(deps.storage, info.sender.clone())?;

  // only the lottery contract has authority to trigger this callback
  if info.sender != lottery.addr {
    return Err(ContractError::NotAuthorized {});
  }

  // update global counts
  METADATA.update(deps.storage, |mut meta| -> Result<_, ContractError> {
    meta.active_lottery_count -= 1;
    meta.closed_lottery_count += 1;
    Ok(meta)
  })?;

  // move lottery from active to closed set
  ACTIVE_SET.remove(deps.storage, info.sender.clone());
  CLOSED_SET.save(deps.storage, info.sender.clone(), &lottery)?;

  Ok(Response::new().add_attributes(vec![attr("action", "on_end_lottery")]))
}
