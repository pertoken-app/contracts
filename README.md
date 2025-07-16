# Ethicrawler Soroban Smart Contract

A Soroban smart contract that handles payment processing and proof generation for the Ethicrawler pay-per-crawl system.

## Features

- Payment invoice generation with unique identifiers
- Stellar blockchain payment verification
- JWT proof token issuance for authenticated content access
- Transparent on-chain payment logging
- Cryptographically secure payment validation

## Contract Functions

### `request_payment(site_id, url_hash, amount)`
Generates a unique payment invoice for content access.

### `submit_payment(payment_id, tx_hash, payer_key)`
Verifies payment transaction and issues JWT proof token.

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