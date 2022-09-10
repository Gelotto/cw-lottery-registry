use crate::error::ContractError;
use crate::lottery::{Lottery, WinnerSelection};
use crate::msg::Style;
use crate::state::{ACTIVE_SET, CLOSED_SET, METADATA, WHITELIST};
use cosmwasm_std::{attr, Addr, DepsMut, Env, MessageInfo, Response, Timestamp, Uint128};

pub fn on_create_lottery(
  deps: DepsMut,
  _env: Env,
  _info: MessageInfo,
  creator: Addr,
  code_id: u32,
  addr: Addr,
  name: Option<String>,
  denom: String,
  cw20_token_address: Option<Addr>,
  ticket_price: Uint128,
  ticket_count: u32,
  ends_after: Option<Timestamp>,
  funding_threshold: Option<Uint128>,
  winner_selection: WinnerSelection,
  duration_minutes: Option<u32>,
  style: Style,
) -> Result<Response, ContractError> {
  // only whitelisted senders can register games
  if !WHITELIST.has(deps.storage, creator.clone()) {
    return Err(ContractError::NotAuthorized {});
  }

  // don't allow a lottery to be registered multiple times
  if ACTIVE_SET.has(deps.storage, addr.clone()) || CLOSED_SET.has(deps.storage, addr.clone()) {
    return Err(ContractError::AlreadyRegistered { addr });
  }

  // save lottery data in active set
  ACTIVE_SET.save(
    deps.storage,
    addr.clone(),
    &Lottery {
      style,
      addr: addr.clone(),
      code_id,
      name,
      denom,
      cw20_token_address,
      ticket_price,
      ticket_count,
      ends_after,
      funding_threshold,
      winner_selection,
      duration_minutes,
    },
  )?;

  // increment global lottery game count
  METADATA.update(deps.storage, |mut meta| -> Result<_, ContractError> {
    meta.active_lottery_count += 1;
    Ok(meta)
  })?;

  Ok(Response::new().add_attributes(vec![
    attr("action", "on_create_lottery"),
    attr("code_id", code_id.to_string()),
    attr("addr", addr.clone().to_string()),
  ]))
}
