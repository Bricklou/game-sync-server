use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use serde_with::rust::string_empty_as_none;
use validator::Validate;

use crate::{
    app::utils::paginator::{PaginatorObject, PaginatorResponse},
    db::DbPool,
};

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

    pub async fn get_games(
        pool: &DbPool,
        pagination: &PaginatorObject,
    ) -> Result<PaginatorResponse<Game>, sqlx::Error> {
        let search = pagination.search.as_ref().map(|s| s.as_str()).unwrap_or("");

        log::debug!("Search input: {}", search);
        let mut games = sqlx::query!(
            r#"
            SELECT * FROM games
            WHERE
                $1 = ''
                OR (name ILIKE '%'||$1||'%' OR author ILIKE '%'||$1||'%')
            ORDER BY created_at DESC
            OFFSET $3 ROWS FETCH NEXT $2 ROWS ONLY;
            "#,
            search,
            pagination.per_page,
            pagination.page - 1
        )
        .fetch_all(pool)
        .await?;

        games.sort_by(|a, b| a.name.cmp(&b.name));

        let games: Vec<Game> = games
            .into_iter()
            .map(|game| Self {
                id: game.id,
                name: game.name,
                link: game.link,
                author: game.author,
                created_at: game.created_at,
                updated_at: game.updated_at,
            })
            .collect();

        Ok(PaginatorResponse::<Game> {
            page: pagination.page,
            per_page: pagination.per_page,
            total: games.len() as i64,
            total_pages: (games.len() as i64 / pagination.per_page as i64).max(1),
            items: games,
        })
    }
}
