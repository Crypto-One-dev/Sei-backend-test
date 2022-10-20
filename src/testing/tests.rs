use crate::balance::get_balance;
use crate::contract::{execute, instantiate, migrate, query, sudo};
use crate::error::ContractError;
use crate::msg::{
    ExecuteMsg, GetBalanceResponse, InstantiateMsg,
    MigrateMsg, QueryMsg, SudoMsg, GetOwnerResponse,
};
use crate::state::{ Balance };
use crate::testing::mock_querier::mock_dependencies;
use cosmwasm_std::testing::{mock_env, mock_info};
use cosmwasm_std::{from_binary, Coin, Decimal, StdError};

#[test]
fn test_owner_query() {
    let mut deps = mock_dependencies(&vec![]);
    let env = mock_env();
    instantiate(
        deps.as_mut(),
        env,
        mock_info("admin", &[]),
        InstantiateMsg {},
    )
    .unwrap();

    let msg = SudoMsg::SetOwner{
        owner: "test".to_owned()
    };
    sudo(deps.as_mut(), mock_env(), msg).unwrap();

    let balance_query = QueryMsg::GetBalance {
        account: "test".to_owned(),
        denom: "usei".to_owned(),
    };
    let balance_bin = query(deps.as_ref(), mock_env(), balance_query).unwrap();
    let resp: GetBalanceResponse = from_binary(&balance_bin).unwrap();
    assert_eq!(
        Balance {
            amount: Decimal::zero(),
            withheld: Decimal::zero()
        },
        resp.balance
    );

    //check initial owner
    let owner_query = QueryMsg::GetOwner {  };
    let owner_bin = query(deps.as_ref(), mock_env(), owner_query).unwrap();
    let resp: GetOwnerResponse = from_binary(&owner_bin).unwrap();
    assert_eq!("test".to_owned(), resp.owner.account);

    //update owner
    let new_owner = "newOwner".to_owned();
    let msg = ExecuteMsg::SetOwner { owner: new_owner };
    execute(
        deps.as_mut(),
        mock_env(),
        mock_info("newOwner", &vec![]),
        msg,
    )
    .unwrap();
    let owner_query = QueryMsg::GetOwner {  };
    let new_owner_bin = query(deps.as_ref(), mock_env(), owner_query).unwrap();
    let res_new_owner: GetOwnerResponse = from_binary(&new_owner_bin).unwrap();
    assert_eq!("newOwner".to_owned(), res_new_owner.owner.account);

}

#[test]
fn test_deposit_withdraw() {
    let mut deps = mock_dependencies(&vec![]);
    let env = mock_env();
    instantiate(
        deps.as_mut(),
        env,
        mock_info("admin", &[]),
        InstantiateMsg {},
    )
    .unwrap();

    let receiver1 = "receiver1".to_owned();
    let receiver2 = "receiver2".to_owned();

    let msg = ExecuteMsg::Deposit { receiver1, receiver2 };
    execute(
        deps.as_mut(),
        mock_env(),
        mock_info("test", &vec![Coin::new(100, "usei")]),
        msg,
    )
    .unwrap();

    let balance1 = get_balance(deps.as_ref().storage, "receiver1".to_owned(), "usei".to_owned());
    assert_eq!(balance1.amount, Decimal::from_atomics(50u128, 0).unwrap());

    let balance2 = get_balance(deps.as_ref().storage, "receiver2".to_owned(), "usei".to_owned());
    assert_eq!(balance2.amount, Decimal::from_atomics(50u128, 0).unwrap());

    assert_eq!(balance1.withheld, Decimal::zero());
    assert_eq!(balance2.withheld, Decimal::zero());

    let msg = ExecuteMsg::Withdraw {
        coins: vec![Coin::new(200, "usei")],
    };
    match execute(deps.as_mut(), mock_env(), mock_info("receiver1", &[]), msg) {
        Ok(_) => panic!("Withdrawing more than you have is no ok"),
        Err(_) => (),
    };
    let msg = ExecuteMsg::Withdraw {
        coins: vec![Coin::new(49, "usei")],
    };
    execute(deps.as_mut(), mock_env(), mock_info("receiver1", &[]), msg).unwrap();
    let balance = get_balance(deps.as_ref().storage, "receiver1".to_owned(), "usei".to_owned());
    assert_eq!(balance.amount, Decimal::one());
    assert_eq!(balance.withheld, Decimal::zero());
}

#[test]
fn test_migration() {
    let mut deps = mock_dependencies(&vec![]);
    let instantiate_msg = InstantiateMsg {};
    let info = mock_info("", &vec![]);
    instantiate(deps.as_mut(), mock_env(), info.clone(), instantiate_msg).unwrap();

    // test incorrect contract name to assert error
    cw2::set_contract_version(&mut deps.storage, "this_is_the_wrong_contract", "0.0.1").unwrap();
    let res = migrate(deps.as_mut(), mock_env(), MigrateMsg {});
    match res {
        Err(ContractError::Std(x)) => {
            assert_eq!(x, StdError::generic_err("Can only upgrade from same type"))
        }
        _ => panic!("This should raise error on contract type mismatch"),
    };
}
