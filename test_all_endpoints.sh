#!/bin/bash

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Base URL
BASE_URL="http://127.0.0.1:3000"

echo -e "${BLUE}🚀 Solana Fellowship API - Complete Endpoint Test${NC}"
echo -e "${BLUE}================================================${NC}"
echo ""

# Function to make API calls and format output
test_endpoint() {
    local endpoint=$1
    local method=$2
    local data=$3
    local description=$4
    
    echo -e "${YELLOW}Testing: $description${NC}"
    echo -e "${YELLOW}Endpoint: $method $BASE_URL$endpoint${NC}"
    
    if [ -n "$data" ]; then
        echo -e "${YELLOW}Request Data: $data${NC}"
        response=$(curl -s -X $method "$BASE_URL$endpoint" \
            -H "Content-Type: application/json" \
            -d "$data")
    else
        response=$(curl -s -X $method "$BASE_URL$endpoint")
    fi
    
    if [ $? -eq 0 ]; then
        echo -e "${GREEN}✅ Success${NC}"
        echo -e "${GREEN}Response: $response${NC}"
    else
        echo -e "${RED}❌ Failed${NC}"
        echo -e "${RED}Error: $response${NC}"
    fi
    echo ""
}

# Test 1: Generate Keypair
echo -e "${BLUE}1️⃣  Testing Generate Keypair${NC}"
echo "----------------------------------------"
keypair_response=$(curl -s -X POST "$BASE_URL/keypair")
if [ $? -eq 0 ]; then
    echo -e "${GREEN}✅ Keypair Generated Successfully${NC}"
    echo -e "${GREEN}Response: $keypair_response${NC}"
    
    # Extract pubkey and secret for use in other tests
    pubkey=$(echo $keypair_response | grep -o '"pubkey":"[^"]*"' | cut -d'"' -f4)
    secret=$(echo $keypair_response | grep -o '"secret":"[^"]*"' | cut -d'"' -f4)
    echo -e "${BLUE}📋 Extracted pubkey: $pubkey${NC}"
    echo -e "${BLUE}📋 Extracted secret: ${secret:0:20}...${NC}"
else
    echo -e "${RED}❌ Failed to generate keypair${NC}"
    echo -e "${RED}Error: $keypair_response${NC}"
    exit 1
fi
echo ""

# Test 2: Create Token
echo -e "${BLUE}2️⃣  Testing Create Token${NC}"
echo "----------------------------------------"
create_token_data="{\"mint_authority\":\"$pubkey\",\"mint\":\"$pubkey\",\"decimals\":6}"
test_endpoint "/token/create" "POST" "$create_token_data" "Create SPL Token Mint Instruction"
echo ""

# Test 3: Mint Token
echo -e "${BLUE}3️⃣  Testing Mint Token${NC}"
echo "----------------------------------------"
mint_token_data="{\"mint\":\"$pubkey\",\"destination\":\"$pubkey\",\"authority\":\"$pubkey\",\"amount\":1000000}"
test_endpoint "/token/mint" "POST" "$mint_token_data" "Create Mint-to Instruction"
echo ""

# Test 4: Sign Message
echo -e "${BLUE}4️⃣  Testing Sign Message${NC}"
echo "----------------------------------------"
sign_message_data="{\"message\":\"Hello, Solana!\",\"secret\":\"$secret\"}"
sign_response=$(curl -s -X POST "$BASE_URL/message/sign" \
    -H "Content-Type: application/json" \
    -d "$sign_message_data")

if [ $? -eq 0 ]; then
    echo -e "${GREEN}✅ Message Signed Successfully${NC}"
    echo -e "${GREEN}Response: $sign_response${NC}"
    
    # Extract signature and public key for verification
    signature=$(echo $sign_response | grep -o '"signature":"[^"]*"' | cut -d'"' -f4)
    signed_pubkey=$(echo $sign_response | grep -o '"public_key":"[^"]*"' | cut -d'"' -f4)
    echo -e "${BLUE}📋 Extracted signature: ${signature:0:20}...${NC}"
    echo -e "${BLUE}📋 Extracted public key: $signed_pubkey${NC}"
else
    echo -e "${RED}❌ Failed to sign message${NC}"
    echo -e "${RED}Error: $sign_response${NC}"
fi
echo ""

# Test 5: Verify Message
echo -e "${BLUE}5️⃣  Testing Verify Message${NC}"
echo "----------------------------------------"
verify_message_data="{\"message\":\"Hello, Solana!\",\"signature\":\"$signature\",\"pubkey\":\"$signed_pubkey\"}"
test_endpoint "/message/verify" "POST" "$verify_message_data" "Verify Signed Message"
echo ""

# Test 6: Send SOL
echo -e "${BLUE}6️⃣  Testing Send SOL${NC}"
echo "----------------------------------------"
send_sol_data="{\"from\":\"$pubkey\",\"to\":\"$pubkey\",\"lamports\":100000}"
test_endpoint "/send/sol" "POST" "$send_sol_data" "Create SOL Transfer Instruction"
echo ""

# Test 7: Send Token
echo -e "${BLUE}7️⃣  Testing Send Token${NC}"
echo "----------------------------------------"
send_token_data="{\"destination\":\"$pubkey\",\"mint\":\"$pubkey\",\"owner\":\"$pubkey\",\"amount\":100000}"
test_endpoint "/send/token" "POST" "$send_token_data" "Create SPL Token Transfer Instruction"
echo ""

# Test Health Check
echo -e "${BLUE}🏥 Testing Health Check${NC}"
echo "----------------------------------------"
test_endpoint "/health" "GET" "" "Health Check"
echo ""

echo -e "${GREEN}🎉 All endpoint tests completed!${NC}"
echo -e "${BLUE}================================================${NC}"
echo -e "${BLUE}Summary:${NC}"
echo -e "${BLUE}- 7 main endpoints tested${NC}"
echo -e "${BLUE}- 1 health check endpoint tested${NC}"
echo -e "${BLUE}- All responses should show 'success: true'${NC}"
echo ""
echo -e "${YELLOW}💡 Tips:${NC}"
echo -e "${YELLOW}- Make sure your server is running with: cargo run${NC}"
echo -e "${YELLOW}- Check the responses above for any errors${NC}"
echo -e "${YELLOW}- All endpoints should return JSON with 'success' field${NC}" 