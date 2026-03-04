# SocketFi Soroban Wallet Contract

This contract implements a smart-wallet-style **Soroban contract account** with:

- **Owner-controlled actions** authorized via a **nonce-bound payload** + optional signature (`tx_signature`)
- **Passkey / web key** storage & retrieval
- **BLS key aggregation** (reset/rotation)
- **Per-token limits** + **approvals / allowances**
- **Dapp invocation** with optional embedded auth rules (`auth_vec`)

---

## Key Idea: Owner Authorization Model

Most privileged functions do **not** rely on `Address::require_auth()` directly.  
Instead they do:

1. Build a canonical **payload** using:
   - function name (`String`)
   - arguments serialized as XDR (`ToXdr`)
   - a **nonce binding** (`compute_tx_nonce`)
2. Enforce owner authorization via:
   - `owner_require_auth(env, payload, tx_signature)`

This design enables:

- account-like UX (sign once off-chain, submit tx on-chain)
- replay protection via nonce
- consistent signing payload across calls

---

## Public API Overview

### 1) Initialization

#### `initialize(env, username, passkey, bls_keys, factory, version) -> Result<(), ContractError>`

One-time setup. Fails with `AlreadyInitialized` if called again.

Stores:

- username
- web passkey
- BLS keys (vector)
- factory address (master contract)
- installed version hash

---

### 2) Wallet Settings (Owner-gated)

#### `update_allowance_expiration(env, ledger_offset, tx_signature)`

Sets allowance expiry offset (must be `> 0`), else `InvalidExpiration`.

#### `set_external_wallet(env, external_wallet, tx_signature)`

Updates the linked external wallet (owner address).

#### `update_default_limit(env, limit, tx_signature)`

Updates default per-tx spending/approval limit. `limit < 0` → `InvalidLimit`.

#### `reset_account(env, new_bls_keys, tx_signature)`

Rotates / replaces aggregated BLS keys.

#### `update_factory(env, factory, tx_signature)`

Updates the factory/master contract address.

---

### 3) Token Flows

#### `deposit(env, from, token, amount)`

- Requires `from.require_auth()`
- Transfers tokens into the wallet contract via `take_token`
- `amount <= 0` → `InvalidAmount`

#### `withdraw(env, to, token, amount, tx_signature)` (Owner-gated)

- `amount <= 0` → `InvalidAmount`
- `amount > read_limit(token)` → `ExceedMaxAllowance`
- Uses `send_token` to transfer out

---

### 4) Approvals / Allowances

#### `add_limit(env, token, limit, tx_signature)` (Owner-gated)

Sets a custom token limit (per token). `limit < 0` → `InvalidLimit`.

#### `approve(env, token, spender, amount, tx_signature)` (Owner-gated)

Writes allowance:

- `amount < 0` → `InvalidAmount`
- `amount > read_limit(token)` → `ExceedMaxAllowance`

#### `spend(env, token, spender, amount, to)`

- Requires `spender.require_auth()`
- Spends from the allowance bucket using `spend_token`
- `amount <= 0` → `InvalidAmount`

#### `get_allowance(env, token, spender) -> i128`

Reads current allowance.

---

### 5) Dapp Invocation

#### `dapp_invoker(env, contract_id, func, args, auth_vec, tx_signature)` (Owner-gated)

Allows wallet to invoke another contract function:

- Optionally processes additional auth rules via `dapp_invoke_auth(auth_vec)`
- Executes: `env.invoke_contract(&contract_id, &func, args.unwrap_or(vec![&env]))`

This is useful for routing interactions through the wallet while applying extra auth constraints.

---

### 6) Read / Utility Methods

#### `get_account_parameters(env) -> AccessSettings`

Returns:

- `max_allowance` (default spend limit)
- `g_account` (optional owner if set)

#### `get_passkey(env) -> Result<WebKeyDetails, ContractError>`

Reads stored web/passkey details.

#### `get_version(env) -> Result<BytesN<32>, ContractError>`

Returns installed contract version hash.

#### `get_nonce(env) -> Option<BytesN<32>>`

Returns current nonce used for payload binding.

#### `get_tx_payload(env, func, args) -> Result<BytesN<32>, ContractError>`

Helper to compute the exact payload that must be authorized.

#### `get_balance(env, token) -> i128`

Reads the wallet’s token balance.

#### `get_owner(env) -> Result<Address, ContractError>`

Returns owner external wallet address.

#### `get_factory(env) -> Result<Address, ContractError>`

Returns factory/master address.

---

### 7) Upgrade

#### `upgrade(env, new_version, tx_signature)` (Owner-gated)

- Reads installed version
- If already equal → `AlreadyLatest`
- Calls `env.deployer().update_current_contract_wasm(new_version)`
- Persists new version via `write_installed_version`

---

## Error Behaviors (Common)

- `AlreadyInitialized`: initializing twice
- `InvalidAmount`: non-positive amount where positive is required
- `InvalidLimit`: negative limit
- `InvalidExpiration`: expiration offset is zero
- `ExceedMaxAllowance`: amount exceeds token limit
- `AlreadyLatest`: upgrade target equals installed version

---

## Payload Binding (Signing) Notes

For owner-gated calls, the payload is built like:

- `args: Vec<Bytes> = vec![&env, <each_arg>.to_xdr(&env), ...]`
- `payload = compute_tx_nonce(&env, "<function_name>", args)?`
- `owner_require_auth(env, payload, tx_signature)?`

If you’re building a client:

1. call `get_tx_payload(func, args_as_bytes)` (or mirror the same rules off-chain)
2. sign the returned `BytesN<32>`
3. submit the transaction with `tx_signature`

> **Important:** the payload depends on XDR encoding; clients must match encoding exactly.

---

## Quick Function Map

Owner-gated:

- update_allowance_expiration
- set_external_wallet
- update_default_limit
- reset_account
- update_factory
- withdraw
- dapp_invoker
- add_limit
- approve
- upgrade

Require caller auth:

- deposit (from)
- spend (spender)

Read-only:

- get_account_parameters, get_passkey, get_version, get_allowance, get_nonce,
  get_tx_payload, get_balance, get_owner, get_factory

---
