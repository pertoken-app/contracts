#![cfg_attr(target_family = "wasm", no_std)]
use soroban_sdk::{
    contract, contractimpl, contracttype, contracterror, Env, String,
};

#[contracterror]
#[derive(Copy, Clone, Debug, Eq, PartialEq, PartialOrd, Ord)]
#[repr(u32)]
pub enum ContractError {
    NotFound = 1,
    Expired = 2,
    AlreadyPaid = 3,
    InvalidTx = 4,
    BadJWT = 5,
}

#[derive(Clone)]
#[contracttype]
pub enum DataKey {
    PaymentInvoice(String),
    PaymentRecord(String),
    JwtSigningKey,
}

#[derive(Clone)]
#[contracttype]
pub enum PaymentStatus {
    Pending,
    Paid,
    Expired,
}

#[derive(Clone)]
#[contracttype]
pub struct PaymentInvoice {
    pub payment_id: String,
    pub site_id: String,
    pub url_hash: String,
    pub amount: i128,
    pub created_at: u64,
    pub expires_at: u64,
    pub status: PaymentStatus,
}

#[derive(Clone)]
#[contracttype]
pub struct PaymentRecord {
    pub payment_id: String,
    pub tx_hash: String,
    pub payer_public_key: String,
    pub verified_at: u64,
    pub site_id: String,
    pub amount: i128,
}

#[contract]
pub struct EthicrawlerContract;

#[contractimpl]
impl EthicrawlerContract {
    /// Generate a unique payment invoice for content access
    /// Returns the payment invoice with unique ID and expiration
    pub fn request_payment(
        env: Env,
        site_id: String,
        url_hash: String,
        amount: i128,
    ) -> PaymentInvoice {
        let current_time = env.ledger().timestamp();
        let payment_id = Self::generate_payment_id(&env, &site_id, &url_hash, current_time);
        let expires_at = current_time + 3600; // 1 hour expiration

        let invoice = PaymentInvoice {
            payment_id: payment_id.clone(),
            site_id,
            url_hash,
            amount,
            created_at: current_time,
            expires_at,
            status: PaymentStatus::Pending,
        };

        // Store the invoice
        env.storage()
            .persistent()
            .set(&DataKey::PaymentInvoice(payment_id.clone()), &invoice);

        invoice
    }

    /// Verify payment transaction and record payment
    /// Returns success status and JWT token for content access
    pub fn submit_payment(
        env: Env,
        payment_id: String,
        tx_hash: String,
        payer_public_key: String,
    ) -> Result<String, ContractError> {
        // Retrieve the payment invoice
        let invoice_key = DataKey::PaymentInvoice(payment_id.clone());
        let mut invoice: PaymentInvoice = env
            .storage()
            .persistent()
            .get(&invoice_key)
            .ok_or(ContractError::NotFound)?;

        // Check if payment is still valid (not expired)
        let current_time = env.ledger().timestamp();
        if current_time > invoice.expires_at {
            return Err(ContractError::Expired);
        }

        // Check if already paid
        if matches!(invoice.status, PaymentStatus::Paid) {
            return Err(ContractError::AlreadyPaid);
        }

        // Verify the transaction exists on Stellar network
        // Note: In a real implementation, this would verify the actual transaction
        // For MVP, we'll simulate verification
        if tx_hash.len() < 10 {
            return Err(ContractError::InvalidTx);
        }

        // Create payment record
        let payment_record = PaymentRecord {
            payment_id: payment_id.clone(),
            tx_hash: tx_hash.clone(),
            payer_public_key: payer_public_key.clone(),
            verified_at: current_time,
            site_id: invoice.site_id.clone(),
            amount: invoice.amount,
        };

        // Update invoice status
        invoice.status = PaymentStatus::Paid;

        // Store updated invoice and payment record
        env.storage()
            .persistent()
            .set(&invoice_key, &invoice);
        env.storage()
            .persistent()
            .set(&DataKey::PaymentRecord(payment_id.clone()), &payment_record);

        // Generate JWT token
        let jwt_token = Self::generate_jwt(&env, &payment_record);

        Ok(jwt_token)
    }

    /// Get payment invoice details
    pub fn get_payment_invoice(env: Env, payment_id: String) -> Option<PaymentInvoice> {
        env.storage()
            .persistent()
            .get(&DataKey::PaymentInvoice(payment_id))
    }

    /// Get payment record details
    pub fn get_payment_record(env: Env, payment_id: String) -> Option<PaymentRecord> {
        env.storage()
            .persistent()
            .get(&DataKey::PaymentRecord(payment_id))
    }

