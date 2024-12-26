use sqlx::{Pool, Postgres};

use super::{
    transaction::create_trx_table,
    user::{create_users_table, seed_users},
};

pub async fn init_db(pool: &Pool<Postgres>) -> Result<(), sqlx::Error> {
    // Ensure the "transactions" table exists
    create_trx_table(pool).await;

    // Ensure the "users" table exists
    create_users_table(pool).await;

    // Seed the default users
    seed_users(pool).await.unwrap();

    Ok(())
}
