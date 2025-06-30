use axum::{Json, http::StatusCode};
use serde::{Deserialize, Serialize};
use ed25519_dalek::{Keypair as Ed25519Keypair, PublicKey, SecretKey, Signature, Signer, Verifier};
use crate::utils::{encode_base58, encode_base64, decode_base64, decode_base58, validate_private_key};

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

#[derive(Deserialize)]
pub struct SignMessageRequest {
    pub message: String,
    pub secret: String,
}

#[derive(Serialize)]
pub struct SignMessageResponse {
    pub signature: String,
    pub public_key: String,
    pub message: String,
}

#[derive(Deserialize)]
pub struct VerifyMessageRequest {
    pub message: String,
    pub signature: String,
    pub pubkey: String,
}

#[derive(Serialize)]
pub struct VerifyMessageResponse {
    pub valid: bool,
    pub message: String,
    pub pubkey: String,
}

pub async fn sign_message(
    Json(payload): Json<SignMessageRequest>,
) -> Result<Json<SuccessResponse<SignMessageResponse>>, (StatusCode, Json<ErrorResponse>)> {
    if payload.message.is_empty() || payload.secret.is_empty() {
        return Err((
            StatusCode::BAD_REQUEST,
            Json(ErrorResponse {
                success: false,
                error: "Missing required fields".to_string(),
            }),
        ));
    }

    let secret_key_bytes = validate_private_key(&payload.secret)
        .map_err(|e| (
            StatusCode::BAD_REQUEST,
            Json(ErrorResponse {
                success: false,
                error: format!("Invalid private key: {}", e),
            }),
        ))?;

    let secret_key = SecretKey::from_bytes(&secret_key_bytes[..32])
        .map_err(|e| (
            StatusCode::BAD_REQUEST,
            Json(ErrorResponse {
                success: false,
                error: format!("Invalid secret key: {}", e),
            }),
        ))?;

    let public_key = PublicKey::from(&secret_key);
    let keypair = Ed25519Keypair {
        secret: secret_key,
        public: public_key,
    };

    let message_bytes = payload.message.as_bytes();
    let signature = keypair.sign(message_bytes);

    Ok(Json(SuccessResponse {
        success: true,
        data: SignMessageResponse {
            signature: encode_base64(&signature.to_bytes()),
            public_key: encode_base58(&public_key.to_bytes()),
            message: payload.message,
        },
    }))
}

pub async fn verify_message(
    Json(payload): Json<VerifyMessageRequest>,
) -> Result<Json<SuccessResponse<VerifyMessageResponse>>, (StatusCode, Json<ErrorResponse>)> {
    if payload.message.is_empty() || payload.signature.is_empty() || payload.pubkey.is_empty() {
        return Err((
            StatusCode::BAD_REQUEST,
            Json(ErrorResponse {
                success: false,
                error: "Missing required fields".to_string(),
            }),
        ));
    }

    let public_key_bytes = decode_base58(&payload.pubkey)
        .map_err(|e| (
            StatusCode::BAD_REQUEST,
            Json(ErrorResponse {
                success: false,
                error: format!("Invalid public key: {}", e),
            }),
        ))?;

    let public_key = PublicKey::from_bytes(&public_key_bytes)
        .map_err(|e| (
            StatusCode::BAD_REQUEST,
            Json(ErrorResponse {
                success: false,
                error: format!("Invalid public key: {}", e),
            }),
        ))?;

    let signature_bytes = decode_base64(&payload.signature)
        .map_err(|e| (
            StatusCode::BAD_REQUEST,
            Json(ErrorResponse {
                success: false,
                error: format!("Invalid signature: {}", e),
            }),
        ))?;

    let signature = Signature::from_bytes(&signature_bytes)
        .map_err(|e| (
            StatusCode::BAD_REQUEST,
            Json(ErrorResponse {
                success: false,
                error: format!("Invalid signature: {}", e),
            }),
        ))?;

    let message_bytes = payload.message.as_bytes();
    let valid = public_key.verify(message_bytes, &signature).is_ok();

    Ok(Json(SuccessResponse {
        success: true,
        data: VerifyMessageResponse {
            valid,
            message: payload.message,
            pubkey: payload.pubkey,
        },
    }))
} 