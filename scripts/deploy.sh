#!/bin/bash

# Stylus ERC-20 Token Deployment Script
echo "========================================"
echo "  Deploying Stylus ERC-20 Token"
echo "========================================"

# Load environment variables if .env exists
if [ -f .env ]; then
    export $(cat .env | grep -v '^#' | xargs)
fi

# Check if required environment variables are set
if [ -z "$PRIVATE_KEY" ]; then
    echo "Error: PRIVATE_KEY not set in .env"
    echo "Please add your private key to .env file:"
    echo "PRIVATE_KEY=your_private_key_here"
    exit 1
fi

if [ -z "$RPC_URL" ]; then
    echo "Warning: RPC_URL not set, using default Arbitrum Sepolia RPC"
    RPC_URL="https://sepolia-rollup.arbitrum.io/rpc"
fi

echo ""
echo "Step 1: Building the contract..."
cargo build --release --target wasm32-unknown-unknown

if [ $? -ne 0 ]; then
    echo "Error: Build failed"
    exit 1
fi

echo ""
echo "Step 2: Checking Stylus tools installation..."
if ! command -v cargo-stylus &> /dev/null; then
    echo "Installing cargo-stylus..."
    cargo install cargo-stylus
fi

echo ""
echo "Step 3: Deploying to Stylus..."
echo "Network: $RPC_URL"

# Deploy the contract
DEPLOY_OUTPUT=$(cargo stylus deploy \
    --private-key="$PRIVATE_KEY" \
    --endpoint="$RPC_URL" \
    --no-verify \
    2>&1)

echo "$DEPLOY_OUTPUT"

# Extract contract address from deployment output
CONTRACT_ADDRESS=$(echo "$DEPLOY_OUTPUT" | grep -oE "0x[a-fA-F0-9]{40}" | head -1)

if [ -z "$CONTRACT_ADDRESS" ]; then
    echo ""
    echo "Error: Failed to extract contract address from deployment output"
    exit 1
fi

echo ""
echo "=========================================="
echo "  Deployment Successful!"
echo "=========================================="
echo "Contract Address: $CONTRACT_ADDRESS"
echo ""

# Update .env file with contract address
if [ -f .env ]; then
    # Remove old CONTRACT_ADDRESS if it exists
    sed -i.bak '/^CONTRACT_ADDRESS=/d' .env
    rm -f .env.bak
fi

# Add new CONTRACT_ADDRESS
echo "CONTRACT_ADDRESS=$CONTRACT_ADDRESS" >> .env

echo "Contract address has been saved to .env"
echo ""
echo "You can now interact with your token at:"
echo "$CONTRACT_ADDRESS"
echo ""