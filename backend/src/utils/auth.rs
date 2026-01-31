use jsonwebtoken::{encode, decode, Header, Algorithm, Validation, EncodingKey, DecodingKey};
use serde::{Serialize, Deserialize};
use time::{OffsetDateTime, Duration};

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String,     // User ID (subject)
    pub sid: String,     // Session ID
    pub exp: i64,        // Expiration time
    pub iat: i64,        // Issued at
}

pub fn create_jwt(user_id: i32, session_id: &str) -> Result<String, String> {
    let secret = std::env::var("JWT_SECRET").map_err(|_| "JWT_SECRET not set")?;
    
    let now = OffsetDateTime::now_utc();
    let expiration = now + Duration::days(365); // "Onthoud mij" - 1 jaar geldig
    
    let claims = Claims {
        sub: user_id.to_string(),
        sid: session_id.to_string(),
        exp: expiration.unix_timestamp(),
        iat: now.unix_timestamp(),
    };
    
    encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(secret.as_bytes())
    ).map_err(|e| format!("Token creation failed: {}", e))
}

pub fn verify_jwt(token: &str) -> Result<Claims, String> {
    let secret = std::env::var("JWT_SECRET").map_err(|_| "JWT_SECRET not set")?;
    
    decode::<Claims>(
        token,
        &DecodingKey::from_secret(secret.as_bytes()),
        &Validation::new(Algorithm::HS256)
    )
    .map(|data| data.claims)
    .map_err(|e| format!("Token verification failed: {}", e))
}
