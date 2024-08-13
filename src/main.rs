use std::env;
use tokio::net::TcpListener;
use app::config::logging::init_logging;
use app::config::mongo_client::mongo_client;
use app::routes::init_routes;
use app::support::app_state::AppState;

#[tokio::main]
async fn main() {

    init_logging();
    
    let uri  = env::var("MONGO_URI")
        .unwrap_or("mongodb://root:pass@localhost:27017/?authSource=admin&w=majority".to_owned());
    
    let database_name = env::var("DB_NAME")
        .unwrap_or("rust-rest".to_owned());
    
    let database = mongo_client(uri, database_name).await;

    let state = AppState {
        db: database
    };
    
    let app = init_routes(state);

    let listener = TcpListener::bind("0.0.0.0:3000").await.unwrap();

    tracing::info!("Listening on {}", listener.local_addr().unwrap());
    
    axum::serve(listener, app).await.unwrap();
    
}