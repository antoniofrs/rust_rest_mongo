mod learn;
mod crud;

use std::env;
use axum::routing::{get, post};
use axum::Router;
use crud::create::create_user;
use crud::delete::delete_user;
use crud::find_all::find_all_users;
use crud::find_by_id::find_user_by_id;
use crud::update::update_user;
use learn::hello_world::hello_world;
use learn::json_body::body_mirror;
use learn::path_mirror::path_mirror;
use learn::query_mirror::query_mirror;
use crate::config::mongo_client::mongo_client;
use crate::repository::user_repository::{UserRepository, UserRepositoryTrait};
use crate::service::user_service::{UserService, UserServiceTrait};

pub async fn init_routes() -> Router {
    let learn = Router::new()
        .route("/hello", get(hello_world))
        .route("/body-mirror", post(body_mirror))
        .route("/path-mirror/:id", get(path_mirror))
        .route("/query-mirror", get(query_mirror));


    let uri  = env::var("MONGO_URI")
        .unwrap_or("mongodb://root:pass@localhost:27017/?authSource=admin&w=majority".to_owned());

    let database_name = env::var("DB_NAME")
        .unwrap_or("rust-rest".to_owned());

    let database = mongo_client(uri, database_name).await;
    let user_repository = UserRepository::new(database);
    let user_service = UserService::new(user_repository);

    let crud = Router::new()
        .route("/", get(find_all_users).post(create_user))
        .route("/:id", get(find_user_by_id).put(update_user).delete(delete_user))
        .with_state(user_service);

    Router::new()
        .nest("/api/learn", learn)
        .nest("/api/crud/users", crud)
}