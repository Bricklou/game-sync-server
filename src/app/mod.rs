use actix_web::web;
use actix_web::web::ServiceConfig;
use tera::Tera;

mod api;
mod panel;

pub fn register_urls(cfg: &mut ServiceConfig) {
    panel::register_urls(cfg);
    api::register_urls(cfg);
}

pub fn setup_templates(cfg: &mut ServiceConfig) {
    let tera = Tera::new(concat!(env!("CARGO_MANIFEST_DIR"), "/templates/**/*")).unwrap();

    cfg.app_data(web::Data::new(tera));
}
