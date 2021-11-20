pub mod action;
mod controllers;
mod login;
pub mod models;
pub mod utils;

use actix_web::web;
use log::debug;

pub fn init(cfg: &mut web::ServiceConfig) {
    debug!("Loading Login Controllers");
    cfg.service(login::login)
        .service(login::me)
        .service(login::one_time_password)
        .service(login::one_time_password_create);
    debug!("Loading User Controllers");
    cfg.service(controllers::change_property)
        .service(controllers::submit_user);
}
