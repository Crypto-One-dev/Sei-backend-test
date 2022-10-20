use crate::state::Owner as OwnerState;
use crate::error::ContractError;
use cosmwasm_std::Storage;

use cosmwasm_std::{ Response };
use cw_storage_plus::Item;

pub const OWNER: Item<OwnerState> = Item::new("owner");

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
