use std::time::{Duration, Instant};


use actix::prelude::*;
use actix_files as fs;
use actix_web::{middleware, get, post,web, App, Error, HttpRequest, HttpResponse, HttpServer};
use actix_web_actors::ws;
use crate::{DbPool, RedditRoyalty, action};
use tera::Tera;
use new_rawr::responses::listing::SubmissionData;
use serde::{Serialize, Deserialize};
use diesel::{MysqlConnection, Connection};
use actix_session::{Session, CookieSession};
use std::rc::Rc;
use std::sync::{Mutex, Arc};
use std::cell::RefCell;
use actix_web_actors::ws::{CloseReason, CloseCode};
use crate::schema::users::dsl::created;
use new_rawr::client::RedditClient;
use new_rawr::auth::AnonymousAuthenticator;
use crate::models::User;
use new_rawr::structures::submission::Submission;
use new_rawr::traits::{Votable, Content};
use rand::Rng;
use rand::distributions::Alphanumeric;
use serde_json::Value;
use actix_web::web::Form;
use std::collections::HashMap;
use serde_json::Number;

#[get("/api/user/{user}")]
pub async fn user(pool: web::Data<DbPool>, web::Path((user)): web::Path<( String)>, r: HttpRequest) -> Result<HttpResponse, Error> {
    let conn = pool.get().expect("couldn't get db connection from pool");

    let result1 = action::get_user_by_name(user, &conn);
    if result1.is_err(){

    }
    if result1.is_none() {
        let mut map = HashMap::<String, Value>::new();
        map.insert("error".parse()?, Value::from(Number::from(404)));
        return Ok(HttpResponse::NotFound().content_type("application/json").body(serde_json::to_string(&map).unwrap()));
    }
    let fuser = result1.unwrap();
    Ok(HttpResponse::Ok().content_type("application/json").body(serde_json::to_string(&fuser).unwrap()))
}
#[post("/api/login")]
pub async fn user_login(pool: web::Data<DbPool>, web::Path((user)): web::Path<( String)>, r: HttpRequest) -> Result<HttpResponse, Error> {
    let conn = pool.get().expect("couldn't get db connection from pool");

    let result1 = action::get_user_by_name(user, &conn);
    if result1.is_err(){

    }
    if result1.is_none() {
        let mut map = HashMap::<String, Value>::new();
        map.insert("error".parse()?, Value::from(Number::from(404)));
        return Ok(HttpResponse::NotFound().content_type("application/json").body(serde_json::to_string(&map).unwrap()));
    }
    let fuser = result1.unwrap();
    Ok(HttpResponse::Ok().content_type("application/json").body(serde_json::to_string(&fuser).unwrap()))
}