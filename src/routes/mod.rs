mod learn;
mod crud;
mod sqs_queue;

use std::sync::Arc;
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
use crate::routes::sqs_queue::queue_producer::queue_producer;
use crate::service::sqs_service::SqsService;

pub async fn init_routes(user_service: Arc<UserService>, sqs_service: SqsService) {
    let user_routes = init_user_routes(user_service).await;
    let sqs_routes = init_sqs_routes(sqs_service).await;
    let learn_routes = init_learn_routes();


    let app = Router::new()
        .nest("/api/learn", learn_routes)
        .nest("/api/crud/users", user_routes)
        .nest("/api/sqs_queue", sqs_routes);

    let listener = TcpListener::bind("0.0.0.0:3000").await.unwrap();

    tracing::info!("Listening on {}", listener.local_addr().unwrap());

    axum::serve(listener, app).await.unwrap();
}

async fn init_sqs_routes(sqs_service: SqsService) -> Router {
    Router::new()
        .route("/send-message", post(queue_producer))
        .with_state(sqs_service)
}

async fn init_user_routes(user_service: Arc<UserService>) -> Router {

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