use cosmwasm_std::{Addr, Timestamp, Uint128};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::lottery::{Lottery, WinnerSelection};

/// Initial contract state.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct InstantiateMsg {}

/// Executable contract endpoints.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum ExecuteMsg {
  OnCreateLottery {
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
  },
  OnEndLottery {},
  OnBuyTickets {
    new_ticket_count: u32,
  },
  WhitelistAdd {
    addr: Addr,
  },
  WhitelistRemove {
    addr: Addr,
  },
}

/// Custom contract query endpoints.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum QueryMsg {
  GetActiveLotteries {},
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct MigrateMsg {}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct GetLotteriesResponse {
  pub lotteries: Vec<Lottery>,
}
