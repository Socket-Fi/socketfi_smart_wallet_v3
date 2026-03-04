use soroban_sdk::{BytesN, Env};

use crate::{data::DataKey, error::ContractError};

pub fn read_installed_version(e: &Env) -> Result<BytesN<32>, ContractError> {
    let key = DataKey::InstalledVersion;
    e.storage()
        .instance()
        .get(&key)
        .ok_or(ContractError::VersionNotFound)
}

pub fn write_installed_version(e: &Env, version: BytesN<32>) {
    let key = DataKey::InstalledVersion;
    e.storage().instance().set(&key, &version);
}
