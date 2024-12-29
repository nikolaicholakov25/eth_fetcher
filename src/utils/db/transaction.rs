use sqlx::{Executor, Postgres};

use crate::utils::structs::transaction::ResultTransaction;

pub async fn save_transaction_to_db<'c, E>(
    executor: E,
    trx: &ResultTransaction,
) -> Result<(), sqlx::Error>
where
    E: Executor<'c, Database = Postgres>,
{
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
    .execute(executor)
    .await?;

    Ok(())
}

pub async fn check_transaction_in_db<'c, E>(
    executor: E,
    trx_hash: &String,
) -> Result<Option<ResultTransaction>, sqlx::Error>
where
    E: Executor<'c, Database = Postgres>,
{
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
    .fetch_optional(executor) // Fetch the result as an Option<ResultTransaction>
    .await?;

    Ok(transaction)
}

pub async fn create_trx_table<'c, E>(executor: E) -> Result<(), sqlx::Error>
where
    E: Executor<'c, Database = Postgres>,
{
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
    .execute(executor)
    .await
    .unwrap();

    Ok(())
}

pub async fn fetch_all_transactions<'c, E>(
    executor: E,
) -> Result<Vec<ResultTransaction>, sqlx::Error>
where
    E: Executor<'c, Database = Postgres>,
{
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
    .fetch_all(executor) // Fetch the result as an Option<ResultTransaction>
    .await?;

    Ok(transaction)
}

pub async fn fetch_matching_transactions<'c, E>(
    pool: E,
    transaction_hashes: Vec<String>,
) -> Result<Vec<ResultTransaction>, sqlx::Error>
where
    E: Executor<'c, Database = Postgres>,
{
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

#[cfg(test)]
mod tests {
    use crate::load_config;
    use std::env;

    use super::*;
    use sqlx::Pool;

    async fn fixture_pool() -> Pool<Postgres> {
        load_config();

        let database_url = env::var("DB_CONNECTION_URL").expect("DB_CONNECTION_URL must be set");
        let pool = Pool::<Postgres>::connect(&database_url)
            .await
            .expect("Failed to connect to the database");

        pool
    }

    #[tokio::test]
    async fn test_save_transaction_to_db() {
        let pool = fixture_pool().await;
        let mut db_trx = pool.begin().await.expect("Pool transaction failed");

        create_trx_table(&mut *db_trx)
            .await
            .expect("Failed to create TRANSACTIONS table");

        let trx = ResultTransaction {
            transaction_hash: "hash1".to_string(),
            transaction_status: 1,
            block_hash: "blockhash1".to_string(),
            block_number: 100,
            from: "from_address".to_string(),
            to: Some("to_address".to_string()),
            contract_address: None,
            logs_count: 10,
            input: "input_data".to_string(),
            value: "1000".to_string(),
        };

        save_transaction_to_db(&mut *db_trx, &trx)
            .await
            .expect("Failed to save trx to db");

        let fetched_trx = check_transaction_in_db(&mut *db_trx, &trx.transaction_hash)
            .await
            .unwrap();

        assert!(fetched_trx.is_some());
        assert_eq!(fetched_trx.unwrap().transaction_hash, trx.transaction_hash);

        db_trx
            .rollback()
            .await
            .expect("Failed to rollback test trx");
    }

    #[tokio::test]
    async fn test_fetch_all_transactions() {
        let pool = fixture_pool().await;
        let mut db_trx = pool.begin().await.expect("Pool transaction failed");

        create_trx_table(&mut *db_trx)
            .await
            .expect("Failed to create TRANSACTIONS table");

        let trx1 = ResultTransaction {
            transaction_hash: "hash1".to_string(),
            transaction_status: 1,
            block_hash: "blockhash1".to_string(),
            block_number: 100,
            from: "from_address".to_string(),
            to: Some("to_address".to_string()),
            contract_address: None,
            logs_count: 10,
            input: "input_data".to_string(),
            value: "1000".to_string(),
        };

        let trx2 = ResultTransaction {
            transaction_hash: "hash2".to_string(),
            transaction_status: 2,
            block_hash: "blockhash2".to_string(),
            block_number: 200,
            from: "from_address2".to_string(),
            to: None,
            contract_address: None,
            logs_count: 5,
            input: "input_data2".to_string(),
            value: "2000".to_string(),
        };

        save_transaction_to_db(&mut *db_trx, &trx1)
            .await
            .expect("Failed to save db_trx 1 in db");
        save_transaction_to_db(&mut *db_trx, &trx2)
            .await
            .expect("Failed to save db_trx 2 in db");

        let transactions = fetch_all_transactions(&mut *db_trx)
            .await
            .expect("Failed to fetch all transactions");

        assert_eq!(transactions.contains(&trx1), true);
        assert_eq!(transactions.contains(&trx2), true);
    }

    #[tokio::test]
    async fn test_fetch_matching_transactions() {
        let pool = fixture_pool().await;
        let mut db_trx = pool.begin().await.expect("Pool transaction failed");

        create_trx_table(&mut *db_trx)
            .await
            .expect("Failed to create TRANSACTIONS table");

        let trx1 = ResultTransaction {
            transaction_hash: "hash1".to_string(),
            transaction_status: 1,
            block_hash: "blockhash1".to_string(),
            block_number: 100,
            from: "from_address".to_string(),
            to: Some("to_address".to_string()),
            contract_address: None,
            logs_count: 10,
            input: "input_data".to_string(),
            value: "1000".to_string(),
        };

        let trx2 = ResultTransaction {
            transaction_hash: "hash2".to_string(),
            transaction_status: 2,
            block_hash: "blockhash2".to_string(),
            block_number: 200,
            from: "from_address2".to_string(),
            to: None,
            contract_address: None,
            logs_count: 5,
            input: "input_data2".to_string(),
            value: "2000".to_string(),
        };

        save_transaction_to_db(&mut *db_trx, &trx1)
            .await
            .expect("Failed to save db_trx 1 in db");
        save_transaction_to_db(&mut *db_trx, &trx2)
            .await
            .expect("Failed to save db_trx 2 in db");

        let transaction_hashes = vec![trx1.transaction_hash.clone(), trx2.transaction_hash.clone()];
        let transactions = fetch_matching_transactions(&mut *db_trx, transaction_hashes)
            .await
            .expect("Failed to fetch matching transactions");

        assert_eq!(transactions.len(), 2);
        assert_eq!(transactions[0].transaction_hash, trx1.transaction_hash);
        assert_eq!(transactions[1].transaction_hash, trx2.transaction_hash);
    }
}
