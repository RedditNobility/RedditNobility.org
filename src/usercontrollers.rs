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
use actix::prelude::*;
use std::borrow::BorrowMut;
use std::collections::HashMap;
use actix_web::web::Form;
use crate::siteerror::SiteError;
use crate::websiteerror::WebsiteError;
use bcrypt::verify;
use crate::recaptcha::validate;

#[derive(Deserialize)]
pub struct SubmitUser {
    pub username: String,
}

#[get("/submit")]
pub async fn index(pool: web::Data<DbPool>, tera: web::Data<Tera>, req: HttpRequest) -> Result<HttpResponse, Error> {
    let mut ctx = tera::Context::new();


    let result = tera.get_ref().render("index.html", &ctx);
    if result.is_err() {
        let error = result.err().unwrap();
        return Ok(HttpResponse::InternalServerError().finish());
    }
    Ok(HttpResponse::Ok().content_type("text/html").body(&result.unwrap()))
}


#[get("/login")]
pub async fn get_login(pool: web::Data<DbPool>, tera: web::Data<Tera>, req: HttpRequest) -> Result<HttpResponse, Error> {
    let mut ctx = tera::Context::new();
    let conn = pool.get().expect("couldn't get db connection from pool");

    let result = tera.get_ref().render("login.html", &ctx);
    if result.is_err() {
        let error = result.err().unwrap();
        return Ok(HttpResponse::InternalServerError().finish());
    }
    Ok(HttpResponse::Ok().content_type("text/html").body(&result.unwrap()))
}

#[derive(Serialize, Deserialize)]
pub struct LoginRequest {
    pub username: String,
    pub password: Option<String>,
    pub recaptcha: Option<String>,
}

#[post("/login/post")]
pub async fn post_login(pool: web::Data<DbPool>, tera: web::Data<Tera>, request: HttpRequest, rr: web::Data<Arc<Mutex<RedditRoyalty>>>, form: Form<LoginRequest>) -> HttpResponse {
    if form.recaptcha.is_none() {
        return HttpResponse::Found().header(http::header::LOCATION, "/login?status=BAD_RECAPTCHA").finish().into_body();
    } else {
        let string1 = form.recaptcha.as_ref().unwrap().clone();
        let result1 = std::env::var("RECAPTCHA_SECRET").unwrap();
        let url = std::env::var("URL").unwrap();
        let validate1 = validate(result1, string1, url).await;
        if validate1.is_err() {
            return validate1.err().unwrap().site_error(tera);
        }
        if !validate1.unwrap() {
            return HttpResponse::Found().header(http::header::LOCATION, "/login?status=BAD_RECAPTCHA").finish().into_body();
        }
    }
    let conn = pool.get().expect("couldn't get db connection from pool");
    let result = action::get_user_by_name(form.username.clone(), &conn);
    if result.is_err() {
        return SiteError::DBError(result.err().unwrap()).site_error(tera);
    }
    let user = result.unwrap();
    if user.is_none() {
        return HttpResponse::Found().header(http::header::LOCATION, "/login?status=NOT_FOUND").finish().into_body();
    }
    let user = user.unwrap();
    if form.password.is_none() {
        utils::send_login(&user, &conn, rr.clone());
        return HttpResponse::Found().header(http::header::LOCATION, "/login?status=LOGIN_SENT").finish().into_body();
    } else {
        let string = form.password.as_ref().unwrap();
        if verify(string, &user.password).unwrap() {
            return HttpResponse::Found().header("Location", "/").cookie(http::Cookie::build("auth_token", utils::create_token(&user, &conn).unwrap().token.clone())
                .domain(request.headers().get("HOST").unwrap().to_str().unwrap())
                .path("/")
                .secure(true)
                .http_only(true)
                .finish()).finish().into_body();
        }
    }
    return HttpResponse::Found().header("Location", "/login?status=NOT_FOUND").finish().into_body();
}

#[derive(Serialize, Deserialize)]
pub struct KeyLogin {
    pub key: String,
}

#[get("/login/key")]
pub async fn key_login(pool: web::Data<DbPool>, tera: web::Data<Tera>, request: HttpRequest, rr: web::Data<Arc<Mutex<RedditRoyalty>>>, form: Form<KeyLogin>) -> HttpResponse {
    let conn = pool.get().expect("couldn't get db connection from pool");
    let result = action::get_auth_token(form.key.clone(), &conn);
    if result.is_err() {
        return SiteError::DBError(result.err().unwrap()).site_error(tera);
    }
    let token = result.unwrap();
    if token.is_none() {
        return HttpResponse::Found().header(http::header::LOCATION, "/login?status=NOT_FOUND").finish().into_body();
    }
    let token = token.unwrap();
    return HttpResponse::Found().header("Location", "/").cookie(http::Cookie::build("auth_token", token.token.clone())
        .domain(request.headers().get("HOST").unwrap().to_str().unwrap())
        .path("/")
        .secure(true)
        .http_only(true)
        .finish()).finish().into_body();
}

