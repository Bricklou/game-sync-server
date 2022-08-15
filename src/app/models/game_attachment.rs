use serde::{Deserialize, Serialize};
use sqlx::FromRow;

use crate::{app::utils::errors::ServiceError, db::DbPool};

use super::game::Game;

#[derive(Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "varchar", rename_all = "snake_case")]
pub enum GameAttachmentType {
    #[serde(rename = "screenshot")]
    Screenshot,
    #[serde(rename = "logo")]
    Logo,
}

#[derive(Serialize, Deserialize, FromRow)]
pub struct GameAttachment {
    pub id: i32,
    #[sqlx()]
    pub attachment_type: GameAttachmentType,
    pub path: String,
    #[serde(skip_serializing)]
    pub game_id: i32,
}

impl GameAttachment {
    pub async fn get_attachments(pool: &DbPool, game: &Game) -> Result<Vec<Self>, sqlx::Error> {
        let attachments = sqlx::query_as!(
            GameAttachment,
            r#"
            SELECT
                id,
                attachment_type as "attachment_type: GameAttachmentType",
                path,
                game_id
            FROM game_attachment
            WHERE
                game_id = $1;
            "#,
            game.id
        )
        .fetch_all(pool)
        .await?;

        Ok(attachments)
    }

    pub async fn get_logo(pool: &DbPool, game: &Game) -> Result<Option<Self>, sqlx::Error> {
        let attachments = sqlx::query_as!(
            GameAttachment,
            r#"
            SELECT
                id,
                attachment_type as "attachment_type: GameAttachmentType",
                path,
                game_id
            FROM game_attachment
            WHERE
                attachment_type = 'logo'
                AND game_id = $1
            LIMIT 1;
            "#,
            game.id
        )
        .fetch_optional(pool)
        .await?;

        Ok(attachments)
    }

    pub async fn get_screenshots(pool: &DbPool, game: &Game) -> Result<Vec<Self>, sqlx::Error> {
        let attachments = sqlx::query_as!(
            GameAttachment,
            r#"
            SELECT
                id,
                attachment_type as "attachment_type: GameAttachmentType",
                path,
                game_id
            FROM game_attachment
            WHERE
                attachment_type = 'screenshot'
                AND game_id = $1;
            "#,
            game.id
        )
        .fetch_all(pool)
        .await?;

        Ok(attachments)
    }

    pub async fn update_logo(pool: &DbPool, game: &Game, path: &str) -> Result<Self, ServiceError> {
        let logo = sqlx::query_as!(
            GameAttachment,
            r#"
            INSERT INTO game_attachment(attachment_type,path,game_id)
                VALUES ('logo',$1,$2)
            ON CONFLICT (attachment_type,game_id)
                DO UPDATE SET path = $1
            RETURNING
                id,
                attachment_type as "attachment_type: GameAttachmentType",
                path,
                game_id
            ;
            "#,
            path,
            game.id
        )
        .fetch_one(pool)
        .await?;

        Ok(logo)
    }

    pub fn insert_screenshot(game: &Game, path: &str) -> Result<(), sqlx::Error> {
        Ok(())
    }

    pub fn delete_screenhot(game: &Game, screenshot_id: i32) -> Result<(), sqlx::Error> {
        Ok(())
    }
}
