use cosmwasm_std::testing::{MockApi, MockQuerier, MockStorage};
use cosmwasm_std::{ Coin, OwnedDeps};
use sei_cosmwasm::{
    SeiQueryWrapper, 
};
use std::marker::PhantomData;

pub fn mock_dependencies(
    contract_balance: &[Coin],
) -> OwnedDeps<MockStorage, MockApi, MockQuerier<SeiQueryWrapper>, SeiQueryWrapper> {
    let mock_querier = MockQuerier::new(&[("addr0001", contract_balance)]);

    OwnedDeps {
        storage: MockStorage::default(),
        api: MockApi::default(),
        querier: mock_querier,
        custom_query_type: PhantomData,
    }
}