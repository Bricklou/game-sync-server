use std::{fs, io::Write, path::Path};

use actix_multipart::{Field, Multipart, MultipartError};
use actix_web::web;
use futures_util::TryStreamExt;

use super::app_paths::cache_path;

pub async fn get_image_field(
    payload: &mut Multipart,
    name: &str,
) -> Result<Option<Field>, MultipartError> {
    // iterate over multipart stream
    while let Some(f) = payload.try_next().await? {
        if f.name() == name && f.content_type().essence_str() == "image/jpeg" {
            return Ok(Some(f));
        }
    }
    Ok(None)
}

pub async fn save_to_cache(field: &mut Field) -> Result<String, anyhow::Error> {
    let timestamp = chrono::offset::Local::now().timestamp();
    let file_path = Path::new(cache_path().as_str()).join(timestamp.to_string());

    let p = file_path.clone();
    let mut f = web::block(move || fs::File::create(&p)).await??;

    // Field turn in stream of Bytes object
    while let Some(chunk) = field.try_next().await? {
        f = web::block(move || f.write_all(&chunk).map(|_| f)).await??;
    }

    f.sync_all()?;

    Ok(file_path.into_os_string().into_string().unwrap())
}
