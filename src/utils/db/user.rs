use sqlx::{Pool, Postgres};

use crate::utils::structs::auth::{AuthPayload, DbUser};

pub async fn fetch_user(pool: &Pool<Postgres>, user_name: &String) -> Result<DbUser, sqlx::Error> {
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
    .fetch_one(pool)
    .await?;

    Ok(user)
}

pub async fn login_user(
    pool: &Pool<Postgres>,
    user_name: &String,
    password: &String,
) -> Result<DbUser, sqlx::Error> {
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
    .fetch_one(pool)
    .await?;

    Ok(user)
}

pub async fn create_users_table(pool: &Pool<Postgres>) {
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
    .execute(pool)
    .await
    .unwrap();
}

pub async fn seed_users(pool: &Pool<Postgres>) -> Result<(), sqlx::Error> {
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
    .execute(pool)
    .await?;

    Ok(())
}

pub async fn save_user_trx(pool: &Pool<Postgres>, trx: &String, user_name: &String) {
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
    .execute(pool)
    .await
    .unwrap();
}
