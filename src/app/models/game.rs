use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use serde_with::rust::string_empty_as_none;
use validator::Validate;

use crate::db::DbPool;

#[derive(Serialize, Deserialize)]
pub struct Game {
    pub id: i32,
    pub name: String,
    pub link: Option<String>,
    pub author: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Deserialize, Serialize, Validate)]
pub struct GameCreate {
    // Since `required` doesn't accept messages, we need to bypass by asking
    // a message having at least one charater
    #[validate(length(min = 1, message = "This field is required"))]
    pub name: String,
    #[validate(url(message = "This field is not a valid URL"))]
    #[serde(with = "string_empty_as_none")]
    pub link: Option<String>,
    #[validate(length(min = 1, message = "This field is required"))]
    pub author: String,
}

impl Game {
    pub async fn create(game_obj: &GameCreate, pool: &DbPool) -> Result<Self, sqlx::Error> {
        let game = sqlx::query!(
            r#"
            INSERT INTO games (name, link, author) VALUES ($1, $2, $3) RETURNING id, name, link, author, created_at, updated_at;
            "#,
            game_obj.name,
            game_obj.link,
            game_obj.author
        )
        .fetch_one(pool)
        .await?;

        Ok(Self {
            id: game.id,
            name: game.name,
            link: game.link,
            author: game.author,
            created_at: game.created_at,
            updated_at: game.updated_at,
        })
    }

    pub async fn check_if_exist(game_obj: &GameCreate, pool: &DbPool) -> Result<bool, sqlx::Error> {
        let game = sqlx::query!(
            r#"
            SELECT id, name, link, author, created_at, updated_at FROM games WHERE name = $1;
            "#,
            game_obj.name
        )
        .fetch_optional(pool)
        .await?;

        Ok(game.is_some())
    }
}
