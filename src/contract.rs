#[cfg(not(feature = "library"))]
use crate::error::ContractError;
use crate::execute;
use crate::msg::{ExecuteMsg, InstantiateMsg, MigrateMsg, QueryMsg};
use crate::query;
use crate::state;
use cosmwasm_std::{
  entry_point, to_binary, Binary, Deps, DepsMut, Env, MessageInfo, Response, StdResult,
};
use cw2::set_contract_version;

const CONTRACT_NAME: &str = "crates.io:cw-contract-template";
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
  deps: DepsMut,
  env: Env,
  info: MessageInfo,
  msg: InstantiateMsg,
) -> Result<Response, ContractError> {
  set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;
  state::initialize(deps, &env, &info, &msg)?;
  Ok(Response::new().add_attribute("action", "instantiate"))
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
  deps: DepsMut,
  env: Env,
  info: MessageInfo,
  msg: ExecuteMsg,
) -> Result<Response, ContractError> {
  match msg {
    ExecuteMsg::OnCreateLottery {
      code_id,
      addr,
      name,
      denom,
      cw20_token_address,
      ticket_price,
      ticket_count,
      ends_after,
      funding_threshold,
      winner_selection,
      duration_minutes,
    } => execute::on_create_lottery(
      deps,
      env,
      info,
      code_id,
      addr,
      name,
      denom,
      cw20_token_address,
      ticket_price,
      ticket_count,
      ends_after,
      funding_threshold,
      winner_selection,
      duration_minutes,
    ),
    ExecuteMsg::OnEndLottery {} => execute::on_end_lottery(deps, env, info),
    ExecuteMsg::OnBuyTickets { new_ticket_count } => {
      execute::on_buy_tickets(deps, env, info, new_ticket_count)
    },
    ExecuteMsg::WhitelistAdd { addr } => execute::whitelist_add(deps, env, info, addr),
    ExecuteMsg::WhitelistRemove { addr } => execute::whitelist_remove(deps, env, info, addr),
  }
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(
  deps: Deps,
  _env: Env,
  msg: QueryMsg,
) -> StdResult<Binary> {
  let result = match msg {
    QueryMsg::GetActiveLotteries {} => to_binary(&query::get_active_lotteries(deps)?),
  }?;
  Ok(result)
}

#[entry_point]
pub fn migrate(
  _deps: DepsMut,
  _env: Env,
  _msg: MigrateMsg,
) -> Result<Response, ContractError> {
  // No state migrations performed, just returned a Response
  Ok(Response::default())
}
