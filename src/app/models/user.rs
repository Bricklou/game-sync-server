use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};

use crate::{app::utils::hash::Hash, db::DbPool};

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
    ) -> Result<Option<Self>, sqlx::Error> {
        let rec = sqlx::query!(
            r#"
            SELECT id, username, password, created_at, updated_at FROM users WHERE username = $1;
           "#,
            username
        )
        .fetch_optional(pool)
        .await?;

        if let Some(user) = rec {
            let pass_check = hash.verify_password(password.to_string(), user.password);

            let res = match pass_check {
                Err(_) => false,
                Ok(check) => check,
            };

            if !res {
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

    pub async fn check_if_exists(pool: &DbPool, username: String) -> Result<bool, sqlx::Error> {
        let rec = sqlx::query!(
            r#"
            SELECT count(id) as u FROM users WHERE username = $1;
            "#,
            username
        )
        .fetch_one(pool)
        .await?;

        let exists = rec.u > Some(0);

        Ok(exists)
    }

    pub async fn get_from_id(pool: &DbPool, id: i32) -> Result<User, sqlx::Error> {
        let rec = sqlx::query!(
            r#"
            SELECT * FROM users WHERE id = $1;
            "#,
            id
        )
        .fetch_one(pool)
        .await?;

        Ok(Self {
            id: rec.id,
            username: rec.username,
            password: None,
            created_at: rec.created_at,
            updated_at: rec.updated_at,
        })
    }

    pub async fn register(
        user_obj: &UserRegister,
        pool: &DbPool,
        hash: &Hash,
    ) -> Result<Self, sqlx::Error> {
        let hashed_password = hash.hash_password(&user_obj.password);

        let hashed_password = match hashed_password {
            Ok(p) => p,
            Err(_) => "".to_string(),
        };

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
