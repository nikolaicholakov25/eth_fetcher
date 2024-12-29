use crate::{
    utils::{
        chain::fetch_from_chain,
        db::{transaction::check_transaction_in_db, user::save_user_trx},
        rlp::decode_rlp_encoded_list,
        structs::{
            auth::AuthUser,
            transaction::{FetchResponse, ResultTransaction, TransactionHashesQuery},
        },
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
            get(|app_state, query, user| fetch_eth_txs(app_state, query, None, false, user)),
        )
        .route(
            "/:rlp",
            get(|app_state, query, path, user| {
                fetch_eth_txs(app_state, query, Some(path), true, user)
            }),
        )
}

pub async fn fetch_eth_txs(
    State(state): State<AppState>,
    Query(query): Query<TransactionHashesQuery>,
    rlp: Option<Path<String>>,
    are_rlp_encoded: bool,
    user: Option<AuthUser>,
) -> Json<FetchResponse> {
    let mut result: Vec<ResultTransaction> = vec![];

    let query_list = if are_rlp_encoded {
        match decode_rlp_encoded_list(&rlp.unwrap()) {
            Ok(trx_hashes) => trx_hashes
                .clone()
                .iter_mut()
                // the decoded rlp hexes are missing the "0x" prefix
                // added it for consistency
                .map(|hash| format!("0x{}", hash))
                .collect(),
            Err(_) => vec![],
        }
    } else {
        query.transaction_hashes
    };

    for i in 0..query_list.len() {
        let transaction_hash = query_list.get(i).unwrap();

        match check_transaction_in_db(&state.db_connection, transaction_hash).await {
            Ok(Some(res)) => {
                println!("{} fetched from db", transaction_hash);
                result.push(res)
            }
            Ok(None) => match fetch_from_chain(transaction_hash, &state, &mut result).await {
                Ok(_) => {
                    println!("{} fetched and saved in db", transaction_hash);

                    // save user trx if authenticated and if trx exists
                    if let Some(auth_user) = &user {
                        save_user_trx(
                            &state.db_connection,
                            transaction_hash,
                            &auth_user.db_user().name,
                        )
                        .await
                        .expect("Failed to save user_trx");
                    };
                }
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
