use actix_web::web;

pub mod controllers;

pub fn init(cfg: &mut web::ServiceConfig) {
    cfg.service(controllers::index)
        .service(controllers::moderator)
        .service(controllers::install)
        .service(controllers::review)
        .service(controllers::review_with)
        .service(controllers::user)
        .service(controllers::about)
        .service(controllers::login);
}
