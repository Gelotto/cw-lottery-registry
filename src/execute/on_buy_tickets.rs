use crate::error::ContractError;
use crate::state::ACTIVE_SET;
use cosmwasm_std::{attr, DepsMut, Env, MessageInfo, Response};

pub fn on_buy_tickets(
  deps: DepsMut,
  _env: Env,
  info: MessageInfo,
  new_ticket_count: u32,
) -> Result<Response, ContractError> {
  ACTIVE_SET.update(
    deps.storage,
    info.sender.clone(),
    |some_lottery| -> Result<_, ContractError> {
      if let Some(mut lottery) = some_lottery {
        if info.sender != lottery.addr {
          // only the lottery contract has authority to trigger this callback
          return Err(ContractError::NotAuthorized {});
        }
        lottery.ticket_count = new_ticket_count;
        Ok(lottery)
      } else {
        Err(ContractError::NotActive {})
      }
    },
  )?;

  Ok(Response::new().add_attributes(vec![attr("action", "on_buy_tickets")]))
}
