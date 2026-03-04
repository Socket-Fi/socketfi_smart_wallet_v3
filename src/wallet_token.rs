use soroban_sdk::{token, Address, Env};

use crate::{data::DataKey, error::ContractError};

pub fn take_token(env: &Env, from: &Address, token: &Address, amount: i128) {
    let token_client = token::Client::new(env, token);
    let contract_address = env.current_contract_address();
    token_client.transfer(from, &contract_address, &amount);
}

pub fn send_token(env: &Env, to: &Address, token: &Address, amount: i128) {
    let token_client = token::Client::new(env, token);
    let contract_address = env.current_contract_address();
    token_client.transfer(&contract_address, to, &amount);
}

pub fn spend_token(env: &Env, spender: &Address, token: &Address, amount: i128, to: &Address) {
    let token_client = token::Client::new(env, token);
    let contract_address = env.current_contract_address();
    token_client.transfer_from(&spender, &contract_address, to, &amount);
}

pub fn read_balance(env: &Env, token: &Address) -> i128 {
    let token_client = token::Client::new(env, token);
    let contract_address = env.current_contract_address();
    token_client.balance(&contract_address)
}

pub fn read_allowance(env: &Env, token: &Address, spender: &Address) -> i128 {
    let token_client = token::Client::new(env, token);
    let contract_address = env.current_contract_address();
    token_client.allowance(&contract_address, spender)
}

pub fn write_approve(
    env: &Env,
    token: &Address,
    spender: &Address,
    amount: &i128,
) -> Result<(), ContractError> {
    let token_client = token::Client::new(env, token);
    let contract_address = env.current_contract_address();

    let expiration = read_allowance_expiration(env)
        .checked_add(env.ledger().sequence())
        .ok_or(ContractError::InvalidExpiration)?;

    token_client.approve(&contract_address, spender, amount, &expiration);
    Ok(())
}

pub fn write_allowance_expiration(env: &Env, ledger_offset: u32) {
    env.storage()
        .persistent()
        .set(&DataKey::AllowanceExpiration, &ledger_offset);
}

pub fn read_allowance_expiration(env: &Env) -> u32 {
    env.storage()
        .persistent()
        .get(&DataKey::AllowanceExpiration)
        .unwrap_or(17_000)
}

pub fn write_default_spend_limit(env: &Env, limit: i128) {
    env.storage()
        .instance()
        .set(&DataKey::DefaultSpendLimit, &limit);
}

pub fn read_default_spend_limit(env: &Env) -> i128 {
    env.storage()
        .instance()
        .get(&DataKey::DefaultSpendLimit)
        .unwrap_or(0)
}

pub fn read_limit(env: &Env, token: Address) -> i128 {
    env.storage()
        .instance()
        .get(&DataKey::SpendLimit(token))
        .unwrap_or(read_default_spend_limit(&env))
}

pub fn write_limit(env: &Env, token: Address, limit: i128) {
    env.storage()
        .instance()
        .set(&DataKey::SpendLimit(token), &limit);
}
