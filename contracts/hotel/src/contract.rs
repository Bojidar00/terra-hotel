#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{to_binary, Binary, Deps, DepsMut, Env, MessageInfo, Response, StdResult,BankMsg,Coin, CosmosMsg,Attribute,attr};
use cw2::set_contract_version;

use crate::error::ContractError;
use crate::msg::{HotelResponse, CountResponse, ExecuteMsg, InstantiateMsg, QueryMsg};
use crate::state::{State, STATE, Hotel, HOTELS};

// version info for migration info
const CONTRACT_NAME: &str = "crates.io:counter";
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    let state = State {
        count: msg.count,
        owner: info.sender.clone(),
    };
    set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;
    STATE.save(deps.storage, &state)?;

    Ok(Response::new()
        .add_attribute("method", "instantiate")
        .add_attribute("owner", info.sender)
        .add_attribute("count", msg.count.to_string()))
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    match msg {
        ExecuteMsg::Increment {} => try_increment(deps),
        ExecuteMsg::Reset { count } => try_reset(deps, info, count),
        ExecuteMsg::CreateHotel{name,rooms,price_per_day}=>try_create_hotel(deps,info,name,rooms,price_per_day),
        ExecuteMsg::TakeRoom{hotel_name,days}=>try_take_room(deps,info,env,hotel_name,days),
        ExecuteMsg::TakeFunds{hotel_name}=>take_funds(deps, info,env, hotel_name),
    }
}

pub fn try_increment(deps: DepsMut) -> Result<Response, ContractError> {
    STATE.update(deps.storage, |mut state| -> Result<_, ContractError> {
        state.count += 1;
        Ok(state)
    })?;

    Ok(Response::new().add_attribute("method", "try_increment"))
}
pub fn try_reset(deps: DepsMut, info: MessageInfo, count: i32) -> Result<Response, ContractError> {
    STATE.update(deps.storage, |mut state| -> Result<_, ContractError> {
        if info.sender != state.owner {
            return Err(ContractError::Unauthorized {});
        }
        state.count = count;
        Ok(state)
    })?;
    Ok(Response::new().add_attribute("method", "reset"))
}

pub fn try_create_hotel(deps: DepsMut,info: MessageInfo,name:String,rooms:u32,price_per_day:u32)->Result<Response,ContractError>{
    HOTELS.update(deps.storage,name.clone(), | hotel: Option<Hotel>| -> Result<_, ContractError> {
        match hotel {
            Some(_hotel)=>{return Err(ContractError::Exists)},
            None =>{let hotel = Hotel{name:name.clone(),owner:info.sender , rooms_count:rooms.clone(),price_per_day:price_per_day,free_rooms:rooms,taken_rooms:Vec::new(),generated_funds:0};Ok(hotel)},
        }
        
    })?;

    Ok(Response::new().add_attribute("method", "create").add_attribute("hotel_name",name ))
}

pub fn try_take_room(deps:DepsMut,info: MessageInfo,env:Env, hotel_name:String, days:u32)->Result<Response,ContractError>{
   
    HOTELS.update(deps.storage,hotel_name.clone(), | hotel: Option<Hotel>| -> Result<_, ContractError> {
        match hotel {
            Some(mut _hotel)=>{_hotel = check_rooms(_hotel,env.clone());if _hotel.free_rooms>0{
               if (_hotel.price_per_day * days) as u128 <= info.funds[0].amount.u128(){ 
                  
                let time = env.block.time;
                let millis = (time.seconds() * 1_000) + (time.nanos() / 1_000_000);
                   _hotel.free_rooms-=1;
                   _hotel.taken_rooms.push(millis+(days as u64)*(60*60*24*1000));
                   _hotel.generated_funds+=_hotel.price_per_day * days;

                   
                
                   Ok(_hotel)
            }else{return Err(ContractError::InsufficientFunds)}
            }else{return Err(ContractError::NoFreeRooms)}},
            None =>{return Err(ContractError::NoHotel)},
        }
        
    })?;
    Ok(Response::new())
}

pub fn check_rooms(mut hotel: Hotel,env:Env) -> Hotel{
    let time = env.block.time;
    let millis = (time.seconds() * 1_000) + (time.nanos() / 1_000_000);
    let mut i=0;


    
    for f in hotel.taken_rooms.clone() {
        if f > millis {
            hotel.taken_rooms.swap_remove(i);
            hotel.free_rooms+=1;
        }
        i+=1;
    }
    

    hotel
}

pub fn take_funds(deps:DepsMut,_info: MessageInfo,_env:Env, hotel_name:String)->Result<Response,ContractError>{
    let mut msgs:Vec<CosmosMsg> = Vec::new();
    let attrs: Vec<Attribute> = vec![attr("action", "send")];
    let hotel = HOTELS.load(deps.storage, hotel_name.clone())?;
    if hotel.generated_funds >0 {
        let amount =vec![ Coin::new((hotel.generated_funds * 1000000 ) as u128, "uluna")];
        msgs.push(CosmosMsg::Bank(BankMsg::Send{to_address:hotel.owner.to_string(),amount:amount}));
    } else{
        return Err(ContractError::NoFunds)
    }
    HOTELS.update(deps.storage,hotel_name, | hotel: Option<Hotel>| -> Result<_, ContractError> {
        let mut h = hotel.unwrap();
        h.generated_funds=0;
        Ok(h)
    })?;
Ok(Response::new().add_attributes(attrs).add_messages(msgs))

}


