use crate::state::Owner as OwnerState;
use crate::state::Fee as FeeState;
use crate::error::ContractError;
use cosmwasm_std::{Storage, Uint128};

use cosmwasm_std::{ Response };
use cw_storage_plus::Item;

pub const OWNER: Item<OwnerState> = Item::new("owner");
pub const FEE: Item<FeeState> = Item::new("fee");

pub fn set_owner(storage: &mut dyn Storage, owner: String) -> Result<Response, ContractError> {
    let own = OwnerState {
        account: owner
    };
    OWNER.save(storage, &own).unwrap();
    Ok(Response::default())
}

pub fn get_owner(storage: & dyn Storage) -> Result<OwnerState, cosmwasm_std::StdError>  {
    OWNER.load(storage)
}

pub fn set_fee(storage: &mut dyn Storage, new_fee: Uint128) -> Result<Response, ContractError> {
    let resp = FeeState {
        fee: new_fee
    };
    FEE.save(storage, &resp).unwrap();
    Ok(Response::default())
}

pub fn get_fee(storage: & dyn Storage) -> Result<FeeState, cosmwasm_std::StdError>  {
    FEE.load(storage)
}