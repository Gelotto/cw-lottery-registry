use cosmwasm_std::{Addr, Timestamp, Uint128};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

/// WinnerSelection defines the number of and manner in which winners are chosen
/// when a game ends.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum WinnerSelection {
  Fixed {
    // Ex: [60, 30, 10] means 60% to 1st place, 30% to 2nd, 10% to 3rd
    pct_split: Vec<u8>,
    winner_count: u32,
    max_winner_count: Option<u32>,
  },
  Percent {
    // Ex: 2 means that max(1, 0.02 * player_count) win
    pct_player_count: u8,
  },
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct Lottery {
  pub code_id: u32,
  pub addr: Addr,
  pub name: Option<String>,
  pub denom: String,
  pub cw20_token_address: Option<Addr>,
  pub ticket_price: Uint128,
  pub ticket_count: u32,
  pub ends_after: Option<Timestamp>,
  pub funding_threshold: Option<Uint128>,
  pub winner_selection: WinnerSelection,
  pub duration_minutes: Option<u32>,
}
