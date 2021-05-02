use std::time::{Duration, Instant};


use actix::prelude::*;
use actix_files as fs;
use actix_web::{middleware, get, post, web, App, Error, HttpRequest, HttpResponse, HttpServer, HttpMessage};
use crate::{DbPool, RedditRoyalty, action, utils};
use tera::Tera;
use new_rawr::responses::listing::SubmissionData;
use serde::{Serialize, Deserialize};
use diesel::{MysqlConnection, Connection};
use std::rc::Rc;
use std::sync::{Mutex, Arc};
use std::cell::RefCell;
use crate::schema::users::dsl::created;
use new_rawr::client::RedditClient;
use new_rawr::auth::AnonymousAuthenticator;
use crate::models::{User, Level};
use new_rawr::structures::submission::Submission;
use new_rawr::traits::{Votable, Content};
use rand::Rng;
use rand::distributions::Alphanumeric;
use serde_json::Value;
use actix::prelude::*;
use std::borrow::BorrowMut;
use std::collections::HashMap;
use actix_web::web::Form;
use actix_web::cookie::Cookie;

#[get("/admin")]
pub async fn admin(pool: web::Data<DbPool>, tera: web::Data<Tera>, req: HttpRequest) -> HttpResponse {
    let mut ctx = tera::Context::new();
    let conn = pool.get();
    if conn.is_err() {
        //Return Error
    }
    let conn = conn.unwrap();

    let token: Option<Cookie> = req.cookie("auth_token");

    if token.is_none() {
        //No Auth
    }
    let token = token.unwrap().value().to_string();
    if !utils::is_authorized(token, Level::Admin, &conn).unwrap() {
        //No Auth
    }
    let result = tera.get_ref().render("admin.html", &ctx);
    if result.is_err() {
        let error = result.err().unwrap();
        return HttpResponse::InternalServerError().into();
    }
    HttpResponse::Ok().content_type("text/html").body(&result.unwrap())
}


