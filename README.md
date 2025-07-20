# Stellar Paywall System - Smart Contracts

Soroban smart contracts for the Stellar blockchain-based paywall system that enables pay-per-token content access.

## Features

- Token-based payment processing for content access
- Stellar blockchain payment verification
- Content access authorization and proof generation
- Transparent on-chain payment logging
- Secure payment validation and token management

## Contract Functions

### `request_payment(content_id, token_amount)`
Generates a unique payment request for token-based content access.

### `verify_payment(payment_id, tx_hash, payer_key)`
Verifies payment transaction and authorizes content access.

### `get_content_access(content_id, user_key)`
Validates user's payment and returns content access authorization.

## Deployed Contracts

### Futurenet
- **Contract ID**: `CC4N2TWAOLYTTQL43JTDKT6TLJAEIDCGROSQNP74WFTHKKA2RBUREDJKMaybe`
- **Network**: Futurenet
- **RPC URL**: `https://rpc-futurenet.stellar.org:443`
- **Deployed**: January 20, 2025

## Development

```bash
# Build the contract
stellar contract build

# Run tests
stellar contract test

# Deploy to testnet
stellar contract deploy --network testnet

# Deploy to futurenet
stellar contract deploy --network futurenet
```

## Configuration

Contract deployment information is stored in `contract-config.json` for easy reference by the backend and other services.

## Requirements

- Stellar CLI (v23+)
- Rust toolchain
- Stellar account for deployment

## License

MIT License - see LICENSE file for details