use chrono::Utc;
use jsonwebtoken::{DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};

use crate::config::CONFIG;

#[derive(Serialize, Deserialize)]
pub struct Claims {
    sub: String,
    exp: usize,
    iat: usize,
}

impl Claims {
    fn new(user_id: &str) -> Self {
        let now = Utc::now();
        let exp = now + CONFIG.jwt_expiration_duration;

        Self {
            sub: user_id.to_string(),
            exp: exp.timestamp() as usize,
            iat: now.timestamp() as usize,
        }
    }
}

pub async fn generate_token(user_id: &str, secret: &str) -> Result<String, String> {
    let claims = Claims::new(user_id);

    jsonwebtoken::encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(secret.as_bytes()),
    )
    .map_err(|e| e.to_string())
}

pub async fn validate_token(token: &str, secret: &str) -> Result<String, String> {
    let claims = jsonwebtoken::decode::<Claims>(
        token,
        &DecodingKey::from_secret(secret.as_bytes()),
        &Validation::default(),
    )
    .map_err(|e| e.to_string())?;

    Ok(claims.claims.sub)
}
