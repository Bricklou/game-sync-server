extern crate log;
extern crate pretty_env_logger;
extern crate tera;

mod app;
mod bootstrap;
mod db;

use actix_cors::Cors;
use actix_session::{storage::RedisSessionStore, SessionMiddleware};
use actix_web::{
    cookie::Key,
    middleware::{Compress, Logger},
    web::Data,
    App, HttpServer,
};
use std::process::exit;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    bootstrap::boot()
        .await
        .map_err(|e| {
            println!("Failed to boot the application: {:?}", e);
            exit(1);
        })
        .unwrap();

    let server_data = db::connection_builder().await.unwrap();
    let _migrate_result = sqlx::migrate!("./migrations")
        .run(&server_data)
        .await
        .unwrap();

    let server_host = dotenvy::var("SERVER_HOST").unwrap_or("0.0.0.0".to_string());
    let server_port = dotenvy::var("SERVER_PORT").unwrap_or("8080".to_string());
    let server_location = server_host + ":" + &server_port;

    let redis_connection_string = std::env::var("REDIS_URL").expect("REDIS_URL not set");
    let app_secret = dotenvy::var("APP_SECRET").expect("APP_SECRET not set");

    let store = RedisSessionStore::new(redis_connection_string)
        .await
        .unwrap();

    match db::init_database(&server_data).await {
        Err(e) => {
            log::error!("Failed to create admin user: {}", e);
            exit(1);
        }
        Ok(_) => {}
    }

    HttpServer::new(move || {
        let cors_origin = dotenvy::var("CORS_ORIGIN").unwrap_or_else(|_| "".to_string());
        let mut cors = Cors::default()
            .allow_any_method()
            .supports_credentials()
            .max_age(3600);

        if cors_origin.is_empty() {
            println!("CORS_ORIGIN not set, CORS disabled");
            cors = cors.allow_any_origin();
        } else {
            cors = cors.allowed_origin(cors_origin.as_str());
        }

        App::new()
            .app_data(Data::new(server_data.clone()))
            .wrap(cors)
            .wrap(SessionMiddleware::new(
                store.clone(),
                Key::from(app_secret.as_bytes()),
            ))
            .wrap(Logger::default())
            .wrap(Compress::default())
            .configure(app::setup_data)
            .configure(app::setup_templates)
            .configure(app::register_urls)
    })
    .bind(&server_location)?
    .run()
    .await
}
