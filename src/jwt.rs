use hmac_sha256::HMAC;
use std::str;
use serde_json::{Value};

extern crate base64;

fn to_base64(v: &[u8]) -> String {
    return base64::encode(v).replace("=", "");
}

fn encoding(s: String) -> String {
    return to_base64(s.as_bytes());
}

fn signature(input: String, secret: String) -> String {
    let v = HMAC::mac(input.as_bytes(), secret.as_bytes());
    return to_base64(&v);
}

pub fn signin(header: Value, payload: Value, secret: String) -> String {
    let h_str = encoding(header.to_string());
    let p_str = encoding(payload.to_string());
    let i = format!("{}.{}", h_str, p_str);
    return format!("{}.{}", i, signature(i.to_string(), secret));
}

pub mod header {
    use serde_json::{Value, json};
    pub fn default() -> Value {
        let v: Value = json!({
            "typ": "JWT",
            "alg": "HS256"
        });
        return v;
    }
}
