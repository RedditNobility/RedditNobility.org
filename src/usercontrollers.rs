use std::time::{Duration, Instant};


use actix::prelude::*;
use actix_files as fs;
use actix_web::{middleware, get, post, web, App, Error, HttpRequest, HttpResponse, HttpServer, http, HttpMessage};
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
use crate::models::{User, Level, Status};
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
use actix_web::cookie::SameSite;
use actix_web::http::header::LOCATION;

#[derive(Deserialize)]
pub struct SubmitUser {
    pub username: String,
}

#[get("/submit")]
pub async fn submit(pool: web::Data<DbPool>, tera: web::Data<Tera>, req: HttpRequest) -> HttpResponse {
    let mut ctx = tera::Context::new();
    let conn = pool.get().expect("couldn't get db connection from pool");

    let option1 = req.cookie("auth_token");
    if option1.is_some() {
        let result1 = action::get_user_from_auth_token(option1.unwrap().value().to_string(), &conn);
        if result1.is_err() {
            return SiteError::DBError(result1.err().unwrap()).site_error(tera);
        }
        let option2 = result1.unwrap();
        if option2.is_none() {
            //Handle need new login
            println!("not Found User");
        } else {
            println!("Hey");
            ctx.insert("user", &option2.unwrap())
        }
    }
    let result = tera.get_ref().render("submit.html", &ctx);
    if result.is_err() {
        let error = result.err().unwrap();
        return HttpResponse::InternalServerError().finish();
    }
    return HttpResponse::Ok().content_type("text/html").body(&result.unwrap());
}


#[get("/login")]
pub async fn get_login(pool: web::Data<DbPool>, tera: web::Data<Tera>, req: HttpRequest) -> Result<HttpResponse, Error> {
    let mut ctx = tera::Context::new();
    ctx.insert("recaptcha_pub", std::env::var("RECAPTCHA_PUB").unwrap().as_str());
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
    #[serde(rename = "g-recaptcha-response")]
    pub g_recaptcha_response: Option<String>,
}

#[post("/login/post")]
pub async fn post_login(pool: web::Data<DbPool>, tera: web::Data<Tera>, request: HttpRequest, rr: web::Data<Arc<Mutex<RedditRoyalty>>>, form: web::Form<LoginRequest>) -> HttpResponse {
    if form.g_recaptcha_response.is_none() {
        println!("Nothing");
        //return HttpResponse::Found().header(http::header::LOCATION, "/login?status=BAD_RECAPTCHA").finish().into_body();
    } else {
        println!("Something");
        let string1 = form.g_recaptcha_response.as_ref().unwrap().clone();
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
    if user.status != Status::Approved{
        return HttpResponse::Found().header(http::header::LOCATION, "/login?status=NOT_FOUND").finish().into_body();
    }
    if form.password.is_some() && !form.password.as_ref().unwrap().is_empty() {
        let string = form.password.as_ref().unwrap();
        if verify(string, &user.password).unwrap() {
            let x = request.headers().get("HOST").unwrap().to_str().unwrap();
            println!("{}", &x);
            return HttpResponse::Found().header(LOCATION, "/").cookie(http::Cookie::build("auth_token", utils::create_token(&user, &conn).unwrap().token.clone())
                .path("/")
                .secure(true).same_site(SameSite::None).max_age(time::Duration::weeks(1))
                .http_only(false)
                .finish()).finish().into_body();
        } else {
            return HttpResponse::Found().header("Location", "/login?status=NOT_FOUND").finish().into_body();
        }
    }
    utils::send_login(&user, &conn, &rr.try_lock().unwrap().reddit);
    return HttpResponse::Found().header(http::header::LOCATION, "/login?status=LOGIN_SENT").finish().into_body();
}

#[derive(Serialize, Deserialize)]
pub struct KeyLogin {
    pub token: String,
}

#[get("/login/key")]
pub async fn key_login(pool: web::Data<DbPool>, tera: web::Data<Tera>, request: HttpRequest, rr: web::Data<Arc<Mutex<RedditRoyalty>>>, form: web::Query<KeyLogin>) -> HttpResponse {
    let conn = pool.get().expect("couldn't get db connection from pool");
    let result = action::get_auth_token(form.token.clone(), &conn);
    if result.is_err() {
        return SiteError::DBError(result.err().unwrap()).site_error(tera);
    }
    let token = result.unwrap();
    if token.is_none() {
        return HttpResponse::Found().header(http::header::LOCATION, "/login?status=NOT_FOUND").finish().into_body();
    }
    let token = token.unwrap();
    return HttpResponse::Found().header(LOCATION, "/").cookie(http::Cookie::build("auth_token", token.token.clone())
        .path("/")
        .secure(true).same_site(SameSite::None).max_age(time::Duration::weeks(1))
        .http_only(false)
        .finish()).finish().into_body();
}

