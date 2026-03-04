use soroban_sdk::{
    xdr::{FromXdr, ToXdr},
    Bytes, ConversionError, Env, String,
};

pub fn to_lower_bytes(e: &Env, string: String) -> Bytes {
    let string_xdr = string.clone().to_xdr(e);

    let mut formatted_string_xdr = string_xdr.clone();

    for i in 0..string_xdr.len() {
        let ascii_val = string_xdr.get_unchecked(i);
        if ascii_val >= 65 && ascii_val <= 90 {
            formatted_string_xdr.set(i, ascii_val.saturating_add(32));
        }
    }

    formatted_string_xdr
}

pub fn convert_to_lower(e: &Env, string: String) -> Result<String, ConversionError> {
    String::from_xdr(e, &to_lower_bytes(e, string))
}
