use crate::utils::misc::comma_separated_to_vec;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
pub struct TransactionHashesQuery {
    #[serde(
        rename = "transactionHashes",
        default,
        deserialize_with = "comma_separated_to_vec"
    )]
    pub transaction_hashes: Vec<String>,
}
#[derive(sqlx::FromRow, PartialEq, Eq, Debug, Deserialize, Serialize)]
pub struct ResultTransaction {
    #[serde(rename = "transactionHash")]
    pub transaction_hash: String, // the hex encoded transaction hash of the transaction
    #[serde(rename = "transactionStatus")]
    pub transaction_status: i16, // the status of the transaction either 1 (success) or 0 (failure)
    #[serde(rename = "blockHash")]
    pub block_hash: String, // the hex encoding of the hash of the block the transaction was included in
    #[serde(rename = "blockNumber")]
    pub block_number: i32, // the number of the block the transaction was included in
    pub from: String,       // the etherum address of the transaction sender
    pub to: Option<String>, // the etherum address of the transaction receiver or null when its a contract creation transaction.
    #[serde(rename = "contractAddress")]
    pub contract_address: Option<String>, // the etherum address of the newly created contract if this transaction is contract creation
    #[serde(rename = "logsCount")]
    pub logs_count: i32, // number of log objects, which this transaction generated.
    pub input: String, // the hex encoding of the data send along with the transaction.
    pub value: String, // the value transferred in wei
}
#[derive(Serialize)]
pub struct FetchResponse {
    pub transactions: Vec<ResultTransaction>,
}
