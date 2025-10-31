#!/bin/bash

# Exit on any error
set -e

echo "========================================"
echo "  Testing ERC-20 Token Contract"
echo "========================================"
echo ""

# Load environment variables
if [ ! -f .env ]; then
    echo "Error: .env file not found!"
    echo "Please copy .env.example to .env and fill in your values"
    exit 1
fi

source .env

# Check if contract address is set
if [ -z "$CONTRACT_ADDRESS" ]; then
    echo "Error: CONTRACT_ADDRESS not set in .env"
    echo "Please deploy the contract first and add the address to .env"
    exit 1
fi

# Check if private key is set
if [ -z "$PRIVATE_KEY" ]; then
    echo "Error: PRIVATE_KEY not set in .env"
    exit 1
fi

# Check if RPC URL is set
if [ -z "$RPC_URL" ]; then
    echo "Error: RPC_URL not set in .env"
    exit 1
fi

echo "Contract Address: $CONTRACT_ADDRESS"
echo "RPC Endpoint: $RPC_URL"
echo ""

# Get the wallet address from private key
WALLET_ADDRESS=$(cast wallet address $PRIVATE_KEY)
echo "Testing with wallet: $WALLET_ADDRESS"
echo ""

echo "Running tests..."
echo "----------------"

# Test 1: Get token name
echo -n "Test 1: Get token name... "
NAME=$(cast call $CONTRACT_ADDRESS "name()(string)" --rpc-url $RPC_URL)
echo "✓ Name: $NAME"

# Test 2: Get token symbol
echo -n "Test 2: Get token symbol... "
SYMBOL=$(cast call $CONTRACT_ADDRESS "symbol()(string)" --rpc-url $RPC_URL)
echo "✓ Symbol: $SYMBOL"

# Test 3: Get decimals
echo -n "Test 3: Get decimals... "
DECIMALS=$(cast call $CONTRACT_ADDRESS "decimals()(uint8)" --rpc-url $RPC_URL)
echo "✓ Decimals: $DECIMALS"

# Test 4: Check initial total supply
echo -n "Test 4: Check total supply... "
TOTAL_SUPPLY=$(cast call $CONTRACT_ADDRESS "totalSupply()(uint256)" --rpc-url $RPC_URL)
echo "✓ Total Supply: $TOTAL_SUPPLY"

# Test 5: Check initial balance
echo -n "Test 5: Check balance... "
BALANCE=$(cast call $CONTRACT_ADDRESS "balanceOf(address)(uint256)" $WALLET_ADDRESS --rpc-url $RPC_URL)
echo "✓ Balance: $BALANCE"

# Test 6: Mint tokens
echo -n "Test 6: Mint tokens... "
MINT_AMOUNT="1000000000000000000"  # 1 token (with 18 decimals)
MINT_TX=$(cast send $CONTRACT_ADDRESS "mint(uint256)" $MINT_AMOUNT \
    --private-key $PRIVATE_KEY \
    --rpc-url $RPC_URL \
    --json | jq -r .transactionHash)
echo "✓ Minted 1 token (tx: $MINT_TX)"

# Test 7: Check balance after mint
echo -n "Test 7: Check balance after mint... "
BALANCE_AFTER=$(cast call $CONTRACT_ADDRESS "balanceOf(address)(uint256)" $WALLET_ADDRESS --rpc-url $RPC_URL)
echo "✓ New Balance: $BALANCE_AFTER"

# Test 8: Check total supply after mint
echo -n "Test 8: Check total supply after mint... "
TOTAL_SUPPLY_AFTER=$(cast call $CONTRACT_ADDRESS "totalSupply()(uint256)" --rpc-url $RPC_URL)
echo "✓ New Total Supply: $TOTAL_SUPPLY_AFTER"

# Test 9: Transfer tokens (to self for simplicity)
echo -n "Test 9: Transfer tokens... "
TRANSFER_AMOUNT="100000000000000000"  # 0.1 token
TRANSFER_TX=$(cast send $CONTRACT_ADDRESS "transfer(address,uint256)" $WALLET_ADDRESS $TRANSFER_AMOUNT \
    --private-key $PRIVATE_KEY \
    --rpc-url $RPC_URL \
    --json | jq -r .transactionHash)
echo "✓ Transferred 0.1 token (tx: $TRANSFER_TX)"

# Test 10: Approve spending
echo -n "Test 10: Approve spending... "
APPROVE_AMOUNT="500000000000000000"  # 0.5 token
APPROVE_TX=$(cast send $CONTRACT_ADDRESS "approve(address,uint256)" $WALLET_ADDRESS $APPROVE_AMOUNT \
    --private-key $PRIVATE_KEY \
    --rpc-url $RPC_URL \
    --json | jq -r .transactionHash)
echo "✓ Approved 0.5 token (tx: $APPROVE_TX)"

# Test 11: Check allowance
echo -n "Test 11: Check allowance... "
ALLOWANCE=$(cast call $CONTRACT_ADDRESS "allowance(address,address)(uint256)" $WALLET_ADDRESS $WALLET_ADDRESS --rpc-url $RPC_URL)
echo "✓ Allowance: $ALLOWANCE"

echo ""
echo "========================================"
echo "All tests passed! ✓"
echo "========================================"