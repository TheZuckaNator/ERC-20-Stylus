#!/bin/bash

# Colors for output
GREEN='\033[0;32m'
RED='\033[0;31m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

echo -e "${BLUE}========================================${NC}"
echo -e "${BLUE}  Push-Based Distributor Test Suite${NC}"
echo -e "${BLUE}========================================${NC}\n"

# Step 1: Run Rust unit tests
echo -e "${YELLOW}📋 Step 1: Running unit tests...${NC}\n"
cargo test

if [ $? -eq 0 ]; then
    echo -e "\n${GREEN}✅ Unit tests passed!${NC}\n"
else
    echo -e "\n${RED}❌ Unit tests failed!${NC}\n"
    exit 1
fi

# Step 2: Build the contract
echo -e "${YELLOW}🔨 Step 2: Building contract...${NC}\n"
cargo build --release

if [ $? -eq 0 ]; then
    echo -e "\n${GREEN}✅ Build successful!${NC}\n"
else
    echo -e "\n${RED}❌ Build failed!${NC}\n"
    exit 1
fi

# Step 3: Run Stylus checks (try local node first, then testnet)
echo -e "${YELLOW}🔍 Step 3: Running Stylus validation checks...${NC}\n"

# Check if local node is available
if curl -s -X POST http://localhost:8547 \
    -H "Content-Type: application/json" \
    -d '{"jsonrpc":"2.0","method":"eth_blockNumber","params":[],"id":1}' \
    > /dev/null 2>&1; then
    
    echo -e "${BLUE}Using local devnode at http://localhost:8547${NC}\n"
    cargo stylus check
    STYLUS_CHECK_RESULT=$?
else
    echo -e "${YELLOW}Local devnode not available, using Arbitrum Sepolia testnet${NC}\n"
    cargo stylus check --endpoint https://sepolia-rollup.arbitrum.io/rpc
    STYLUS_CHECK_RESULT=$?
fi

if [ $STYLUS_CHECK_RESULT -eq 0 ]; then
    echo -e "\n${GREEN}✅ Stylus validation passed!${NC}\n"
else
    echo -e "\n${RED}❌ Stylus validation failed!${NC}\n"
    exit 1
fi

# Final summary
echo -e "${BLUE}========================================${NC}"
echo -e "${GREEN}🎉 All tests passed successfully!${NC}"
echo -e "${BLUE}========================================${NC}\n"

echo -e "${GREEN}Contract is ready for deployment!${NC}\n"