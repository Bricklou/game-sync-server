use std::process::exit;

use crate::app::utils::hash::Hash;
use log::{error, info};

pub type DbPool = sqlx::postgres::PgPool;

pub async fn connection_builder() -> Result<DbPool, sqlx::Error> {
    let connectspec = dotenv::var("DATABASE_URL").expect("DATABASE_URL is not set");
    sqlx::postgres::PgPool::connect(&connectspec).await
}

pub async fn init_database(pool: &DbPool) -> Result<(), sqlx::Error> {
    info!("Checking for admin user");

    let rec = sqlx::query!(
        r#"
        SELECT id, username FROM users WHERE id = 0;
        "#
    )
    .fetch_optional(pool)
    .await?;

    let account_exists = match rec {
        None => false,
        Some(_) => true,
    };

    if !account_exists {
        let hash = Hash::new();
        let hashed_password = hash.hash_password(&"admin".to_string());

        if let Err(_) = hashed_password {
            error!("Failed to hash password for admin user");
            exit(1);
        }

        let hashed_password = hashed_password.unwrap();

        sqlx::query!(
            r#"
            INSERT INTO users(id,username,password) VALUES(0,$1,$2);
        "#,
            "admin",
            hashed_password
        )
        .execute(pool)
        .await?;

        info!(r#"Admin user created with identifiers "admin"/"admin""#)
    }

    Ok(())
}
