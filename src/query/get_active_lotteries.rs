use crate::{lottery::Lottery, msg::GetLotteriesResponse, state::ACTIVE_SET};
use cosmwasm_std::{Deps, StdResult};

pub fn get_active_lotteries(deps: Deps) -> StdResult<GetLotteriesResponse> {
  let lotteries: Vec<Lottery> = ACTIVE_SET
    .range(deps.storage, None, None, cosmwasm_std::Order::Descending)
    .map(|some_entry| some_entry.unwrap().1)
    .collect();

  Ok(GetLotteriesResponse { lotteries })
}
