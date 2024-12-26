use sqlx::{Pool, Postgres};

use crate::utils::structs::transaction::ResultTransaction;

pub async fn save_transaction_to_db(
    pool: &Pool<Postgres>,
    trx: &ResultTransaction,
) -> Result<(), sqlx::Error> {
    // save transaction
    sqlx::query(
        r#"
        INSERT INTO transactions (
            transaction_hash,
            transaction_status,
            block_hash,
            block_number,
            "from",
            "to",
            contract_address,
            logs_count,
            input,
            value
        )
        VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10)
        "#,
    )
    .bind(trx.transaction_hash.clone())
    .bind(trx.transaction_status.clone() as i16)
    .bind(trx.block_hash.clone())
    .bind(trx.block_number.clone() as i32)
    .bind(trx.from.clone())
    .bind(trx.to.clone().map(|val| val.to_string()))
    .bind(trx.contract_address.clone())
    .bind(trx.logs_count.clone() as i32)
    .bind(trx.input.clone())
    .bind(trx.value.clone().to_string())
    .execute(pool)
    .await?;

    Ok(())
}

pub async fn check_transaction_in_db(
    pool: &Pool<Postgres>,
    trx_hash: &String,
) -> Result<Option<ResultTransaction>, sqlx::Error> {
    let transaction = sqlx::query_as::<_, ResultTransaction>(
        r#"
        SELECT
            transaction_hash,
            transaction_status,
            block_hash,
            block_number,
            "from",
            "to",
            contract_address,
            logs_count,
            input,
            value
        FROM transactions
        WHERE transaction_hash = $1
        "#,
    )
    .bind(trx_hash) // Bind the transaction hash parameter
    .fetch_optional(pool) // Fetch the result as an Option<ResultTransaction>
    .await?;

    Ok(transaction)
}

pub async fn create_trx_table(pool: &Pool<Postgres>) {
    // Ensure the "transactions" table exists
    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS transactions (
            transaction_hash TEXT PRIMARY KEY,
            transaction_status SMALLINT NOT NULL,
            block_hash TEXT NOT NULL,
            block_number INTEGER NOT NULL,
            "from" TEXT NOT NULL,
            "to" TEXT,
            contract_address TEXT,
            logs_count INTEGER NOT NULL,
            input TEXT NOT NULL,
            value TEXT NOT NULL
        )
        "#,
    )
    .execute(pool)
    .await
    .unwrap();
}

pub async fn fetch_all_transactions(
    pool: &Pool<Postgres>,
) -> Result<Vec<ResultTransaction>, sqlx::Error> {
    let transaction = sqlx::query_as::<_, ResultTransaction>(
        r#"
        SELECT
            transaction_hash,
            transaction_status,
            block_hash,
            block_number,
            "from",
            "to",
            contract_address,
            logs_count,
            input,
            value
        FROM transactions
        "#,
    )
    .fetch_all(pool) // Fetch the result as an Option<ResultTransaction>
    .await?;

    Ok(transaction)
}

pub async fn fetch_matching_transactions(
    pool: &Pool<Postgres>,
    transaction_hashes: Vec<String>,
) -> Result<Vec<ResultTransaction>, sqlx::Error> {
    // Convert Vec<String> to a format suitable for SQL
    let placeholders: Vec<String> = transaction_hashes
        .iter()
        .enumerate()
        .map(|(i, _)| format!("${}", i + 1))
        .collect();

    let query = format!(
        r#"
        SELECT * FROM transactions
        WHERE transaction_hash = ANY(ARRAY[{}]::TEXT[])
        "#,
        placeholders.join(", ")
    );

    // bind all trx hashes
    let mut query_builder = sqlx::query_as::<_, ResultTransaction>(&query);
    for hash in transaction_hashes {
        query_builder = query_builder.bind(hash);
    }

    let transactions = query_builder.fetch_all(pool).await?;

    Ok(transactions)
}
