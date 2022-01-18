pub mod action;
mod controllers;
mod login;
pub mod models;
mod team_controllers;
pub mod title;
pub mod utils;

use actix_web::web;
use controllers::*;
use log::debug;

pub fn init(cfg: &mut web::ServiceConfig) {
    debug!("Loading Login Controllers");
    cfg.service(login::login)
        .service(login::me)
        .service(login::one_time_password)
        .service(login::one_time_password_create);
    debug!("Loading User Controllers");
    cfg.service(change_property)
        .service(submit_user)
        .service(update_password);
    debug!("Loading Team Controllers");
    cfg.service(team_controllers::get_team)
        .service(team_controllers::get_team_as_list);
}
