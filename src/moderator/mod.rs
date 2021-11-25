pub mod action;
mod controllers;

use actix_web::web;

pub fn init(cfg: &mut web::ServiceConfig) {
    cfg.service(controllers::review_user_update)
        .service(controllers::review_user)
        .service(controllers::user_page)
        .service(controllers::user_stats)
        .service(controllers::system_stats)
        .service(controllers::moderator_update_properties);
}
