mod learn;
mod crud;

use crate::service::user_service::{UserService};
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
use crate::config::mongo_client::get_database_from_env;
use crate::repository::user_repository::{UserRepository, UserRepositoryTrait};

pub async fn init_routes() -> Router {
    let user_routes = init_user_routes().await;
    let learn_routes = init_learn_routes();



    Router::new()
        .nest("/api/learn", learn_routes)
        .nest("/api/crud/users", user_routes)
}

async fn init_user_routes() -> Router {

    let database = get_database_from_env().await;
    let user_repository = UserRepository::init(database);
    let user_service = UserService::init(user_repository);

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