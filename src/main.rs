use std::env;
use app::config::logging::init_logging;
use app::config::mongo_client::{get_database, MongoConfig};
use app::config::sqs_client::get_local_sqs_client;
use app::repository::user_repository::UserRepository;
use app::routes::init_routes;
use app::service::user_service::UserService;
use app::support::sqs_listener::SqsListenerBuilder;
use std::sync::Arc;
use app::service::sqs_service::SqsService;

#[tokio::main]
async fn main() {
    init_logging();

    let sqs_client = Arc::new(get_local_sqs_client().await);
    let database = get_database(MongoConfig::default()).await;
    let user_repository = Arc::new(UserRepository::init(database));
    let user_service = Arc::new(UserService::init(user_repository));

    let sqs_service = SqsService::new(Arc::clone(&sqs_client));

    let routes = init_routes(
        user_service.clone(),
        sqs_service
    );

    let listener = SqsListenerBuilder::from(sqs_client).await
        .polling_delay(30)
        .add_queue(env::var("QUEUE_1_URL").unwrap(), user_service.clone())
        .add_queue(env::var("QUEUE_2_URL").unwrap(), user_service.clone())
        .add_queue(env::var("QUEUE_3_URL").unwrap(), user_service.clone())
        .add_queue(env::var("QUEUE_4_URL").unwrap(), user_service.clone())
        .add_queue(env::var("QUEUE_5_URL").unwrap(), user_service.clone())
        .add_queue(env::var("QUEUE_6_URL").unwrap(), user_service.clone())
        .run();

    let _ = tokio::join!(listener, routes);
}