use crate::config::AppState;
use alloy::{
    consensus::Transaction,
    primitives::FixedBytes,
    providers::Provider,
    rpc::types::{Filter, Log, TransactionReceipt},
    transports::{RpcError, TransportErrorKind},
};

use super::{db::save_transaction_to_db, misc::parse_b256_from_str, structs::ResultTransaction};

fn unwrap_receipt(
    receipt: Result<Option<TransactionReceipt>, RpcError<TransportErrorKind>>,
    b256_transaction_hash: FixedBytes<32>,
) -> Result<TransactionReceipt, String> {
    match receipt.unwrap() {
        Some(receipt_result) => Ok(receipt_result),
        None => Err(format!(
            "No receipt found for trx_hash {}",
            b256_transaction_hash
        )),
    }
}

async fn fetch_logs(
    state: &AppState,
    block_hash: FixedBytes<32>,
    trx_hash: FixedBytes<32>,
) -> Result<Vec<Log>, String> {
    // fetch log
    let logs_filter = Filter::new().at_block_hash(block_hash);

    match state.eth_client.get_logs(&logs_filter).await {
        Ok(block_logs) => {
            let transaction_logs: Vec<Log> = block_logs
                .into_iter()
                .filter(|log| log.transaction_hash == Some(trx_hash))
                .collect();

            Ok(transaction_logs)
        }
        Err(msg) => Err(format!("Failed to fetch logs: {}", msg)),
    }
}

pub async fn fetch_from_chain(
    transaction_hash: &String,
    state: &AppState,
    result: &mut Vec<ResultTransaction>,
) -> Result<(), String> {
    match parse_b256_from_str(transaction_hash) {
        Ok(b256_transaction_hash) => {
            let (transaction, receipt) = tokio::join!(
                state
                    .eth_client
                    .get_transaction_by_hash(b256_transaction_hash),
                state
                    .eth_client
                    .get_transaction_receipt(b256_transaction_hash)
            );

            match transaction.unwrap() {
                Some(trx_result) => {
                    let block_hash = trx_result.block_hash.unwrap_or(FixedBytes::default());

                    let receipt_option: Option<TransactionReceipt> =
                        match unwrap_receipt(receipt, b256_transaction_hash) {
                            Ok(rec) => Option::from(rec),
                            _ => None,
                        };

                    let mapped_trx = ResultTransaction {
                        block_hash: block_hash.to_string(),
                        block_number: trx_result.block_number.unwrap_or(0) as i32,
                        contract_address: match receipt_option.clone() {
                            Some(receipt_value) => match receipt_value.contract_address {
                                Some(contract_add) => Option::from(contract_add.to_string()),
                                None => None,
                            },
                            _ => None,
                        },
                        from: trx_result.from.to_string(),
                        to: match trx_result.to() {
                            Some(to_value) => Option::from(to_value.to_string()),
                            None => None,
                        },
                        input: trx_result.input().to_string(),
                        logs_count: match fetch_logs(&state, block_hash, b256_transaction_hash)
                            .await
                        {
                            Ok(logs) => logs.len() as i32,
                            Err(_) => 0,
                        },
                        transaction_hash: transaction_hash.to_owned(),
                        transaction_status: match receipt_option.clone() {
                            Some(receipt_value) => receipt_value.status() as i16,
                            _ => 0,
                        },
                        value: trx_result.value().to_string(),
                    };

                    // save trx to db
                    let db_result = save_transaction_to_db(&state.db_connection, &mapped_trx).await;

                    match db_result {
                        Ok(_) => println!("Transaction saved to db"),
                        Err(error) => {
                            println!(
                                "Failed to save trx - {} in db: {:#?}",
                                transaction_hash, error
                            )
                        }
                    }

                    // add to result VEC
                    result.push(mapped_trx);
                }
                None => {
                    println!(
                        "No transaction found for trx_hash {}",
                        b256_transaction_hash
                    );
                }
            }
        }
        Err(err_msg) => return Err(err_msg),
    };

    Ok(())
}
