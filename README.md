# Solana HTTP Server

A Rust-based HTTP server that provides Solana-related endpoints for keypair generation, SPL token operations, message signing/verification, and transaction instruction creation.

## Features

- Generate Solana keypairs
- Create SPL token mint instructions
- Mint SPL tokens
- Sign and verify messages using Ed25519
- Create SOL transfer instructions
- Create SPL token transfer instructions

## Prerequisites

- Rust (latest stable version)
- Cargo

## Installation

1. Clone or download this project
2. Navigate to the project directory
3. Run the following command to build and start the server:

```bash
cargo run
```

The server will start on `http://127.0.0.1:3000`

## API Endpoints

### 1. Generate Keypair
**POST** `/keypair`

Generates a new Solana keypair.

**Response:**
```json
{
  "success": true,
  "data": {
    "pubkey": "base58-encoded-public-key",
    "secret": "base58-encoded-secret-key"
  }
}
```

### 2. Create Token
**POST** `/token/create`

Creates a new SPL token mint initialization instruction.

**Request:**
```json
{
  "mint_authority": "base58-encoded-public-key",
  "mint": "base58-encoded-public-key",
  "decimals": 6
}
```

**Response:**
```json
{
  "success": true,
  "data": {
    "program_id": "string",
    "accounts": [
      {
        "pubkey": "pubkey",
        "is_signer": false,
        "is_writable": true
      }
    ],
    "instruction_data": "base64-encoded-data"
  }
}
```

### 3. Mint Token
**POST** `/token/mint`

Creates a mint-to instruction for SPL tokens.

**Request:**
```json
{
  "mint": "mint-address",
  "destination": "destination-user-address",
  "authority": "authority-address",
  "amount": 1000000
}
```

**Response:**
```json
{
  "success": true,
  "data": {
    "program_id": "string",
    "accounts": [
      {
        "pubkey": "pubkey",
        "is_signer": false,
        "is_writable": true
      }
    ],
    "instruction_data": "base64-encoded-data"
  }
}
```

### 4. Sign Message
**POST** `/message/sign`

Signs a message using a private key.

**Request:**
```json
{
  "message": "Hello, Solana!",
  "secret": "base58-encoded-secret-key"
}
```

**Response:**
```json
{
  "success": true,
  "data": {
    "signature": "base64-encoded-signature",
    "public_key": "base58-encoded-public-key",
    "message": "Hello, Solana!"
  }
}
```

### 5. Verify Message
**POST** `/message/verify`

Verifies a signed message.

**Request:**
```json
{
  "message": "Hello, Solana!",
  "signature": "base64-encoded-signature",
  "pubkey": "base58-encoded-public-key"
}
```

**Response:**
```json
{
  "success": true,
  "data": {
    "valid": true,
    "message": "Hello, Solana!",
    "pubkey": "base58-encoded-public-key"
  }
}
```

### 6. Send SOL
**POST** `/send/sol`

Creates a SOL transfer instruction.

**Request:**
```json
{
  "from": "sender-address",
  "to": "recipient-address",
  "lamports": 100000
}
```

**Response:**
```json
{
  "success": true,
  "data": {
    "program_id": "respective program id",
    "accounts": [
      "address of first account",
      "address of second account"
    ],
    "instruction_data": "instruction_data"
  }
}
```

### 7. Send Token
**POST** `/send/token`

Creates an SPL token transfer instruction.

**Request:**
```json
{
  "destination": "destination-user-address",
  "mint": "mint-address",
  "owner": "owner-address",
  "amount": 100000
}
```

**Response:**
```json
{
  "success": true,
  "data": {
    "program_id": "respective program id",
    "accounts": [
      {
        "pubkey": "pubkey",
        "is_signer": false,
        "is_writable": true
      }
    ],
    "instruction_data": "instruction_data"
  }
}
```

## Error Handling

All endpoints return consistent error responses:

```json
{
  "success": false,
  "error": "Description of error"
}
```

Common error scenarios:
- Invalid public/private keys
- Missing required fields
- Invalid amounts (must be > 0)
- Invalid signatures
- Malformed input data

## Security Considerations

- No private keys are stored on the server
- All cryptographic operations use standard libraries
- Input validation is performed on all endpoints
- Proper error handling to avoid information leakage

## Testing

You can test the endpoints using curl or any HTTP client. Here's an example:

```bash
# Generate a keypair
curl -X POST http://127.0.0.1:3000/keypair

# Sign a message
curl -X POST http://127.0.0.1:3000/message/sign \
  -H "Content-Type: application/json" \
  -d '{"message": "Hello, Solana!", "secret": "your-secret-key"}'
```

## Dependencies

- `axum`: HTTP web framework
- `tokio`: Async runtime
- `solana-sdk`: Solana SDK for blockchain operations
- `spl-token`: SPL token program support
- `ed25519-dalek`: Ed25519 signature operations
- `bs58`: Base58 encoding/decoding
- `base64`: Base64 encoding/decoding
- `serde`: Serialization/deserialization
- `tower-http`: HTTP middleware (CORS support) 