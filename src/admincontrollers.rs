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

#[get("/admin")]
pub async fn admin(pool: web::Data<DbPool>, session: Session, tera: web::Data<Tera>, req: HttpRequest) -> HttpResponse {
    let mut ctx = tera::Context::new();
    let conn = pool.get();
    if conn.is_err() {
        //Return Error
    }
    let conn = conn.unwrap();
    let token = session.get("auth_token");
    if token.is_err() {
        //Return err
    }
    let token: Option<String> = token.unwrap();
    if token.is_none() {
        //No Auth
    }
    let token = token.unwrap();
    if !utils::is_authorized(token, Level::Admin, &conn){
        //No Auth
    }
    let result = tera.get_ref().render("admin.html", &ctx);
    if result.is_err() {
        let error = result.err().unwrap();
        return HttpResponse::InternalServerError().into();
    }
    HttpResponse::Ok().content_type("text/html").body(&result.unwrap())
}


#[derive(Deserialize)]
pub struct CreateMod {
    pub username: String,
    pub password: String,

}

#[post("/admin/user/del")]
pub async fn admin_del_user(pool: web::Data<DbPool>, tera: web::Data<Tera>, session: Session, req: HttpRequest) -> HttpResponse {
    HttpResponse::Found().header("Location", "/admin").finish()
}

#[post("/admin/user/create")]
pub async fn admin_create_user(pool: web::Data<DbPool>, tera: web::Data<Tera>, session: Session, form: Form<CreateMod>, req: HttpRequest) -> HttpResponse {
    let conn = pool.get().expect("couldn't get db connection from pool");
    println!("1");
    let result1 = action::get_moderators(&conn);
    if (result1.is_err()) {
        println!("Hey");
        return HttpResponse::InternalServerError().finish();
    }
    let result1 = result1.unwrap();
    let mut approved = false;

    println!("1");

    if !result1.is_empty() {
        let moderator = session.get("moderator");
        let option = moderator.unwrap();
        if option.is_some() {
            let value: String = option.unwrap();
            for x in result1 {
                if x.username.eq(&value) {
                    if x.admin {
                        approved = true;
                    }
                }
            }
        }
    } else {
        approved = true;
    }
    println!("1");
    if !approved {
        return HttpResponse::Unauthorized().header("Location", "/").finish();
    }
    let result1 = action::get_moderators(&conn);
    let result1 = result1.unwrap();

    for x in result1 {
        if x.username.eq(&form.username) {
            return HttpResponse::Found().header("Location", "/admin").finish();
        }
    }
    println!("1");
    let moderator1 = Moderator {
        id: 0,
        username: form.username.clone(),
        password: hash(&form.password.clone(), DEFAULT_COST).unwrap(),
        admin: false,
    };
    action::add_moderator(&moderator1, &conn);
    println!("1");

    HttpResponse::Found().header("Location", "/admin").finish()
}