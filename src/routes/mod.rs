mod learn;
mod crud;

use std::sync::Arc;
use crate::config::mongo_client::{get_database, MongoConfig};
use crate::repository::user_repository::UserRepository;
use crate::service::user_service::UserService;
use axum::routing::{get, post};
use axum::Router;
use tokio::net::TcpListener;
use crud::create::create_user;
use crud::delete::delete_user;
use crud::find_all::find_all_users;
use crud::find_by_id::find_user_by_id;
use crud::update::update_user;
use learn::hello_world::hello_world;
use learn::json_body::body_mirror;
use learn::path_mirror::path_mirror;
use learn::query_mirror::query_mirror;

pub async fn init_routes() {
    let user_routes = init_user_routes().await;
    let learn_routes = init_learn_routes();


    let app = Router::new()
        .nest("/api/learn", learn_routes)
        .nest("/api/crud/users", user_routes);

    let listener = TcpListener::bind("0.0.0.0:3000").await.unwrap();

    tracing::info!("Listening on {}", listener.local_addr().unwrap());

    axum::serve(listener, app).await.unwrap();
}

async fn init_user_routes() -> Router {
    let database = get_database(MongoConfig::default()).await;
    let user_repository = UserRepository::init(database);
    let user_service: UserService = UserService::init(Arc::new(user_repository));

    Router::new()
        .route("/", get(find_all_users).post(create_user))
        .route("/:id", get(find_user_by_id).put(update_user).delete(delete_user))
        .with_state(user_service)
}

fn init_learn_routes() -> Router {
    Router::new()
        .route("/hello", get(hello_world))
        .route("/body-mirror", post(body_mirror))
        .route("/path-mirror/:id", get(path_mirror))
        .route("/query-mirror", get(query_mirror))
}