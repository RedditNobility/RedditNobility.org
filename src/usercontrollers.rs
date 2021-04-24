use std::time::{Duration, Instant};


use actix::prelude::*;
use actix_files as fs;
use actix_web::{middleware, get, web, App, Error, HttpRequest, HttpResponse, HttpServer};
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
use actix::prelude::*;
use std::borrow::BorrowMut;
use std::collections::HashMap;
use actix_web::web::Form;
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

#[post("/submit/post  ")]
pub async fn submit(pool: web::Data<DbPool>, tera: web::Data<Tera>, req: HttpRequest, form: Form<SubmitUser>) -> Result<HttpResponse, Error> {
    let mut ctx = tera::Context::new();
    let conn = pool.get().expect("couldn't get db connection from pool");
    if !form.username.is_empty() {
        if action::get_fuser(form.username.clone(), &conn).unwrap().is_some() {
            ctx.insert("already_exists", &true);
        } else {
            ctx.insert("success", &true);
            let fuser = User {
                id: 0,
                username: form.username.clone(),
                moderator: "".to_string(),
                status: "Found".to_string(),
                created: SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_millis() as i64,
            };
            action::add_new_fuser(&fuser, &conn);
        }
    }

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

#[post("/login")]
pub async fn post_login(pool: web::Data<DbPool>, tera: web::Data<Tera>, session: Session, req: HttpRequest, form: Form<CreateMod>) -> HttpResponse {
    let conn = pool.get().expect("couldn't get db connection from pool");
    let result = action::get_moderator(form.username.clone(), &conn).unwrap();
    if result.is_none() {
        return HttpResponse::Found().header("Location", "/login").finish().into_body();
    }
    let moderator = result.unwrap();
    if verify(&form.password, &moderator.password).unwrap() {
        println!("Worked!");
        let result1 = session.set("moderator", moderator.username);
        return HttpResponse::Found().header("Location", "/moderator").finish().into_body();
    }
    return HttpResponse::Found().header("Location", "/login").finish().into_body();
}


