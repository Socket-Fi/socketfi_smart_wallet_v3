use soroban_sdk::{Address, BytesN, Env, String, Vec};

use crate::{
    access::{write_agg_bls_key, write_factory, write_web_keys},
    error::ContractError,
    version::write_installed_version,
    wallet_bls_auth::write_nonce,
    wallet_token::{write_allowance_expiration, write_default_spend_limit},
};

pub fn init_constructor(
    env: Env,
    username: String,
    passkey: BytesN<77>,
    bls_keys: Vec<BytesN<96>>,
    factory: Address,
    version: BytesN<32>,
) -> Result<(), ContractError> {
    write_agg_bls_key(&env, bls_keys)?;
    let _ = write_web_keys(&env, username, passkey);
    write_factory(&env, &factory);
    write_default_spend_limit(&env, 1_000_000_000);
    write_allowance_expiration(&env, 17_000);
    write_nonce(&env);
    write_installed_version(&env, version);
    Ok(())
}
