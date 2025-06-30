use axum::{Json, http::StatusCode};
use serde::Serialize;
use solana_sdk::signature::{Keypair, Signer};
use crate::utils::{encode_base58};

#[derive(Serialize)]
pub struct SuccessResponse<T> {
    pub success: bool,
    pub data: T,
}

#[derive(Serialize)]
pub struct ErrorResponse {
    pub success: bool,
    pub error: String,
}

#[derive(Serialize)]
pub struct KeypairResponse {
    pub pubkey: String,
    pub secret: String,
}

pub async fn generate_keypair() -> Result<Json<SuccessResponse<KeypairResponse>>, (StatusCode, Json<ErrorResponse>)> {
    let keypair = Keypair::new();
    let pubkey = keypair.pubkey().to_string();
    let secret = encode_base58(&keypair.to_bytes());
    
    Ok(Json(SuccessResponse {
        success: true,
        data: KeypairResponse {
            pubkey,
            secret,
        },
    }))
} 