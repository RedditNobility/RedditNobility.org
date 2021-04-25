use std::time::{Duration, Instant};


use actix::prelude::*;
use actix_files as fs;
use actix_web::{middleware, get, post, web, App, Error, HttpRequest, HttpResponse, HttpServer, http};
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
use crate::siteerror::SiteError::DBError;

fn api_validate(header_map: &HeaderMap, level: Level, &conn: MysqlConnection) -> Result<bool, Box<dyn WebsiteError>> {
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
            let result = action::get_client_key_by_id(id.into(), conn);
            if result.is_err() {
                return Err(Box::new(SiteError::DBError(result.err().unwrap())));
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

fn get_user_by_header(header_map: &HeaderMap, &conn: MysqlConnection) -> Result<Option<User>, Box<dyn WebsiteError>> {
    let option = header_map.get("Authorization");
    if option.is_none() {
        Ok(None)
    }
    let x = option.unwrap().to_str();
    if x.is_err {}
    let header = x.unwrap().to_string();

    let split = header.split(" ").collect::<Vec<&str>>();
    let option = split.get(0);
    if option.is_none() {
        Ok(None)
    }
    let value = split.get(1);
    if value.is_none() {
        Ok(None)
    }
    let value = value.unwrap().to_string();
    let key = option.unwrap().to_string();
    if key.eq("Bearer") {
        let result = action::get_user_from_auth_token(value, conn);
        if result.is_err() {
            return Err(Box::new(SiteError::DBError(result.err().unwrap())));
        }
        return Ok(result.unwrap());
    }
    Ok(None)
}

#[get("/api/user/{user}")]
pub async fn get_user(pool: web::Data<DbPool>, web::Path((user)): web::Path<( String)>, r: HttpRequest) -> HttpResponse {
    let conn = pool.get().expect("couldn't get db connection from pool");
    let result = api_validate(r.headers(), Level::Client, **&conn);
    if result.is_err() {
        return result.err().unwrap().api_error();
    }
    if !result.unwrap() {
        Ok(UserError::NotFound.api_error());
    }
    let result1 = action::get_user_by_name(user, &conn);
    if result1.is_err() {
        return DBError(result1.err().unwrap()).api_error();
    }
    let result1 = result1.unwrap();
    if result1.is_none() {
        let mut map = HashMap::<String, Value>::new();
        map.insert("error".parse()?, Value::from(Number::from(404)));
        return HttpResponse::NotFound().content_type("application/json").body(serde_json::to_string(&map).unwrap());
    }
    let user = result1.unwrap();
    HttpResponse::Ok().content_type("application/json").body(serde_json::to_string(&user).unwrap())
}

#[derive(Serialize, Deserialize)]
pub struct UserSuggest {
    pub username: String,
}

#[post("/api/user/submit")]
pub async fn submit_user(pool: web::Data<DbPool>, suggest: web::Form<UserSuggest>, r: HttpRequest) -> HttpResponse {
    let conn = pool.get().expect("couldn't get db connection from pool");
    let result = api_validate(r.headers(), Level::Client, **&conn);
    if result.is_err() {
        return result.err().unwrap().api_error();
    }
    if !result.unwrap() {
        Ok(UserError::NotFound.api_error());
    }
    let result1 = action::get_user_by_name(suggest.username.clone(), &conn);
    if result1.is_err() {
        return DBError(result1.err().unwrap()).api_error();
    }
    let mut map = HashMap::<String, Value>::new();

    let mut user = result1.unwrap();
    map.insert("success".to_string(), Value::from("true"));
    if user.is_none() {
        utils::quick_add(suggest.username.clone(), &conn);
        let result1 = action::get_user_by_name(suggest.username.clone(), &conn);
        if result1.is_err() {
            return DBError(result1.err().unwrap()).api_error();
        }
        user = result1.unwrap();
        map.insert("status".to_string(), Value::from("added"));
    } else {
        map.insert("status".to_string(), Value::from("already_added"));
    }
    HttpResponse::Ok().content_type("application/json").body(serde_json::to_string(&map).unwrap())
}

#[derive(Serialize, Deserialize)]
pub struct APILoginRequest {
    pub username: String,
    pub password: Option<String>,
}

#[post("/api/login")]
pub async fn user_login(pool: web::Data<DbPool>, login: web::Form<APILoginRequest>, rr: web::Data<Arc<Mutex<RedditRoyalty>>>, r: HttpRequest) -> HttpResponse {
    let conn = pool.get().expect("couldn't get db connection from pool");
    let result = action::get_user_by_name(login.username.clone(), &conn);
    if result.is_err() {
        return SiteError::DBError(result.err().unwrap()).api_error();
    }
    let user = result.unwrap();
    if user.is_none() {
        return UserError::NotAuthorized.api_error();
    }
    let user = user.unwrap();
    if login.password.is_none() {
        utils::send_login(&user, **&conn, rr.clone());
        let mut map = HashMap::<String, Value>::new();
        map.insert("success".parse()?, Value::from(true));
        map.insert("status".parse()?, Value::from("SENT"));
        return HttpResponse::Ok().content_type("application/json").body(serde_json::to_string(&map).unwrap());
    }
    if verify(&login.password, &user.password).unwrap() {
        let x = utils::create_token(&user, &conn);
        let mut map = HashMap::<String, Value>::new();
        map.insert("success".parse()?, Value::from(true));
        map.insert("status".parse()?, Value::from("AUTHORIZED"));
        map.insert("token".parse()?, Value::from(x.unwrap().token.clone()));
        return HttpResponse::Ok().content_type("application/json").body(serde_json::to_string(&map).unwrap());
    }
    return UserError::NotAuthorized.api_error();
}

#[post("/api/validate/key")]
pub async fn validate_key(pool: web::Data<DbPool>, r: HttpRequest) -> Result<HttpResponse, Error> {
    let conn = pool.get().expect("couldn't get db connection from pool");
    let result = api_validate(r.headers(), Level::User, **&conn);
    if !result.unwrap() {
        Ok(UserError::NotAuthorized.api_error());
    }
    let mut map = HashMap::<String, Value>::new();
    map.insert("success".parse()?, Value::from(true));
    return Ok(HttpResponse::Ok().content_type("application/json").body(serde_json::to_string(&map).unwrap()));
}