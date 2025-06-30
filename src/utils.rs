use bs58;
use base64;
use solana_sdk::pubkey::Pubkey;
use std::str::FromStr;

pub fn decode_base58(s: &str) -> Result<Vec<u8>, String> {
    bs58::decode(s).into_vec().map_err(|e| e.to_string())
}

pub fn encode_base58(bytes: &[u8]) -> String {
    bs58::encode(bytes).into_string()
}

pub fn encode_base64(bytes: &[u8]) -> String {
    base64::encode(bytes)
}

pub fn decode_base64(s: &str) -> Result<Vec<u8>, String> {
    base64::decode(s).map_err(|e| e.to_string())
}

pub fn validate_pubkey(pubkey: &str) -> Result<Pubkey, String> {
    Pubkey::from_str(pubkey).map_err(|e| format!("Invalid pubkey: {}", e))
}

pub fn validate_private_key(secret: &str) -> Result<[u8; 64], String> {
    let decoded = decode_base58(secret)?;
    if decoded.len() != 64 {
        return Err("Private key must be 64 bytes".to_string());
    }
    let mut key = [0u8; 64];
    key.copy_from_slice(&decoded);
    Ok(key)
} 