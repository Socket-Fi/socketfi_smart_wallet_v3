use soroban_sdk::{
    bytesn,
    crypto::bls12_381::{G1Affine, G2Affine},
    vec,
    xdr::ToXdr,
    Bytes, BytesN, Env, String, Vec,
};

use crate::{
    access::{read_aggregated_bls_key, read_owner},
    data::DataKey,
    error::ContractError,
};

pub const DST: &str = "BLS_AUTH_XMD:SHA-256_SSWU_SOCKETFI";

pub fn read_dst_bytes(env: &Env) -> Bytes {
    Bytes::from_slice(&env, DST.as_bytes())
}

pub fn read_nonce(env: &Env) -> Option<BytesN<32>> {
    env.storage()
        .instance()
        .get::<DataKey, BytesN<32>>(&DataKey::Nonce)
}
pub fn write_nonce(env: &Env) {
    let mut seed = [0u8; 32];
    env.prng().fill(&mut seed);
    env.storage()
        .instance()
        .set(&DataKey::Nonce, &BytesN::from_array(&env, &seed));
}

pub fn compute_tx_nonce(
    env: &Env,
    func: String,
    args: Vec<Bytes>,
) -> Result<BytesN<32>, ContractError> {
    if let Some(wallet_nonce) = read_nonce(env) {
        let mut payload = wallet_nonce.to_xdr(env);

        payload.append(&env.current_contract_address().to_xdr(env));
        payload.append(&func.to_xdr(env));

        for b in args.iter() {
            let x = b;
            payload.append(&x);
        }

        Ok(BytesN::from(env.crypto().sha256(&payload)))
    } else {
        return Err(ContractError::InvalidNonce);
    }
}

pub fn check_auth(
    env: &Env,
    payload: BytesN<32>,
    tx_signature: BytesN<192>,
) -> Result<(), ContractError> {
    // The sdk module containing access to the bls12_381 functions
    let bls = env.crypto().bls12_381();

    // Retrieve the aggregated pubkey and the DST from storage
    let agg_pk: BytesN<96> = read_aggregated_bls_key(&env)?;
    let dst: Bytes = read_dst_bytes(&env);

    // This is the negative of g1 (generator point of the G1 group)

    let neg_g1 = G1Affine::from_bytes(bytesn!(&env, 0x17f1d3a73197d7942695638c4fa9ac0fc3688c4f9774b905a14e3a3f171bac586c55e83ff97a1aeffb3af00adb22c6bb114d1d6855d545a8aa7d76c8cf2e21f267816aef1db507c96655b9d5caac42364e6f38ba0ecb751bad54dcd6b939c2ca));
    // Hash the signature_payload i.e. the msg being signed and to be
    // verified into a point in G2

    let msg_g2 = bls.hash_to_g2(&payload.into(), &dst);

    // Prepare inputs to the pairing function
    let vp1 = vec![&env, G1Affine::from_bytes(agg_pk), neg_g1];
    let vp2 = vec![&env, msg_g2, G2Affine::from_bytes(tx_signature)];

    // thus it must equal to the RHS if the signature matches.

    if !bls.pairing_check(vp1, vp2) {
        return Err(ContractError::InvalidSignature);
    }
    write_nonce(env);
    Ok(())
}

pub fn owner_require_auth(
    env: Env,
    payload: BytesN<32>,
    tx_signature: Option<BytesN<192>>,
) -> Result<(), ContractError> {
    if let Some(signature) = tx_signature {
        check_auth(&env, payload, signature)?;
    } else {
        let owner = read_owner(&env)?;
        owner.require_auth();
    }

    Ok(())
}
