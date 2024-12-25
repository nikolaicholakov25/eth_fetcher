use axum::{extract::State, routing::get, Json, Router};

use crate::{
    config::AppState,
    utils::{db::fetch_all_transactions, structs::FetchResponse},
};

pub fn routes() -> Router<AppState> {
    Router::new().route("/", get(fetch_all_saved_trx))
}

pub async fn fetch_all_saved_trx(State(state): State<AppState>) -> Json<FetchResponse> {
    let result = fetch_all_transactions(&state.db_connection).await.unwrap();

    Json(FetchResponse {
        transactions: result,
    })
}
