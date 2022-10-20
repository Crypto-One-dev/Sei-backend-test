use crate::state::{
    Balance, Owner,
};
use cosmwasm_std::{ Coin, Uint128 };
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct MigrateMsg {}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct InstantiateMsg {}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum SudoMsg {
    SetOwner {
        owner: String,
    },
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum QueryMsg {
    GetBalance { account: String, denom: String },
    GetOwner {},
}

#[derive(Serialize, Deserialize, Clone, PartialEq, JsonSchema)]
pub struct GetBalanceResponse {
    pub balance: Balance,
}

#[derive(Serialize, Deserialize, Clone, PartialEq, JsonSchema)]
pub struct GetOwnerResponse {
    pub owner: Owner,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum ExecuteMsg {
    Deposit {receiver1: String, receiver2: String},
    Withdraw { coins: Vec<Coin> },
    SetOwner{ owner: String },
    SetFee {fee: Uint128},
}
