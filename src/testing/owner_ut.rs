use crate::owner::{set_owner, get_owner};
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