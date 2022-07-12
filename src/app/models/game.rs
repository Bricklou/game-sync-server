use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
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
    #[validate(length(min = 1, message = "This field is required"))]
    #[validate(url(message = "This field is not a valid URL"))]
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
}