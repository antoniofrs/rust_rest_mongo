use crate::error_handler::model::app_error::AppError;
use jsonwebtoken::DecodingKey;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Jwk {
    kty: String,
    kid: String,
    use_: String,
    n: String,
    e: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Jwks {
    keys: Vec<Jwk>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub iss: String,
    sub: String,
    aud: String,
    exp: usize,
    iat: usize,
}

pub async fn fetch_jwks(uri: String) -> Result<Jwks, AppError> {
    let res = reqwest::get(uri).await.unwrap();
    let jwks = res.json::<Jwks>().await.unwrap();
    Ok(jwks)
}

pub fn get_decoding_key(jwks: &Jwks, kid: &str) -> Option<DecodingKey> {
    for jwk in &jwks.keys {
        if jwk.kid == kid {
            let decoding_key = DecodingKey::from_rsa_components(&jwk.n, &jwk.e).ok()?;
            return Some(decoding_key);
        }
    }
    None
}

#[derive(Debug, Clone)]
pub struct ValidToken {
    pub permissions: Vec<String>,
}