use crate::auth::token::{fetch_jwks, get_decoding_key, Claims, Jwks, ValidToken};
use axum::http::header::AUTHORIZATION;
use axum::http::{Request, StatusCode};
use axum::middleware::Next;
use axum::response::IntoResponse;
use jsonwebtoken::{decode, decode_header, Algorithm, Validation};
use tokio::sync::OnceCell;

#[derive(Debug)]
pub struct Authenticator {
    jwks: Jwks,
    issuer: String,
    audience: String,
}

static AUTH: OnceCell<Authenticator> = OnceCell::const_new();


pub async fn init_authentication(issuer: &str, audience: &str) {
    let jwks_uri = format!("{}{}", issuer, ".well-known/jwks.json");
    let jwks = fetch_jwks(jwks_uri).await.unwrap();
    let authenticator = Authenticator {
        jwks,
        issuer: String::from(issuer),
        audience: String::from(audience),
    };

    AUTH.set(authenticator).expect("Already initialized");
}

pub async fn auth_middleware<B>(mut req: Request<B>, next: Next) -> impl IntoResponse {
    if let Some(auth_header) = req.headers().get(AUTHORIZATION) {
        if let Ok(auth_str) = auth_header.to_str() {
            if let Some(token) = auth_str.strip_prefix("Bearer ") {
                let valid_token = process_token(token).await;
                req.extensions_mut().insert(valid_token).unwrap();
                let response = next.run(req).await;
                return response;
            }
        }
    }

    (StatusCode::UNAUTHORIZED, "Invalid bearer token").into_response()
}

async fn process_token(jwt: &str) -> ValidToken {
    let authenticator = AUTH.get()
        .expect("Missing authenticator initialization");

    let header = decode_header(jwt).expect("Invalid token header");
    let kid = header.kid.expect("No `kid` found in token");

    let decoding_key = get_decoding_key(&authenticator.jwks, &kid)
        .expect("Failed to get decoding key");

    let mut validation = Validation::new(Algorithm::RS256);
    validation.set_audience(&[&authenticator.audience]);
    validation.set_issuer(&[&authenticator.issuer]);

    let token_data = decode::<Claims>(jwt, &decoding_key, &validation)
        .expect("Failed to verify token");

    ValidToken { permissions: vec![String::from(token_data.claims.iss)] }
}
