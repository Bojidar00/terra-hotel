use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use cosmwasm_std::Addr;
use cw_storage_plus::Item;
use cw_storage_plus::Map;

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct State {
    pub owner: Addr,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct Hotel {
    pub name: String,
    pub owner:Addr,
    pub rooms_count: u32,
    pub price_per_day: u32,
    pub free_rooms:u32,
    pub taken_rooms: Vec<u64>,
    pub generated_funds: u32,
}

pub const STATE: Item<State> = Item::new("state");
pub const HOTELS: Map<String,Hotel>=Map::new("hotels");
