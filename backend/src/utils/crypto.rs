use argon2::{
    password_hash::{rand_core::OsRng, PasswordHasher, SaltString, PasswordVerifier},
    Argon2,
};
use aes_gcm::{
    aead::{Aead, KeyInit, OsRng as CryptoOsRng, rand_core::RngCore},
    Aes256Gcm, Nonce
};
use crate::definitions::metadata::Metadata;

pub fn hash_password(password: &str) -> Result<String, String> {
    let salt = SaltString::generate(&mut OsRng);
    let argon2 = Argon2::default();
    argon2
        .hash_password(password.as_bytes(), &salt)
        .map(|h| h.to_string())
        .map_err(|e| format!("Password hashing failed: {}", e))
}

pub fn verify_password(password: &str, hash: &str) -> bool {
    let argon2 = Argon2::default();
    if let Ok(parsed_hash) = argon2::PasswordHash::new(hash) {
        argon2.verify_password(password.as_bytes(), &parsed_hash).is_ok()
    } else {
        false
    }
}

pub fn encrypt_metadata(data: &Metadata) -> Result<String, String> {
    let metadata_str = serde_json::to_string(data).map_err(|e| e.to_string())?;

    let key_hex = std::env::var("ENCRYPTION_KEY").map_err(|_| "ENCRYPTION_KEY not set")?;
    let key_bytes = hex::decode(key_hex).map_err(|_| "Invalid hex in ENCRYPTION_KEY")?;
    
    let cipher = Aes256Gcm::new_from_slice(&key_bytes).map_err(|e| format!("Invalid key: {}", e))?;
    
    let mut nonce_bytes = [0u8; 12];
    CryptoOsRng.fill_bytes(&mut nonce_bytes);
    let nonce = Nonce::from_slice(&nonce_bytes); 
    
    let encrypted_bytes = cipher.encrypt(nonce, metadata_str.as_bytes())
        .map_err(|e| format!("Encryption failed: {}", e))?;
    
    let mut final_payload = nonce_bytes.to_vec();
    final_payload.extend_from_slice(&encrypted_bytes);
    
    Ok(hex::encode(final_payload))
}

#[allow(dead_code)]
pub fn decrypt_metadata(encrypted_hex: &str) -> Result<Metadata, String> {
    let full_payload = hex::decode(encrypted_hex).map_err(|_| "Invalid hex in metadata")?;
    if full_payload.len() < 12 {
        return Err("Invalid metadata".into());
    }

    let (nonce_bytes, encrypted_bytes) = full_payload.split_at(12);
    let nonce = Nonce::from_slice(nonce_bytes);

    let key_hex = std::env::var("ENCRYPTION_KEY").map_err(|_| "ENCRYPTION_KEY not set")?;
    let key_bytes = hex::decode(key_hex).map_err(|_| "Invalid hex in ENCRYPTION_KEY")?;
    
    let cipher = Aes256Gcm::new_from_slice(&key_bytes).map_err(|e| format!("Invalid key: {}", e))?;
    
    let decrypted_bytes = cipher.decrypt(nonce, encrypted_bytes)
        .map_err(|e| format!("Decryption failed: {}", e))?;
    
    serde_json::from_slice(&decrypted_bytes).map_err(|e| format!("Metadata parsing failed: {}", e))
}
