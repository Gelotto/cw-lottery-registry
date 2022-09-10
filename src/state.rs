use crate::error::ContractError;
use crate::lottery::Lottery;
use crate::msg::InstantiateMsg;
use cosmwasm_std::{Addr, DepsMut, Env, MessageInfo};
use cw_storage_plus::{Item, Map};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct Metadata {
  pub active_lottery_count: u32,
  pub closed_lottery_count: u32,
}

pub const METADATA: Item<Metadata> = Item::new("metadata");
pub const WHITELIST: Map<Addr, bool> = Map::new("creator_whitelist");
pub const ACTIVE_SET: Map<Addr, Lottery> = Map::new("active_set");
pub const CLOSED_SET: Map<Addr, Lottery> = Map::new("closed_set");

/// Initialize contract state data.
pub fn initialize(
  deps: DepsMut,
  _env: &Env,
  info: &MessageInfo,
  _msg: &InstantiateMsg,
) -> Result<(), ContractError> {
  // auto-add registry creator to the lottery-creator whitelist
  WHITELIST.save(deps.storage, info.sender.clone(), &true)?;

  // init global lottery metadata
  METADATA.save(
    deps.storage,
    &Metadata {
      active_lottery_count: 0,
      closed_lottery_count: 0,
    },
  )?;
  Ok(())
}
