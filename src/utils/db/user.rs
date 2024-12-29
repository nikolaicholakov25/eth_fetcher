use crate::utils::structs::auth::DbUser;
use sqlx::{Executor, Postgres};

pub async fn fetch_user<'c, E>(executor: E, user_name: &String) -> Result<DbUser, sqlx::Error>
where
    E: Executor<'c, Database = Postgres>,
{
    let user = sqlx::query_as::<_, DbUser>(
        r#"
    SELECT
        name,
        transactions
    FROM users
    WHERE name = $1
    "#,
    )
    .bind(user_name)
    .fetch_one(executor)
    .await?;

    Ok(user)
}

pub async fn login_user<'c, E>(
    executor: E,
    user_name: &String,
    password: &String,
) -> Result<DbUser, sqlx::Error>
where
    E: Executor<'c, Database = Postgres>,
{
    let user = sqlx::query_as::<_, DbUser>(
        r#"
    SELECT
        name,
        transactions
    FROM users
    WHERE name = $1 AND password = $2
    "#,
    )
    .bind(user_name)
    .bind(password)
    .fetch_one(executor)
    .await?;

    Ok(user)
}

pub async fn create_users_table<'c, E>(executor: E) -> Result<(), sqlx::Error>
where
    E: Executor<'c, Database = Postgres>,
{
    // Ensure the "USERS" table exists
    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS users (
            name TEXT PRIMARY KEY,
            password TEXT NOT NULL,
            transactions TEXT[] DEFAULT ARRAY[]::TEXT[]
        )
        "#,
    )
    .execute(executor)
    .await?;

    Ok(())
}

pub async fn seed_users<'c, E>(executor: E) -> Result<(), sqlx::Error>
where
    E: Executor<'c, Database = Postgres>,
{
    sqlx::query(
        r#"
        INSERT INTO users (name, password) VALUES
        ($1, $2),
        ($3, $4),
        ($5, $6),
        ($7, $8)
        ON CONFLICT (name) DO NOTHING
        "#,
    )
    .bind("alice")
    .bind("alice")
    .bind("bob")
    .bind("bob")
    .bind("carol")
    .bind("carol")
    .bind("dave")
    .bind("dave")
    .execute(executor)
    .await?;

    Ok(())
}

pub async fn save_user_trx<'c, E>(
    executor: E,
    trx: &String,
    user_name: &String,
) -> Result<(), sqlx::Error>
where
    E: Executor<'c, Database = Postgres>,
{
    sqlx::query(
        r#"
        UPDATE users
        SET transactions = ARRAY_APPEND(transactions, $1)
        WHERE name = $2 AND NOT $3 = ANY(transactions)
        "#,
    )
    .bind(trx)
    .bind(user_name)
    .bind(trx)
    .execute(executor)
    .await?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::config::load_config;

    use super::*;
    use sqlx::{Pool, Postgres};
    use std::env;

    async fn fixture_pool() -> Pool<Postgres> {
        load_config();

        let database_url = env::var("DB_CONNECTION_URL").expect("DB_CONNECTION_URL must be set");
        let pool = Pool::<Postgres>::connect(&database_url)
            .await
            .expect("Failed to connect to the database");

        pool
    }

    #[tokio::test]
    async fn test_fetch_user() {
        let pool = fixture_pool().await;
        let mut db_trx = pool.begin().await.expect("Pool transaction failed");

        create_users_table(&mut *db_trx)
            .await
            .expect("Failed to create USERS table");
        seed_users(&mut *db_trx)
            .await
            .expect("Failed to seed USERS table");

        let user_name = "alice".to_string();
        let user = fetch_user(&mut *db_trx, &user_name)
            .await
            .expect("Failed to fetch user");

        assert_eq!(user.name, "alice", "Fetched user name should match");
        db_trx
            .rollback()
            .await
            .expect("Failed to rollback test trx");
    }

    #[tokio::test]
    async fn test_login_user_success() {
        let pool = fixture_pool().await;
        let mut db_trx = pool.begin().await.expect("Pool transaction failed");

        create_users_table(&mut *db_trx)
            .await
            .expect("Failed to create USERS table");
        seed_users(&mut *db_trx)
            .await
            .expect("Failed to seed USERS table");

        let user_name = "bob".to_string();
        let password = "bob".to_string();
        let user = login_user(&mut *db_trx, &user_name, &password)
            .await
            .expect("Failed to log in user");

        assert_eq!(user.name, "bob", "Logged in user name should match");
        db_trx
            .rollback()
            .await
            .expect("Failed to rollback test trx");
    }

    #[tokio::test]
    async fn test_login_user_failure() {
        let pool = fixture_pool().await;
        let mut db_trx = pool.begin().await.expect("Pool transaction failed");

        create_users_table(&mut *db_trx)
            .await
            .expect("Failed to create USERS table");
        seed_users(&mut *db_trx)
            .await
            .expect("Failed to seed USERS table");

        let user_name = "bob".to_string();
        let password = "wrong_password".to_string();
        let result = login_user(&mut *db_trx, &user_name, &password).await;

        assert!(result.is_err(), "Login with incorrect password should fail");
        db_trx
            .rollback()
            .await
            .expect("Failed to rollback test trx");
    }

    #[tokio::test]
    async fn test_save_user_trx() {
        let pool = fixture_pool().await;
        let mut db_trx = pool.begin().await.expect("Pool transaction failed");

        create_users_table(&mut *db_trx)
            .await
            .expect("Failed to create USERS table");
        seed_users(&mut *db_trx)
            .await
            .expect("Failed to seed USERS table");

        let user_name = "bob".to_string();
        let test_transaction_name: String = "test_trx".to_string();

        // save test_trx
        save_user_trx(&mut *db_trx, &test_transaction_name, &user_name)
            .await
            .expect("Failed to save user transaction");

        // try fetching for test_trx
        let fetched_user = fetch_user(&mut *db_trx, &user_name)
            .await
            .expect("Failed fetching user");

        assert!(fetched_user.transactions.contains(&test_transaction_name));

        db_trx
            .rollback()
            .await
            .expect("Failed to rollback test trx");
    }
}
