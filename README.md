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

## Development

```bash
# Build the contract
soroban contract build

# Run tests
soroban contract test

# Deploy to testnet
soroban contract deploy --network testnet
```

## Requirements

- Soroban CLI
- Rust toolchain
- Stellar account for deployment

## License

MIT License - see LICENSE file for details