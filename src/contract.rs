use crate::balance::{get_balance, save_balance};
use crate::error::ContractError;
use crate::msg::{
    ExecuteMsg, GetBalanceResponse, InstantiateMsg,
    MigrateMsg, QueryMsg, SudoMsg, GetOwnerResponse
};
use crate::owner::{get_owner, set_owner, set_fee, get_fee};
use cosmwasm_std::{
    entry_point, to_binary, BankMsg, Binary, Coin, Decimal, Deps, DepsMut, Env, MessageInfo,
    Response, StdError, StdResult, Uint128,
};
use cw2::set_contract_version;
use sei_cosmwasm::SeiQueryWrapper;
use semver::{Error as SemErr, Version};

// version info for migration info
const CONTRACT_NAME: &str = "crates.io:sei-test";
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");

// NOTE: New migrations may need store migrations if store changes are being made
#[cfg_attr(not(feature = "library"), entry_point)]
pub fn migrate(
    deps: DepsMut<SeiQueryWrapper>,
    _env: Env,
    _msg: MigrateMsg,
) -> Result<Response, ContractError> {
    let ver = cw2::get_contract_version(deps.storage)?;
    // ensure we are migrating from an allowed contract
    if ver.contract != CONTRACT_NAME {
        return Err(StdError::generic_err("Can only upgrade from same type").into());
    }

    let storage_version: Version = ver
        .version
        .parse()
        .map_err(|err: SemErr| ContractError::SemVer(err.to_string()))?;
    let version: Version = CONTRACT_VERSION
        .parse()
        .map_err(|err: SemErr| ContractError::SemVer(err.to_string()))?;
    if storage_version >= version {
        return Err(StdError::generic_err("Cannot upgrade from a newer version").into());
    }

    // set the new version
    cw2::set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;

    Ok(Response::default())
}

#[entry_point]
pub fn instantiate(
    deps: DepsMut<SeiQueryWrapper>,
    _env: Env,
    msg_info: MessageInfo,
    _: InstantiateMsg,
) -> Result<Response, ContractError> {
    set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;
    set_owner(deps.storage, msg_info.sender.to_string())?;
    set_fee(deps.storage, Uint128::from(2u128))?;
    Ok(Response::default())
}

#[entry_point]
pub fn sudo(
    deps: DepsMut<SeiQueryWrapper>,
    _: Env,
    msg: SudoMsg,
) -> Result<Response, ContractError> {
    match msg {
        SudoMsg::SetOwner { owner } => set_new_owner(deps, owner),
    }
}

#[entry_point]
pub fn query(deps: Deps<SeiQueryWrapper>, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::GetBalance { account, denom } => get_balance_query(deps, account, denom),
        QueryMsg::GetOwner {} => get_owner_query(deps),
    }
}

fn get_owner_query(
    deps: Deps<SeiQueryWrapper>,
) -> StdResult<Binary> {
    let owner = get_owner(deps.storage)?;
    let resp = GetOwnerResponse {owner: owner};
    to_binary(&resp)
}

fn get_balance_query(
    deps: Deps<SeiQueryWrapper>,
    account: String,
    denom: String,
) -> StdResult<Binary> {
    let balance = get_balance(deps.storage, account.to_owned(), denom.to_owned());
    let resp = GetBalanceResponse { balance: balance };
    to_binary(&resp)
}

#[entry_point]
pub fn execute(
    deps: DepsMut<SeiQueryWrapper>,
    _: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    match msg {
        ExecuteMsg::Deposit {receiver1, receiver2} => deposit(deps, info, receiver1, receiver2),
        ExecuteMsg::Withdraw { coins } => withdraw(deps, info, coins ),
        ExecuteMsg::SetOwner { owner } => set_new_owner(deps, owner),
        ExecuteMsg::SetFee { fee } => set_new_fee(deps, fee),
    }
}

fn set_new_owner(deps: DepsMut<SeiQueryWrapper>, owner: String) -> Result<Response, ContractError> {
    set_owner(deps.storage, owner)?;
    Ok(Response::default())
}

fn set_new_fee(deps: DepsMut<SeiQueryWrapper>, fee: Uint128) -> Result<Response, ContractError> {
    set_fee(deps.storage, fee)?;
    Ok(Response::default())
}

fn deposit(deps: DepsMut<SeiQueryWrapper>, info: MessageInfo, receiver1: String, receiver2: String) -> Result<Response, ContractError> {
    for coin in info.funds {
        let mut receiver1_bal = get_balance(deps.storage, receiver1.to_owned(), coin.denom.to_owned());
        let mut receiver2_bal = get_balance(deps.storage, receiver2.to_owned(), coin.denom.to_owned());
        let owner = get_owner(deps.storage)?;
        let mut owner_bal = get_balance(deps.storage, owner.account.to_owned(), coin.denom.to_owned());
        let fee= get_fee(deps.storage).unwrap();
        let total_bal = coin.amount * (Uint128::from(100u128) - fee.fee) / Uint128::from(100u128);
        let owner_fee = coin.amount *fee.fee / Uint128::from(100u128);
        receiver1_bal.amount += Decimal::from_atomics(total_bal / Uint128::from(2u128), 0).unwrap();
        receiver2_bal.amount += Decimal::from_atomics(total_bal- total_bal / Uint128::from(2u128), 0).unwrap();
        owner_bal.amount += Decimal::from_atomics(owner_fee, 0).unwrap();
        save_balance(
            deps.storage,
            receiver1.to_owned(),
            coin.denom.to_owned(),
            &receiver1_bal,
        );
        save_balance(
            deps.storage,
            receiver2.to_owned(),
            coin.denom.to_owned(),
            &receiver2_bal,
        );
        save_balance(
            deps.storage,
            owner.account.to_owned(),
            coin.denom.to_owned(),
            &owner_bal,
        )
    }
    Ok(Response::default())
}

fn withdraw(
    deps: DepsMut<SeiQueryWrapper>,
    info: MessageInfo,
    coins: Vec<Coin>,
) -> Result<Response, ContractError> {
    let account = info.sender.into_string();
    for coin in coins.to_owned() {
        let mut balance = get_balance(deps.storage, account.to_owned(), coin.denom.to_owned());
        let amount = Decimal::from_atomics(coin.amount, 0).unwrap();
        if balance.amount - balance.withheld < amount {
            return Err(ContractError::InsufficientFund());
        }
        balance.amount -= amount;
        save_balance(
            deps.storage,
            account.to_owned(),
            coin.denom.to_owned(),
            &balance,
        )
    }
    let response = Response::new().add_message(BankMsg::Send {
        to_address: account.to_owned(),
        amount: coins,
    });
    Ok(response)
}
