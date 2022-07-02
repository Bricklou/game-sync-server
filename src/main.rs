#[macro_use]
extern crate pretty_env_logger;
extern crate log;
extern crate tera;

mod app;
mod db;

use actix_web::middleware::{Compress, Logger};
use actix_web::web::Data;
use actix_web::{App, HttpServer};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "actix_web=info");
    dotenv::dotenv().ok();
    pretty_env_logger::init();

    let server_data = db::connection_builder().await.unwrap();
    let _migrate_result = sqlx::migrate!("./migrations")
        .run(&server_data)
        .await
        .unwrap();

    let server_host = dotenv::var("SERVER_HOST").unwrap_or("0.0.0.0".to_string());
    let server_port = dotenv::var("SERVER_PORT").unwrap_or("8080".to_string());
    let server_location = server_host + ":" + &server_port;

    HttpServer::new(move || {
        App::new()
            .wrap(Logger::default())
            .wrap(Compress::default())
            .app_data(Data::new(server_data.clone()))
            .configure(app::setup_templates)
            .configure(app::register_urls)
    })
    .bind(&server_location)?
    .run()
    .await
}