#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::GetCount {} => to_binary(&query_count(deps)?),
        QueryMsg::GetHotel {name} => to_binary(&query_hotel(deps,name)?),
    }
}

fn query_count(deps: Deps) -> StdResult<CountResponse> {
    let state = STATE.load(deps.storage)?;
    Ok(CountResponse { count: state.count })
}

fn query_hotel(deps: Deps, name:String)->StdResult<HotelResponse>{
    let hotel = HOTELS.load(deps.storage, name)?;
    Ok(HotelResponse{hotel:hotel})
}

#[cfg(test)]
mod tests {
    use super::*;
    use cosmwasm_std::testing::{mock_dependencies, mock_env, mock_info};
    use cosmwasm_std::{coins, from_binary};


    #[test]
    fn take_room() {
        let mut deps = mock_dependencies(&coins(2, "token"));
        

        let msg = InstantiateMsg { count: 0 };
        let info = mock_info("creator", &coins(2, "token"));
        let _res = instantiate(deps.as_mut(), mock_env(), info, msg).unwrap();

       
        let info = mock_info("anyone", &coins(2, "token"));
        let msg = ExecuteMsg::CreateHotel {name:"myhotel".to_string(),rooms:2,price_per_day:1};
        let _res = execute(deps.as_mut(), mock_env(), info, msg).unwrap();

        let info = mock_info("anyone2", &coins(2, "luna"));
        let msg = ExecuteMsg::TakeRoom {hotel_name:"myhotel".to_string(),days:2};
        let _res = execute(deps.as_mut(), mock_env(), info, msg).unwrap();

       
        let res = query(deps.as_ref(), mock_env(), QueryMsg::GetHotel {name:"myhotel".to_string()}).unwrap();
        let value: HotelResponse = from_binary(&res).unwrap();
        assert_eq!(1, value.hotel.free_rooms);
    }
    #[test]
    fn release_room() {
        let mut deps = mock_dependencies(&coins(2, "token"));
        let  env = mock_env();
        

        let msg = InstantiateMsg { count: 0 };
        let info = mock_info("creator", &coins(2, "token"));
        let _res = instantiate(deps.as_mut(), env.clone(), info, msg).unwrap();

       
        let info = mock_info("anyone", &coins(2, "token"));
        let msg = ExecuteMsg::CreateHotel {name:"myhotel".to_string(),rooms:2,price_per_day:1};
        let _res = execute(deps.as_mut(), env.clone(), info, msg).unwrap();

        let info = mock_info("anyone2", &coins(1, "luna"));
        let msg = ExecuteMsg::TakeRoom {hotel_name:"myhotel".to_string(),days:1};
        let _res = execute(deps.as_mut(), env.clone(), info, msg).unwrap();

       
        let res = query(deps.as_ref(),env.clone(), QueryMsg::GetHotel {name:"myhotel".to_string()}).unwrap();
        let value: HotelResponse = from_binary(&res).unwrap();
        assert_eq!(1, value.hotel.free_rooms);

        env.block.time.plus_seconds((60*60*24*2)+1000);

        let info = mock_info("anyone3", &coins(2, "luna"));
        let msg = ExecuteMsg::TakeRoom {hotel_name:"myhotel".to_string(),days:2};
        let _res = execute(deps.as_mut(), env.clone(), info, msg).unwrap();

        let res = query(deps.as_ref(),env.clone(), QueryMsg::GetHotel {name:"myhotel".to_string()}).unwrap();
        let value: HotelResponse = from_binary(&res).unwrap();
        assert_eq!(1, value.hotel.free_rooms);

       


    }

    #[test]
    fn take_funds() {
       let mut deps = mock_dependencies(&coins(2, "token"));
        let  env = mock_env();
        

        let msg = InstantiateMsg { count: 0 };
        let info = mock_info("creator", &coins(2, "token"));
        let _res = instantiate(deps.as_mut(), env.clone(), info, msg).unwrap();

       
        let info = mock_info("anyone", &coins(2, "token"));
        let msg = ExecuteMsg::CreateHotel {name:"myhotel".to_string(),rooms:2,price_per_day:1};
        let _res = execute(deps.as_mut(), env.clone(), info, msg).unwrap();

        let info = mock_info("anyone2", &coins(1, "luna"));
        let msg = ExecuteMsg::TakeRoom {hotel_name:"myhotel".to_string(),days:1};
        let _res = execute(deps.as_mut(), env.clone(), info, msg).unwrap();

        let res = query(deps.as_ref(),env.clone(), QueryMsg::GetHotel {name:"myhotel".to_string()}).unwrap();
        let value: HotelResponse = from_binary(&res).unwrap();
        assert_eq!(1, value.hotel.generated_funds);

        let info = mock_info("anyone3", &coins(1, "luna"));
        let msg = ExecuteMsg::TakeFunds {hotel_name:"myhotel".to_string()};
        let _res = execute(deps.as_mut(), env.clone(), info, msg).unwrap();

        let res = query(deps.as_ref(),env.clone(), QueryMsg::GetHotel {name:"myhotel".to_string()}).unwrap();
        let value: HotelResponse = from_binary(&res).unwrap();
        assert_eq!(0, value.hotel.generated_funds);

       
    
    } 

  
}
