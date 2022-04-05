use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use crate::state::Hotel;

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct InstantiateMsg {}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum ExecuteMsg {
    CreateHotel {name:String, rooms:u32,price_per_day:u32},
    TakeRoom{hotel_name:String,days:u32},
    TakeFunds{hotel_name:String},
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum QueryMsg {
    GetHotel {name:String},
}



#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct HotelResponse {
    pub hotel: Hotel,
}
