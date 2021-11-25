use actix_web::web;

pub mod action;
//TODO pub mod controller;
pub mod settings;
pub mod utils;
pub mod controller;

pub fn init(cfg: &mut web::ServiceConfig) {
      cfg.service(controller::about_setting)
          .service(controller::setting_report)
        .service(controller::update_setting);
}