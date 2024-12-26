use std::env;
use std::sync::LazyLock;

use alloy::providers::RootProvider;
use alloy::transports::http::Http;
use reqwest::Client;
use sqlx::{Pool, Postgres};

pub fn load_config() {
    // load envs
    dotenv::dotenv().ok();
}

#[derive(Clone, Debug)]
pub struct AppState {
    pub eth_client: RootProvider<Http<Client>>,
    pub db_connection: Pool<Postgres>,
}

pub static API_PORT: LazyLock<String> =
    LazyLock::new(|| env::var("API_PORT").expect("API_PORT is not set"));

pub static ETH_NODE_URL: LazyLock<String> =
    LazyLock::new(|| env::var("ETH_NODE_URL").expect("ETH_NODE_URL is not set"));

pub static DB_CONNECTION_URL: LazyLock<String> =
    LazyLock::new(|| env::var("DB_CONNECTION_URL").expect("DB_CONNECTION_URL is not set"));

pub static JWT_SECRET: LazyLock<String> =
    LazyLock::new(|| env::var("JWT_SECRET").expect("JWT_SECRET is not set"));
