use diesel::MysqlConnection;

use actix::prelude::*;
use log::{error, info, warn};
use actix_files as fs;
use actix_web::{middleware, get, post, web, App, Error, HttpRequest, HttpResponse, HttpServer, http};
use crate::{DbPool, RedditRoyalty, action, utils};
use tera::Tera;
use new_rawr::responses::listing::SubmissionData;
use serde::{Serialize, Deserialize};
use diesel::{Connection};
use std::rc::Rc;
use std::sync::{Mutex, Arc};
use std::cell::RefCell;
use crate::schema::users::dsl::created;
use new_rawr::client::RedditClient;
use new_rawr::auth::AnonymousAuthenticator;
use crate::models::{User, Level, Status, ClientKey};
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
use crate::api::apiresponse::{APIResponse, APIError};
use std::str::FromStr;
use crate::action::{get_user_by_name, update_user};
use crate::api::api_validate;
use crate::api::get_user_by_header;


#[derive(Serialize, Deserialize)]
pub struct UserSuggest {
    pub username: String,
}

#[post("/api/submit/user")]
pub async fn submit_user(pool: web::Data<DbPool>, suggest: web::Form<UserSuggest>, r: HttpRequest) -> HttpResponse {
    let conn = pool.get().expect("couldn't get db connection from pool");
    let result = api_validate(r.headers(), Level::User, &conn);
    if result.is_err() {
        return result.err().unwrap().api_error();
    }
    if !result.unwrap() {
        println!("BAD PERM");
        return UserError::NotFound.api_error();
    }
    let result1 = action::get_user_by_name(suggest.username.clone(), &conn);
    if result1.is_err() {
        return DBError(result1.err().unwrap()).api_error();
    }

    let mut user = result1.unwrap();
    if user.is_none() {
        let discoverer = get_user_by_header(&r.headers(), &conn);
        if discoverer.is_err() {
            return discoverer.err().unwrap().api_error();
        }
        let client = RedditClient::new("RoboticMonarch by u/KingTuxWH", AnonymousAuthenticator::new());
        let user1 = client.user(suggest.username.as_str());
        let result2 = user1.about();
        if result2.is_err() {
            let response = APIResponse::<String> {
                success: true,
                data: Some("NOT_FOUND".parse().unwrap()),
            };
            return HttpResponse::Ok().content_type("application/json").body(serde_json::to_string(&response).unwrap());
        }
        utils::quick_add(suggest.username.clone(), discoverer.unwrap().unwrap().username.clone(), &conn);
        let result1 = action::get_user_by_name(suggest.username.clone(), &conn);
        if result1.is_err() {
            return DBError(result1.err().unwrap()).api_error();
        }
        user = result1.unwrap();
        let response = APIResponse::<String> {
            success: true,
            data: Some("ADDED".parse().unwrap()),
        };
        return HttpResponse::Ok().content_type("application/json").body(serde_json::to_string(&response).unwrap());
    } else {
        let response = APIResponse::<String> {
            success: true,
            data: Some("ALREADY_ADDED".parse().unwrap()),
        };
        return HttpResponse::Ok().content_type("application/json").body(serde_json::to_string(&response).unwrap());
    }
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
        utils::send_login(&user, &conn, &rr.clone().lock().unwrap().reddit);
        let mut map = HashMap::<String, Value>::new();
        map.insert("success".to_string(), Value::from(true));
        map.insert("status".to_string(), Value::from("SENT"));
        return HttpResponse::Ok().content_type("application/json").body(serde_json::to_string(&map).unwrap());
    } else {
        let string = login.password.as_ref().unwrap();
        if verify(&string, &user.password).unwrap() {
            let x = utils::create_token(&user, &conn);
            let mut map = HashMap::<String, Value>::new();
            map.insert("success".to_string(), Value::from(true));
            map.insert("status".to_string(), Value::from("AUTHORIZED"));
            map.insert("token".to_string(), Value::from(x.unwrap().token.clone()));
            return HttpResponse::Ok().content_type("application/json").body(serde_json::to_string(&map).unwrap());
        }
    }

    return UserError::NotAuthorized.api_error();
}

#[post("/api/validate/key")]
pub async fn validate_key(pool: web::Data<DbPool>, r: HttpRequest) -> Result<HttpResponse, Error> {
    let conn = pool.get().expect("couldn't get db connection from pool");
    let result = api_validate(r.headers(), Level::User, &conn);
    if !result.unwrap() {
        return Ok(UserError::NotAuthorized.api_error());
    }
    let mut map = HashMap::<String, Value>::new();
    map.insert("success".to_string(), Value::from(true));
    return Ok(HttpResponse::Ok().content_type("application/json").body(serde_json::to_string(&map).unwrap()));
}
#[derive(Deserialize)]
pub struct ChangeRequest {
    pub username: String,
    pub property: String,
    pub value: String,
}

#[post("/api/change/user")]
pub async fn change_property(pool: web::Data<DbPool>, request: web::Form<ChangeRequest>, r: HttpRequest) -> HttpResponse {
    let conn = pool.get().expect("couldn't get db connection from pool");
    let result = api_validate(r.headers(), Level::User, &conn);
    if result.is_err() {
        return result.err().unwrap().api_error();
    }
    if !result.unwrap() {
        return UserError::NotAuthorized.api_error();
    }
    println!("Reverse3");
    let user = get_user_by_header(&r.headers(), &conn);
    if user.is_err() {
        return user.err().unwrap().api_error();
    }
    let user = user.unwrap().unwrap();
    if !user.username.eq(&request.username) {
        println!("Reverse2");
        let result = api_validate(r.headers(), Level::Moderator, &conn);
        if result.is_err() {
            return result.err().unwrap().api_error();
        }
        if !result.unwrap() {
            return UserError::NotAuthorized.api_error();
        }
    }
    let modifying_user = get_user_by_name(request.username.clone(), &conn);
    if modifying_user.is_err() {
        return SiteError::DBError(modifying_user.err().unwrap()).api_error();
    }
    let modifying_user = modifying_user.unwrap();
    if modifying_user.is_none() {
        return UserError::NotFound.api_error();
    }
    let mut modifying_user = modifying_user.unwrap();
    println!("Reverse1");
    if request.property.eq("avatar") {
        modifying_user.properties.set_avatar(request.value.clone());
    } else if request.property.eq("description") {
        modifying_user.properties.set_description(request.value.clone());
    } else {
        return UserError::InvalidRequest.api_error();
    }
    let result1 = update_user(&modifying_user, &conn);
    if result1.is_err() {
        return SiteError::DBError(result1.err().unwrap()).api_error();
    }
    let response = APIResponse::<User> {
        success: true,
        data: None,
    };
    return HttpResponse::Ok().content_type("application/json").body(serde_json::to_string(&response).unwrap());
}
