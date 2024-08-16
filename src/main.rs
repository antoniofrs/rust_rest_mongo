use app::config::logging::init_logging;
use app::routes::init_routes;
use tokio::net::TcpListener;

#[tokio::main]
async fn main() {

    init_logging();

    let app = init_routes().await;

    let listener = TcpListener::bind("0.0.0.0:3000").await.unwrap();

    tracing::info!("Listening on {}", listener.local_addr().unwrap());
    
    axum::serve(listener, app).await.unwrap();
    
}