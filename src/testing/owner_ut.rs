use cosmwasm_std::Uint128;

use crate::owner::{set_owner, get_owner, set_fee, get_fee};
use crate::testing::mock_querier::mock_dependencies;

const TEST_ACCOUNT: &str = "account";

#[test]
fn test_set_get_owner() {
    let mut deps = mock_dependencies(&vec![]);
    let owner = TEST_ACCOUNT.to_owned();
    set_owner(deps.as_mut().storage, owner).unwrap();
    let stored_owner = get_owner(deps.as_ref().storage).unwrap();
    assert_eq!("account".to_owned(), stored_owner.account);
}

#[test]
fn test_set_get_fee() {
    let mut deps = mock_dependencies(&vec![]);
    let fee = Uint128::from(5u128);
    set_fee(deps.as_mut().storage, fee).unwrap();
    let stored_fee = get_fee(deps.as_ref().storage).unwrap();
    assert_eq!(fee, stored_fee.fee);
}