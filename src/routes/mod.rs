mod learn;
mod crud;


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

use crate::support::app_state::AppState;

pub fn init_routes(app_state: AppState) -> Router {
    let learn = Router::new()
        .route("/hello", get(hello_world))
        .route("/body-mirror", post(body_mirror))
        .route("/path-mirror/:id", get(path_mirror))
        .route("/query-mirror", get(query_mirror));

    let crud = Router::new()
        .route("/", get(find_all_users).post(create_user))
        .route("/:id", get(find_user_by_id).put(update_user).delete(delete_user))
        .with_state(app_state);

    Router::new()
        .nest("/api/learn", learn)
        .nest("/api/crud/users", crud)
}