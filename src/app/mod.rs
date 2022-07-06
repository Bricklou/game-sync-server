use actix_web::web;
use tera::Tera;

use self::utils::hash::Hash;

mod api;
mod models;
mod panel;
mod types;
pub mod utils;

pub fn register_urls(cfg: &mut web::ServiceConfig) {
    api::urls::register_urls(cfg);
    panel::urls::register_urls(cfg);
}

pub fn setup_templates(cfg: &mut web::ServiceConfig) {
    let tera = Tera::new(concat!(env!("CARGO_MANIFEST_DIR"), "/views/**/*")).unwrap();

    cfg.app_data(web::Data::new(tera));
}

pub fn setup_data(cfg: &mut web::ServiceConfig) {
    let hash = Hash::new();

    cfg.app_data(web::Data::new(hash));
}
