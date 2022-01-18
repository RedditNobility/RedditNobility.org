mod action;
mod controllers;
mod models;

use actix_web::web;

pub fn init(cfg: &mut web::ServiceConfig) {
    cfg.service(controllers::update_permission)
        .service(controllers::delete_team_member)
        .service(controllers::add_team);
}