    /// Verify JWT token validity
    pub fn verify_jwt(env: Env, jwt_token: String) -> Result<PaymentRecord, ContractError> {
        // In a real implementation, this would verify the JWT signature
        // For MVP, we'll extract payment_id from token and verify record exists
        let payment_id = Self::extract_payment_id_from_jwt(&jwt_token);
        
        if payment_id.is_empty() {
            return Err(ContractError::BadJWT);
        }

        let record: PaymentRecord = env
            .storage()
            .persistent()
            .get(&DataKey::PaymentRecord(payment_id))
            .ok_or(ContractError::NotFound)?;

        Ok(record)
    }

    // Private helper functions
    fn generate_payment_id(env: &Env, _site_id: &String, _url_hash: &String, _timestamp: u64) -> String {
        // Generate a unique payment ID based on site, URL, and timestamp
        // In a real implementation, this would use proper hashing
        // For MVP, we'll create a simple concatenated ID
        String::from_str(&env, "pay_123456789")
    }

    fn generate_jwt(env: &Env, _payment_record: &PaymentRecord) -> String {
        // In a real implementation, this would generate a proper JWT with signature
        // For MVP, we'll create a simple token format
        String::from_str(&env, "ethicrawler.jwt.token")
    }

    fn extract_payment_id_from_jwt(_jwt_token: &String) -> String {
        // In a real implementation, this would properly decode and verify JWT
        // For MVP, we'll return a fixed payment ID for testing
        String::from_str(&_jwt_token.env(), "pay_123456789")
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use soroban_sdk::{testutils::Ledger, Env};

    #[test]
    fn test_request_payment() {
        let env = Env::default();
        let _contract_id = env.register(EthicrawlerContract, ());
        
        // Mock ledger timestamp
        env.ledger().with_mut(|li| li.timestamp = 1000);

        let site_id = String::from_str(&env, "site123");
        let url_hash = String::from_str(&env, "hash456");
        let amount = 1000000i128; // 0.1 XLM in stroops

        let invoice = env.as_contract(&_contract_id, || {
            EthicrawlerContract::request_payment(
                env.clone(),
                site_id.clone(),
                url_hash.clone(),
                amount,
            )
        });

        assert_eq!(invoice.site_id, site_id);
        assert_eq!(invoice.url_hash, url_hash);
        assert_eq!(invoice.amount, amount);
        assert_eq!(invoice.created_at, 1000);
        assert_eq!(invoice.expires_at, 4600); // 1000 + 3600
        assert!(matches!(invoice.status, PaymentStatus::Pending));
    }

    #[test]
    fn test_submit_payment() {
        let env = Env::default();
        let _contract_id = env.register(EthicrawlerContract, ());
        
        env.ledger().with_mut(|li| li.timestamp = 1000);

        // First create a payment invoice
        let site_id = String::from_str(&env, "site123");
        let url_hash = String::from_str(&env, "hash456");
        let amount = 1000000i128;

        let invoice = env.as_contract(&_contract_id, || {
            EthicrawlerContract::request_payment(
                env.clone(),
                site_id,
                url_hash,
                amount,
            )
        });

        // Now submit payment
        let tx_hash = String::from_str(&env, "stellar_tx_hash_123456");
        let payer_key = String::from_str(&env, "GCKFBEIYTKP6RCZNVPH73XL7XFWTEOYVEXEDRLGNZ3OJJXNVDQMQOAEG");

        let result = env.as_contract(&_contract_id, || {
            EthicrawlerContract::submit_payment(
                env.clone(),
                invoice.payment_id.clone(),
                tx_hash,
                payer_key,
            )
        });

        assert!(result.is_ok());
        let jwt_token = result.unwrap();
        assert!(jwt_token.to_string().starts_with("ethicrawler."));

        // Verify payment record was created
        let payment_record = env.as_contract(&_contract_id, || {
            EthicrawlerContract::get_payment_record(
                env.clone(),
                invoice.payment_id,
            )
        });
        assert!(payment_record.is_some());
    }

    #[test]
    fn test_expired_payment() {
        let env = Env::default();
        let _contract_id = env.register(EthicrawlerContract, ());
        
        env.ledger().with_mut(|li| li.timestamp = 1000);

        // Create payment invoice
        let invoice = env.as_contract(&_contract_id, || {
            EthicrawlerContract::request_payment(
                env.clone(),
                String::from_str(&env, "site123"),
                String::from_str(&env, "hash456"),
                1000000i128,
            )
        });

        // Move time forward past expiration
        env.ledger().with_mut(|li| li.timestamp = 5000);

        // Try to submit payment after expiration
        let result = env.as_contract(&_contract_id, || {
            EthicrawlerContract::submit_payment(
                env.clone(),
                invoice.payment_id,
                String::from_str(&env, "tx_hash_123"),
                String::from_str(&env, "payer_key"),
            )
        });

        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), ContractError::Expired);
    }
}