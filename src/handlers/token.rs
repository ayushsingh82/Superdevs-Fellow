use axum::{Json, http::StatusCode};
use serde::{Deserialize, Serialize};
use spl_token::instruction as token_instruction;
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
pub struct CreateTokenRequest {
    #[serde(rename = "mintAuthority")]
    pub mint_authority: String,
    pub mint: String,
    pub decimals: u8,
}

#[derive(Serialize)]
pub struct CreateTokenResponse {
    pub program_id: String,
    pub accounts: Vec<AccountMetaResponse>,
    pub instruction_data: String,
}

#[derive(Deserialize)]
pub struct MintTokenRequest {
    pub mint: String,
    pub destination: String,
    pub authority: String,
    pub amount: u64,
}

#[derive(Serialize)]
pub struct MintTokenResponse {
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

pub async fn create_token(
    Json(payload): Json<CreateTokenRequest>,
) -> Result<Json<SuccessResponse<CreateTokenResponse>>, (StatusCode, Json<ErrorResponse>)> {
    let mint_authority = validate_pubkey(&payload.mint_authority)
        .map_err(|e| (
            StatusCode::BAD_REQUEST,
            Json(ErrorResponse {
                success: false,
                error: format!("Invalid mint authority: {}", e),
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

    if payload.decimals > 9 {
        return Err((
            StatusCode::BAD_REQUEST,
            Json(ErrorResponse {
                success: false,
                error: "Decimals must be between 0 and 9".to_string(),
            }),
        ));
    }

    let instruction = token_instruction::initialize_mint(
        &spl_token::id(),
        &mint,
        &mint_authority,
        Some(&mint_authority),
        payload.decimals,
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
        data: CreateTokenResponse {
            program_id: instruction.program_id.to_string(),
            accounts,
            instruction_data: encode_base64(&instruction.data),
        },
    }))
}

pub async fn mint_token(
    Json(payload): Json<MintTokenRequest>,
) -> Result<Json<SuccessResponse<MintTokenResponse>>, (StatusCode, Json<ErrorResponse>)> {
    let mint = validate_pubkey(&payload.mint)
        .map_err(|e| (
            StatusCode::BAD_REQUEST,
            Json(ErrorResponse {
                success: false,
                error: format!("Invalid mint: {}", e),
            }),
        ))?;

    let destination = validate_pubkey(&payload.destination)
        .map_err(|e| (
            StatusCode::BAD_REQUEST,
            Json(ErrorResponse {
                success: false,
                error: format!("Invalid destination: {}", e),
            }),
        ))?;

    let authority = validate_pubkey(&payload.authority)
        .map_err(|e| (
            StatusCode::BAD_REQUEST,
            Json(ErrorResponse {
                success: false,
                error: format!("Invalid authority: {}", e),
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

    let instruction = token_instruction::mint_to(
        &spl_token::id(),
        &mint,
        &destination,
        &authority,
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
        data: MintTokenResponse {
            program_id: instruction.program_id.to_string(),
            accounts,
            instruction_data: encode_base64(&instruction.data),
        },
    }))
} 