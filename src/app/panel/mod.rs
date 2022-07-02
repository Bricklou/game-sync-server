use actix_web::web::{self, ServiceConfig};

mod views;

pub fn register_urls(cfg: &mut ServiceConfig) {
    cfg.service(web::resource("/").route(web::get().to(views::index)));
}
