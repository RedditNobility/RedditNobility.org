use std::time::{Duration, Instant};


use actix::prelude::*;
use actix_files as fs;
use actix_web::{middleware, get, post, web, App, Error, HttpRequest, HttpResponse, HttpServer};
use actix_web_actors::ws;
use crate::{DbPool, RedditRoyalty, action, utils};
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
use crate::models::{User, Level};
use new_rawr::structures::submission::Submission;
use new_rawr::traits::{Votable, Content};
use rand::Rng;
use rand::distributions::Alphanumeric;
use serde_json::Value;
use actix_web::web::Form;
use std::collections::HashMap;
use serde_json::Number;
use actix_web::error::ParseError::Header;
use actix_web::http::{HeaderName, HeaderMap};
use crate::websiteerror::WebsiteError;
use crate::siteerror::SiteError;
use bcrypt::verify;
use crate::usererror::UserError;

fn api_validate(header_map: &HeaderMap, level: Level, &conn: MysqlConnection) -> Result<bool, dyn WebsiteError> {
    let option = header_map.get("Authorization");
    if option.is_none() {
        Ok(false)
    }
    let x = option.unwrap().to_str();
    if x.is_err {}
    let header = x.unwrap().to_string();

    let split = header.split(" ").collect::<Vec<&str>>();
    let option = split.get(0);
    if option.is_none() {
        Ok(false)
    }
    let value = split.get(1);
    if value.is_none() {
        Ok(false)
    }
    let value = value.unwrap().to_string();
    let key = option.unwrap().to_string();
    if key.eq("Basic") {
        if level == Level::Client {
            let x1 = value.split(":").collect::<Vec<&str>>();
            let id = x1.get(0);
            if id.is_none() {
                Ok(false)
            }
            let id = id.unwrap();
            let key = x1.get(1);
            if key.is_none() {
                Ok(false)
            }
            let key = key.unwrap();
            let result = action::get_client_key_by_id(id, conn);
            if result.is_err() {
                return Err(SiteError::DBError(result.err().unwrap()));
            }
            let client = result.unwrap();
            if client.is_none() {
                Ok(false)
            }
            Ok(verify(&key, &client.unwrap().api_key).unwrap())
        } else {
            Ok(false)
        }
    } else if key.eq("Bearer") {
        if level == Level::Client {
            Ok(false)
        }
        return utils::is_authorized(key, level, conn);
    }
    Ok(false)
}

#[get("/api/user/{user}")]
pub async fn user(pool: web::Data<DbPool>, web::Path((user)): web::Path<( String)>, r: HttpRequest) -> Result<HttpResponse, Error> {
    let conn = pool.get().expect("couldn't get db connection from pool");
    let result = api_validate(r.headers(), Level::Client, **&conn);
    if !result.unwrap() {
        Ok(UserError::NotFound.api_error());
    }
    let header = option.unwrap().to_str().unwrap().to_string();
    let split = header.split(" ").collect::<Vec<&str>>();
    split.get(0).unwrap();
    println!("{}", header.to_str().unwrap());
    let result1 = action::get_user_by_name(user, &conn);
    if result1.is_err() {}
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
    if result1.is_err() {}
    if result1.is_none() {
        let mut map = HashMap::<String, Value>::new();
        map.insert("error".parse()?, Value::from(Number::from(404)));
        return Ok(HttpResponse::NotFound().content_type("application/json").body(serde_json::to_string(&map).unwrap()));
    }
    let fuser = result1.unwrap();
    Ok(HttpResponse::Ok().content_type("application/json").body(serde_json::to_string(&fuser).unwrap()))
}