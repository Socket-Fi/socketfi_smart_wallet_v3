use soroban_sdk::{
    crypto::bls12_381::G1Affine, Address, BytesN, ConversionError, Env, String, Vec,
};

use crate::{
    data::DataKey, error::ContractError, formatter::convert_to_lower, types::WebKeyDetails,
};

pub fn is_initialized(e: &Env) -> bool {
    let key = DataKey::AggregatedBlsKey;
    e.storage().persistent().has(&key)
}

pub fn try_read_owner(e: &Env) -> Option<Address> {
    let key = DataKey::Owner;
    e.storage().instance().get(&key)
}

pub fn read_owner(e: &Env) -> Result<Address, ContractError> {
    let key = DataKey::Owner;
    e.storage()
        .instance()
        .get(&key)
        .ok_or(ContractError::OwnerNotSet)
}

pub fn write_owner(e: &Env, owner: &Address) {
    let key = DataKey::Owner;
    e.storage().instance().set(&key, owner);
}

pub fn write_agg_bls_key(env: &Env, bls_keys: Vec<BytesN<96>>) -> Result<(), ContractError> {
    let bls = env.crypto().bls12_381();

    let mut keypair_1_array = [0u8; 96];
    bls_keys
        .get_unchecked(0)
        .copy_into_slice(&mut keypair_1_array);

    let mut agg_pk = G1Affine::from_bytes(BytesN::from_array(env, &keypair_1_array));

    const MAX_BLS_KEYS: u32 = 5;
    let n = bls_keys.len();

    if n > MAX_BLS_KEYS {
        return Err(ContractError::TooManyKeys);
    }

    for i in 1..n {
        let mut keypair_i_array = [0u8; 96];
        bls_keys
            .get_unchecked(i)
            .copy_into_slice(&mut keypair_i_array);

        let pk = G1Affine::from_bytes(BytesN::from_array(env, &keypair_i_array));
        agg_pk = bls.g1_add(&agg_pk, &pk);
    }

    env.storage()
        .persistent()
        .set(&DataKey::AggregatedBlsKey, &agg_pk.to_bytes());

    Ok(())
}

pub fn read_aggregated_bls_key(e: &Env) -> Result<BytesN<96>, ContractError> {
    let key = DataKey::AggregatedBlsKey;
    e.storage()
        .persistent()
        .get(&key)
        .ok_or(ContractError::AggregatedBlsKeyNotFound)
}

pub fn write_web_keys(
    env: &Env,
    username: String,
    passkey: BytesN<77>,
) -> Result<(), ConversionError> {
    let lower_username = convert_to_lower(&env, username)?;
    let web_keys = WebKeyDetails {
        passkey: passkey,
        username: lower_username,
    };
    env.storage().persistent().set(&DataKey::WebKeys, &web_keys);
    Ok(())
}

pub fn read_web_keys(e: &Env) -> Result<WebKeyDetails, ContractError> {
    let key = DataKey::WebKeys;
    e.storage()
        .persistent()
        .get(&key)
        .ok_or(ContractError::WebKeysNotFound)
}

pub fn read_factory(e: &Env) -> Result<Address, ContractError> {
    let key = DataKey::FactoryContract;
    e.storage()
        .instance()
        .get(&key)
        .ok_or(ContractError::MasterContractNotFound)
}

pub fn write_factory(e: &Env, factory: &Address) {
    let key = DataKey::FactoryContract;
    e.storage().instance().set(&key, factory);
}
