#!/bin/bash

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Base URL
BASE_URL="http://127.0.0.1:3000"

echo -e "${BLUE}🚀 Solana Fellowship API - Complete Test${NC}"
echo -e "${BLUE}=====================================${NC}"
echo ""

# Test 1: Generate Keypair
echo -e "${YELLOW}1️⃣  Testing Generate Keypair${NC}"
echo "----------------------------------------"
keypair_response=$(curl -s -X POST "$BASE_URL/keypair")
if [ $? -eq 0 ]; then
    echo -e "${GREEN}✅ Success${NC}"
    echo "$keypair_response" | python3 -m json.tool 2>/dev/null || echo "$keypair_response"
    
    # Extract pubkey and secret
    pubkey=$(echo "$keypair_response" | grep -o '"pubkey":"[^"]*"' | cut -d'"' -f4)
    secret=$(echo "$keypair_response" | grep -o '"secret":"[^"]*"' | cut -d'"' -f4)
    echo -e "${BLUE}📋 Pubkey: $pubkey${NC}"
    echo -e "${BLUE}📋 Secret: ${secret:0:20}...${NC}"
else
    echo -e "${RED}❌ Failed${NC}"
    echo "$keypair_response"
    exit 1
fi
echo ""

# Test 2: Create Token (using camelCase field names)
echo -e "${YELLOW}2️⃣  Testing Create Token${NC}"
echo "----------------------------------------"
create_token_data="{\"mintAuthority\":\"$pubkey\",\"mint\":\"$pubkey\",\"decimals\":6}"
response=$(curl -s -X POST "$BASE_URL/token/create" -H "Content-Type: application/json" -d "$create_token_data")
if [ $? -eq 0 ]; then
    echo -e "${GREEN}✅ Success${NC}"
    echo "$response" | python3 -m json.tool 2>/dev/null || echo "$response"
else
    echo -e "${RED}❌ Failed${NC}"
    echo "$response"
fi
echo ""

# Test 3: Mint Token
echo -e "${YELLOW}3️⃣  Testing Mint Token${NC}"
echo "----------------------------------------"
mint_token_data="{\"mint\":\"$pubkey\",\"destination\":\"$pubkey\",\"authority\":\"$pubkey\",\"amount\":1000000}"
response=$(curl -s -X POST "$BASE_URL/token/mint" -H "Content-Type: application/json" -d "$mint_token_data")
if [ $? -eq 0 ]; then
    echo -e "${GREEN}✅ Success${NC}"
    echo "$response" | python3 -m json.tool 2>/dev/null || echo "$response"
else
    echo -e "${RED}❌ Failed${NC}"
    echo "$response"
fi
echo ""

# Test 4: Sign Message
echo -e "${YELLOW}4️⃣  Testing Sign Message${NC}"
echo "----------------------------------------"
sign_message_data="{\"message\":\"Hello, Solana!\",\"secret\":\"$secret\"}"
sign_response=$(curl -s -X POST "$BASE_URL/message/sign" -H "Content-Type: application/json" -d "$sign_message_data")
if [ $? -eq 0 ]; then
    echo -e "${GREEN}✅ Success${NC}"
    echo "$sign_response" | python3 -m json.tool 2>/dev/null || echo "$sign_response"
    
    # Extract signature and public key
    signature=$(echo "$sign_response" | grep -o '"signature":"[^"]*"' | cut -d'"' -f4)
    signed_pubkey=$(echo "$sign_response" | grep -o '"public_key":"[^"]*"' | cut -d'"' -f4)
    echo -e "${BLUE}📋 Signature: ${signature:0:20}...${NC}"
    echo -e "${BLUE}📋 Public Key: $signed_pubkey${NC}"
else
    echo -e "${RED}❌ Failed${NC}"
    echo "$sign_response"
    exit 1
fi
echo ""

# Test 5: Verify Message
echo -e "${YELLOW}5️⃣  Testing Verify Message${NC}"
echo "----------------------------------------"
verify_message_data="{\"message\":\"Hello, Solana!\",\"signature\":\"$signature\",\"pubkey\":\"$signed_pubkey\"}"
response=$(curl -s -X POST "$BASE_URL/message/verify" -H "Content-Type: application/json" -d "$verify_message_data")
if [ $? -eq 0 ]; then
    echo -e "${GREEN}✅ Success${NC}"
    echo "$response" | python3 -m json.tool 2>/dev/null || echo "$response"
else
    echo -e "${RED}❌ Failed${NC}"
    echo "$response"
fi
echo ""

# Test 6: Send SOL (with different addresses for validation)
echo -e "${YELLOW}6️⃣  Testing Send SOL${NC}"
echo "----------------------------------------"
# Generate a second keypair for the "to" address
second_keypair=$(curl -s -X POST "$BASE_URL/keypair")
second_pubkey=$(echo "$second_keypair" | grep -o '"pubkey":"[^"]*"' | cut -d'"' -f4)

send_sol_data="{\"from\":\"$pubkey\",\"to\":\"$second_pubkey\",\"lamports\":100000}"
response=$(curl -s -X POST "$BASE_URL/send/sol" -H "Content-Type: application/json" -d "$send_sol_data")
if [ $? -eq 0 ]; then
    echo -e "${GREEN}✅ Success${NC}"
    echo "$response" | python3 -m json.tool 2>/dev/null || echo "$response"
else
    echo -e "${RED}❌ Failed${NC}"
    echo "$response"
fi
echo ""

# Test 7: Send Token
echo -e "${YELLOW}7️⃣  Testing Send Token${NC}"
echo "----------------------------------------"
send_token_data="{\"destination\":\"$second_pubkey\",\"mint\":\"$pubkey\",\"owner\":\"$pubkey\",\"amount\":100000}"
response=$(curl -s -X POST "$BASE_URL/send/token" -H "Content-Type: application/json" -d "$send_token_data")
if [ $? -eq 0 ]; then
    echo -e "${GREEN}✅ Success${NC}"
    echo "$response" | python3 -m json.tool 2>/dev/null || echo "$response"
else
    echo -e "${RED}❌ Failed${NC}"
    echo "$response"
fi
echo ""

# Test 8: Health Check
echo -e "${YELLOW}🏥 Testing Health Check${NC}"
echo "----------------------------------------"
response=$(curl -s -X GET "$BASE_URL/health")
if [ $? -eq 0 ]; then
    echo -e "${GREEN}✅ Success${NC}"
    echo "$response" | python3 -m json.tool 2>/dev/null || echo "$response"
else
    echo -e "${RED}❌ Failed${NC}"
    echo "$response"
fi
echo ""

echo -e "${GREEN}🎉 All tests completed!${NC}"
echo -e "${BLUE}========================${NC}"
echo -e "${YELLOW}💡 Make sure your server is running with: cargo run${NC}" 