use soroban_sdk::{
    auth::{ContractContext, InvokerContractAuthEntry, SubContractInvocation},
    vec, Address, Env, FromVal, IntoVal, Map, String, Symbol, Val, Vec,
};

use crate::error::ContractError;

pub fn dapp_invoke_auth(e: &Env, auth_vec: Vec<Map<String, Val>>) -> Result<(), ContractError> {
    let len = auth_vec.len();
    let mut auth_entries: Vec<InvokerContractAuthEntry> = Vec::new(&e);

    for i in 0..len {
        let auth_map = auth_vec.get_unchecked(i);

        let args: Vec<Val> = if let Some(val) = auth_map.get(String::from_str(e, "args")) {
            Vec::from_val(e, &val)
        } else {
            Vec::new(e)
        };
        let contract_id: Address = if let Some(val) = auth_map.get(String::from_str(&e, "contract"))
        {
            Address::from_val(e, &val)
        } else {
            return Err(ContractError::InvalidInvokeContract);
        };
        let func: Symbol = if let Some(val) = auth_map.get(String::from_str(&e, "func")) {
            Symbol::from_val(e, &val)
        } else {
            return Err(ContractError::InvalidInvokeFunction);
        };

        let auth_entry = InvokerContractAuthEntry::Contract(SubContractInvocation {
            context: ContractContext {
                contract: contract_id,
                fn_name: func,
                args: args.into_val(e),
            },
            sub_invocations: vec![&e],
        });

        auth_entries.push_back(auth_entry);
    }

    e.authorize_as_current_contract(auth_entries);
    Ok(())
}
