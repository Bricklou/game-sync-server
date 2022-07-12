use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};

use crate::{
    app::utils::{errors::ServiceError, hash::Hash},
    db::DbPool,
};

#[derive(Serialize, Deserialize)]
pub struct User {
    pub id: i32,
    pub username: String,
    #[serde(skip_serializing)]
    pub password: Option<String>,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

pub struct UserRegister {
    pub username: String,
    pub password: String,
    pub confirmation: String,
}

impl User {
    pub async fn login(
        pool: &DbPool,
        hash: &Hash,
        username: &String,
        password: &String,
    ) -> Result<Option<Self>, ServiceError> {
        let rec = sqlx::query!(
            r#"
            SELECT id, username, password, created_at, updated_at FROM users WHERE username = $1;
           "#,
            username
        )
        .fetch_optional(pool)
        .await?;

        if let Some(user) = rec {
            let pass_check = hash.verify_password(password.to_string(), user.password)?;

            if !pass_check {
                return Ok(None);
            }

            let u = Self {
                id: user.id,
                username: user.username,
                password: None,
                created_at: user.created_at,
                updated_at: user.updated_at,
            };
            Ok(Some(u))
        } else {
            Ok(None)
        }
    }

    pub async fn check_if_exists(pool: &DbPool, username: String) -> Result<bool, ServiceError> {
        let rec = sqlx::query!(
            r#"
            SELECT count(id) as u FROM users WHERE username = $1;
            "#,
            username
        )
        .fetch_one(pool)
        .await?;

        Ok(rec.u > Some(0))
    }

    pub async fn get_from_id(pool: &DbPool, id: i32) -> Result<Option<User>, ServiceError> {
        let rec = sqlx::query!(
            r#"
            SELECT * FROM users WHERE id = $1;
            "#,
            id
        )
        .fetch_optional(pool)
        .await?;

        Ok(rec.map(|r| User {
            id: r.id,
            username: r.username,
            password: None,
            created_at: r.created_at,
            updated_at: r.updated_at,
        }))
    }

    pub async fn register(
        user_obj: &UserRegister,
        pool: &DbPool,
        hash: &Hash,
    ) -> Result<Self, ServiceError> {
        let hashed_password = hash.hash_password(&user_obj.password)?;

        let rec = sqlx::query!(
            r#"
            INSERT INTO users (username,password) VALUES ($1,$2) RETURNING id, created_at, updated_at;
            "#,
            user_obj.username,
            hashed_password
        ).fetch_one(pool).await?;

        Ok(Self {
            id: rec.id,
            username: user_obj.username.clone(),
            password: None,
            created_at: rec.created_at,
            updated_at: rec.updated_at,
        })
    }
}
