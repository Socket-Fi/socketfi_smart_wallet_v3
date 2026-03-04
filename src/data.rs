use soroban_sdk::{contracttype, Address};

#[derive(Clone)]
#[contracttype]
pub enum DataKey {
    UserAccountId,
    FactoryContract,
    Owner,
    AggregatedBlsKey,
    SocketUsername,
    WebKeys,
    Allowance(Address, Address),
    AllowanceExpiration,
    SmartAllowance(Address),
    Balance(Address),
    LinkedAccounts,
    MaxAllowanceDefault,
    DefaultSpendLimit,
    SpendLimit(Address),
    Nonce,
    Dst,
    TransactionNonce,
    InstalledVersion,
}
