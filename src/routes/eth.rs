use crate::{
    utils::{
        chain::fetch_from_chain,
        db::check_transaction_in_db,
        rlp::{decode_rlp_encoded_list, encode_hexes_to_rlp},
        structs::{FetchResponse, ResultTransaction, TransactionHashesQuery},
    },
    AppState,
};
use axum::{
    extract::{Path, Query, State},
    routing::get,
    Json, Router,
};

pub fn routes() -> Router<AppState> {
    Router::new()
        .route(
            "/",
            get(|app_state, query| fetch_eth_txs(app_state, query, None, false)),
        )
        .route(
            "/:rlp",
            get(|app_state, query, path| fetch_eth_txs(app_state, query, Some(path), true)),
        )
}

pub async fn fetch_eth_txs(
    State(state): State<AppState>,
    Query(query): Query<TransactionHashesQuery>,
    rlp: Option<Path<String>>,
    are_rlp_encoded: bool,
) -> Json<FetchResponse> {
    let mut result: Vec<ResultTransaction> = vec![];

    let query_list = if are_rlp_encoded {
        match decode_rlp_encoded_list(&rlp.unwrap()).await {
            Ok(trx_hashes) => trx_hashes
                .clone()
                .iter_mut()
                .map(|hash| format!("0x{}", hash))
                .collect(),
            Err(_) => vec![],
        }
    } else {
        query.transaction_hashes
    };

    // let encoded = encode_hexes_to_rlp(&query_list).await;
    // println!("rlp_encoded, {:#?}", encoded);

    for i in 0..query_list.len() {
        let transaction_hash = query_list.get(i).unwrap();

        match check_transaction_in_db(&state.db_connection, transaction_hash).await {
            Ok(Some(res)) => {
                println!("{} fetched from db", transaction_hash);
                result.push(res)
            }
            Ok(None) => match fetch_from_chain(transaction_hash, &state, &mut result).await {
                Ok(_) => println!("{} fetched and saved in db", transaction_hash),
                Err(err_msg) => {
                    println!(
                        "Failed to fetch {} from chain, error:{}",
                        transaction_hash, err_msg
                    )
                }
            },
            Err(error) => println!("Failed to fetch from db: {}", error),
        }
    }

    Json(FetchResponse {
        transactions: result,
    })
}
