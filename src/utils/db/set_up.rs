use sqlx::{Pool, Postgres};

use super::{
    transaction::create_trx_table,
    user::{create_users_table, seed_users},
};

pub async fn init_db(pool: &Pool<Postgres>) -> Result<(), sqlx::Error> {
    // Ensure the "transactions" table exists
    create_trx_table(pool)
        .await
        .expect("Failed to create table TRANSACTIONS");
    println!("TRANSACTIONS table created");

    // Ensure the "users" table exists
    create_users_table(pool)
        .await
        .expect("Failed to create table USERS");
    println!("USERS table created");

    // Seed the default users
    seed_users(pool).await.expect("Failed to seed table USERS");
    println!("USERS table seeded");

    Ok(())
}
