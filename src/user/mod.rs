pub mod models;
pub mod action;
mod utils;
mod login;
mod controllers;

use actix_web::web;

pub fn init(cfg: &mut web::ServiceConfig) {}
