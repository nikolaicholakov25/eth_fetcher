mod config;
mod routes;
mod utils;

use alloy::providers::ProviderBuilder;
use axum::Router;
use config::{load_config, AppState, API_PORT, DB_CONNECTION_URL, ETH_NODE_URL};
use utils::db::set_up::init_db;

#[tokio::main(flavor = "current_thread")]
async fn main() {
    // load envs
    load_config();

    let shared_state: AppState = AppState {
        eth_client: ProviderBuilder::new().on_http(ETH_NODE_URL.parse().unwrap()),
        db_connection: sqlx::postgres::PgPoolOptions::new()
            .max_connections(50)
            .connect(&DB_CONNECTION_URL)
            .await
            .unwrap(),
    };

    init_db(&shared_state.db_connection).await.unwrap();

    // build routes
    let app = Router::new()
        .nest("/lime/eth", routes::eth::routes())
        .nest("/lime/all", routes::all::routes())
        .nest("/lime/", routes::auth::routes())
        .with_state(shared_state);

    // listen for server
    let listener = tokio::net::TcpListener::bind(format!("127.0.0.1:{}", API_PORT.to_string()))
        .await
        .unwrap();
    axum::serve(listener, app).await.unwrap();
}
