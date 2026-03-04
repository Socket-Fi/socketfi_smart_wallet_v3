use soroban_sdk::{Address, Bytes, BytesN, Env, Map, String, Symbol, Val, Vec};

use crate::{
    error::ContractError,
    types::{AccessSettings, WebKeyDetails},
};

pub trait WalletTrait {
    fn initialize(
        env: Env,
        username: String,
        passkey: BytesN<77>,
        bls_keys: Vec<BytesN<96>>,
        factory: Address,
        version: BytesN<32>,
    ) -> Result<(), ContractError>;

    fn update_allowance_expiration(
        env: Env,
        expiration_ledger: u32,
        tx_signature: Option<BytesN<192>>,
    ) -> Result<(), ContractError>;

    fn update_default_limit(
        env: Env,
        limit: i128,
        tx_signature: Option<BytesN<192>>,
    ) -> Result<(), ContractError>;

    fn set_external_wallet(
        env: Env,
        external_wallet: Address,
        tx_signature: Option<BytesN<192>>,
    ) -> Result<(), ContractError>;
    fn reset_account(
        env: Env,
        bls_pubkeys: Vec<BytesN<96>>,
        tx_signature: Option<BytesN<192>>,
    ) -> Result<(), ContractError>;
    fn update_factory(
        env: Env,
        factory: Address,
        tx_signature: Option<BytesN<192>>,
    ) -> Result<(), ContractError>;

    fn deposit(e: Env, from: Address, token: Address, amount: i128) -> Result<(), ContractError>;
    fn withdraw(
        env: Env,
        to: Address,
        token: Address,
        amount: i128,
        tx_signature: Option<BytesN<192>>,
    ) -> Result<(), ContractError>;

    fn dapp_invoker(
        env: Env,
        contract_id: Address,
        func: Symbol,
        args: Option<Vec<Val>>,
        auth_vec: Option<Vec<Map<String, Val>>>,
        tx_signature: Option<BytesN<192>>,
    ) -> Result<(), ContractError>;

    ///Add token custom limit
    fn add_limit(
        env: Env,
        token: Address,
        limit: i128,
        tx_signature: Option<BytesN<192>>,
    ) -> Result<(), ContractError>;

    fn approve(
        env: Env,
        token: Address,
        spender: Address,
        amount: i128,
        tx_signature: Option<BytesN<192>>,
    ) -> Result<(), ContractError>;
    fn spend(
        env: Env,
        token: Address,
        spender: Address,
        amount: i128,
        to: Address,
    ) -> Result<(), ContractError>;

    fn get_version(env: Env) -> Result<BytesN<32>, ContractError>;
    fn get_account_parameters(env: Env) -> AccessSettings;
    fn get_passkey(env: Env) -> Result<WebKeyDetails, ContractError>;
    fn get_allowance(env: Env, token: Address, spender: Address) -> i128;
    fn get_nonce(env: Env) -> Option<BytesN<32>>;

    fn get_tx_payload(
        env: Env,
        func: String,
        args: Vec<Bytes>,
    ) -> Result<BytesN<32>, ContractError>;
    fn get_balance(env: Env, token: Address) -> i128;
    fn get_owner(env: Env) -> Result<Address, ContractError>;
    fn get_factory(env: Env) -> Result<Address, ContractError>;
    fn upgrade(
        env: Env,
        new_version: BytesN<32>,
        tx_signature: Option<BytesN<192>>,
    ) -> Result<(), ContractError>;
}
