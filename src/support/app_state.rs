use axum::extract::FromRef;
use mongodb::Database;

#[derive(Clone, FromRef)]
pub struct AppState {
    pub db: Database
}