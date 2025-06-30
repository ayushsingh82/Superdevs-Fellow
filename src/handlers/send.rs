use axum::{Json, http::StatusCode};
use serde::{Deserialize, Serialize};
use solana_sdk::system_instruction;
use spl_token::instruction as token_instruction;
use spl_associated_token_account::get_associated_token_address;
use crate::utils::{validate_pubkey, encode_base64};

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
pub struct SendSolRequest {
    pub from: String,
    pub to: String,
    pub lamports: u64,
}

#[derive(Serialize)]
pub struct SendSolResponse {
    pub program_id: String,
    pub accounts: Vec<String>,
    pub instruction_data: String,
}

#[derive(Deserialize)]
pub struct SendTokenRequest {
    pub destination: String,
    pub mint: String,
    pub owner: String,
    pub amount: u64,
}

#[derive(Serialize)]
pub struct SendTokenResponse {
    pub program_id: String,
    pub accounts: Vec<AccountMetaResponse>,
    pub instruction_data: String,
}

#[derive(Serialize)]
pub struct AccountMetaResponse {
    pub pubkey: String,
    pub is_signer: bool,
    pub is_writable: bool,
}

pub async fn send_sol(
    Json(payload): Json<SendSolRequest>,
) -> Result<Json<SuccessResponse<SendSolResponse>>, (StatusCode, Json<ErrorResponse>)> {
    let from = validate_pubkey(&payload.from)
        .map_err(|e| (
            StatusCode::BAD_REQUEST,
            Json(ErrorResponse {
                success: false,
                error: format!("Invalid from address: {}", e),
            }),
        ))?;

    let to = validate_pubkey(&payload.to)
        .map_err(|e| (
            StatusCode::BAD_REQUEST,
            Json(ErrorResponse {
                success: false,
                error: format!("Invalid to address: {}", e),
            }),
        ))?;

    if payload.lamports == 0 {
        return Err((
            StatusCode::BAD_REQUEST,
            Json(ErrorResponse {
                success: false,
                error: "Lamports must be greater than 0".to_string(),
            }),
        ));
    }

    if from == to {
        return Err((
            StatusCode::BAD_REQUEST,
            Json(ErrorResponse {
                success: false,
                error: "From and to addresses cannot be the same".to_string(),
            }),
        ));
    }

    if payload.lamports > 1_000_000_000_000_000 {
        return Err((
            StatusCode::BAD_REQUEST,
            Json(ErrorResponse {
                success: false,
                error: "Lamport amount too large".to_string(),
            }),
        ));
    }

    let instruction = system_instruction::transfer(&from, &to, payload.lamports);

    Ok(Json(SuccessResponse {
        success: true,
        data: SendSolResponse {
            program_id: instruction.program_id.to_string(),
            accounts: instruction.accounts.iter().map(|meta| meta.pubkey.to_string()).collect(),
            instruction_data: encode_base64(&instruction.data),
        },
    }))
}

pub async fn send_token(
    Json(payload): Json<SendTokenRequest>,
) -> Result<Json<SuccessResponse<SendTokenResponse>>, (StatusCode, Json<ErrorResponse>)> {
    let destination = validate_pubkey(&payload.destination)
        .map_err(|e| (
            StatusCode::BAD_REQUEST,
            Json(ErrorResponse {
                success: false,
                error: format!("Invalid destination: {}", e),
            }),
        ))?;

    let mint = validate_pubkey(&payload.mint)
        .map_err(|e| (
            StatusCode::BAD_REQUEST,
            Json(ErrorResponse {
                success: false,
                error: format!("Invalid mint: {}", e),
            }),
        ))?;

    let owner = validate_pubkey(&payload.owner)
        .map_err(|e| (
            StatusCode::BAD_REQUEST,
            Json(ErrorResponse {
                success: false,
                error: format!("Invalid owner: {}", e),
            }),
        ))?;

    if payload.amount == 0 {
        return Err((
            StatusCode::BAD_REQUEST,
            Json(ErrorResponse {
                success: false,
                error: "Amount must be greater than 0".to_string(),
            }),
        ));
    }

    let destination_ata = get_associated_token_address(&destination, &mint);
    
    let instruction = token_instruction::transfer(
        &spl_token::id(),
        &destination_ata,
        &destination_ata,
        &owner,
        &[],
        payload.amount,
    ).map_err(|e| (
        StatusCode::INTERNAL_SERVER_ERROR,
        Json(ErrorResponse {
            success: false,
            error: format!("Failed to create instruction: {}", e),
        }),
    ))?;

    let accounts = instruction.accounts.iter().map(|meta| AccountMetaResponse {
        pubkey: meta.pubkey.to_string(),
        is_signer: meta.is_signer,
        is_writable: meta.is_writable,
    }).collect();

    Ok(Json(SuccessResponse {
        success: true,
        data: SendTokenResponse {
            program_id: instruction.program_id.to_string(),
            accounts,
            instruction_data: encode_base64(&instruction.data),
        },
    }))
} 