use mongodb::Database;
use app::config::mongo_client::{get_database, MongoConfig};
use testcontainers::runners::AsyncRunner;
use testcontainers::ContainerAsync;
use testcontainers_modules::mongo::Mongo;
use tokio::sync::OnceCell;

static MONGO_TEST_CONTAINER: OnceCell<ContainerAsync<Mongo>> = OnceCell::const_new();

pub async fn get_test_database_config() -> MongoConfig {
    let container = MONGO_TEST_CONTAINER.get_or_init(|| async {
        Mongo::default().start().await.unwrap()
    }).await;

    let host = container.get_host().await.unwrap();
    let port = container.get_host_port_ipv4(27017).await.unwrap();
    let uri = format!("mongodb://{host}:{port}/?authSource=admin&w=majority");

    MongoConfig { database_name: "test".into(), uri }
}

pub async fn get_test_database() -> Database {
    let config = get_test_database_config().await;
    get_database(config).await
}