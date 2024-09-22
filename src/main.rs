use std::env;
use app::config::logging::init_logging;
use app::config::mongo_client::{get_database, MongoConfig};
use app::config::sqs_client::get_local_sqs_client;
use app::repository::user_repository::UserRepository;
use app::routes::init_routes;
use app::service::user_service::UserService;
use app::support::sqs_listener::SqsListenerBuilder;
use std::sync::Arc;
use tokio::time::Duration;
use app::auth::authenticator::init_authentication;

#[tokio::main]
async fn main() {

    init_authentication("", "").await;
    init_logging();

    let sqs_client = get_local_sqs_client().await;
    let database = get_database(MongoConfig::default()).await;
    let user_repository = Arc::new(UserRepository::init(database));
    let user_service = Arc::new(UserService::init(user_repository));

    let routes = init_routes(user_service.clone());

    let listener = SqsListenerBuilder::from(sqs_client).await
        .polling_delay(Duration::from_secs(1))
        .add_queue(env::var("QUEUE_1_URL").unwrap(), user_service.clone())
        .add_queue(env::var("QUEUE_2_URL").unwrap(), user_service.clone())
        .run();

    let _ = tokio::join!(listener, routes);
}