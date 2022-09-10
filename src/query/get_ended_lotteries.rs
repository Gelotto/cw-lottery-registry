use crate::{lottery::Lottery, msg::GetLotteriesResponse, state::CLOSED_SET};
use cosmwasm_std::{Deps, StdResult};

pub fn get_ended_lotteries(deps: Deps) -> StdResult<GetLotteriesResponse> {
  let lotteries: Vec<Lottery> = CLOSED_SET
    .range(deps.storage, None, None, cosmwasm_std::Order::Descending)
    .map(|some_entry| some_entry.unwrap().1)
    .collect();

  Ok(GetLotteriesResponse { lotteries })
}
