use std::collections::BTreeMap;
use hmac::Hmac;
use jwt::SignWithKey;
use sha2::Sha256;
use crate::web::routing::dto::UserDto;

pub mod login;
pub mod signup;


#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
pub struct AuthResponse {
    pub user: UserDto,
    pub token: String,
}

pub fn generate_jwt_token(key: &Hmac<Sha256>, user_id: u64) -> String {
    let mut token_claims = BTreeMap::new();
    token_claims.insert("user_id", user_id.to_string());
    token_claims.sign_with_key(key)
        .expect("Failed to sign JWT")
}