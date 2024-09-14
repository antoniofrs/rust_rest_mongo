use app::config::logging::init_logging;
use app::listeners::init_sqs_listener;
use app::routes::init_routes;

#[tokio::main]
async fn main() {
    init_logging();

    let sqs = tokio::spawn(async {
        init_sqs_listener().await;
    });

    let routes = tokio::spawn(async {
        init_routes().await;
    });

    let _ = tokio::join!(sqs,routes);
}