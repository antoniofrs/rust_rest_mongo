use mongodb::Database;
use axum::extract::FromRef;

#[derive(Clone, FromRef)]
pub struct AppState {
    pub db: Database
}