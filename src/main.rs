#[macro_use]
extern crate diesel;
#[macro_use]
extern crate diesel_migrations;
extern crate dotenv;
extern crate bcrypt;
extern crate strum_macros;
extern crate strum;

use core::time;
use std::{env, thread};
use std::cell::RefCell;
use std::collections::{BTreeMap, HashMap};
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::ops::Sub;
use std::path::Path;
use std::rc::Rc;
use std::str::FromStr;
use std::sync::{Arc, Mutex};
use std::thread::sleep;
use std::time::{SystemTime, UNIX_EPOCH};

use actix_files as fs;
use actix_web::{App, Error, error, get, http, HttpMessage, HttpRequest, HttpResponse, HttpServer, middleware, post, Responder, web};
use actix_web::body::Body;
use actix_web::http::header::LOCATION;
use actix_web::http::StatusCode;
use actix_web::web::{BytesMut, Form};
use bcrypt::{DEFAULT_COST, hash, verify};
use chrono::{DateTime, Duration, Local};
use diesel::prelude::*;
use diesel::r2d2::{self, ConnectionManager};
use diesel_migrations::name;
use dotenv::dotenv;
use log::{error, info, warn};
use new_rawr::auth::{AnonymousAuthenticator, PasswordAuthenticator};
use new_rawr::client::RedditClient;
use new_rawr::options::ListingOptions;
use new_rawr::structures::submission::Submission;
use new_rawr::traits::{Commentable, Content, Votable};
use openssl::ssl::{SslAcceptor, SslFiletype, SslMethod};
use serde::{Deserialize, Serialize};
use tera::{from_value, Function, Tera, Value};

use crate::models::{Level, Setting, Status, User, UserProperties};
use crate::siteerror::SiteError;
use crate::usererror::UserError;
use crate::websiteerror::WebsiteError;

pub mod models;
pub mod schema;
mod action;
mod utils;
mod siteerror;
mod usererror;
mod websiteerror;
mod recaptcha;
mod api;
mod controllers;

type DbPool = r2d2::Pool<ConnectionManager<MysqlConnection>>;

pub struct RedditRoyalty {
    pub users_being_worked_on: HashMap<i64, DateTime<Local>>,
    pub reddit: RedditClient,
}

impl RedditRoyalty {
    fn new(client: RedditClient) -> RedditRoyalty {
        RedditRoyalty {
            users_being_worked_on: HashMap::new(),
            reddit: client,
        }
    }
    pub fn add_id(&mut self, id: i64) {
        self.users_being_worked_on.insert(id, Local::now());
    }
    fn remove_id(&mut self, i: &i64) {
        self.users_being_worked_on.remove(i);
    }
}

fn url(args: &HashMap<String, Value>) -> Result<tera::Value, tera::Error> {
    let option = args.get("path");
    return if option.is_some() {
        let x = option.unwrap().as_str().unwrap();
        let x1 = std::env::var("URL").unwrap();
        let string = format!("{}/{}", x1, x);
        println!("{}", &string);
        let result = tera::Value::from(&*string);
        Ok(result)
    } else {
        Err(tera::Error::from("Missing Param Tera".to_string()))
    };
}
embed_migrations!();
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "actix_web=trace");
    std::env::set_var("RUST_BACKTRACE", "1");
    log4rs::init_file(Path::new("resources").join("log.yml"), Default::default()).unwrap();
    dotenv::dotenv().ok();
    let connspec = std::env::var("DATABASE_URL").expect("DATABASE_URL");
    let manager = ConnectionManager::<MysqlConnection>::new(connspec);
    let pool = r2d2::Pool::builder()
        .build(manager)
        .expect("Failed to create pool.");
    let connection = pool.get().unwrap();
    embedded_migrations::run_with_output(&connection, &mut std::io::stdout());
    info!("Test");

    let arc = PasswordAuthenticator::new(
        std::env::var("CLIENT_KEY").unwrap().as_str(),
        std::env::var("CLIENT_SECRET").unwrap().as_str(),
        std::env::var("REDDIT_USER").unwrap().as_str(),
        std::env::var("PASSWORD").unwrap().as_str());

    let client = RedditClient::new("RedditNobility bot(by u/KingTuxWH)", arc);
    let reddit_royalty = Arc::new(Mutex::new(RedditRoyalty::new(client)));
    let arc2 = reddit_royalty.clone();
    thread::spawn(move || {
        loop {
            {
                let arc1 = arc2.clone();
                let result = arc1.lock();
                if result.is_err() {
                    panic!("The Site Core has been poisoned. Tux you dumb fuck!")
                }
                let mut rr = result.unwrap();
                for x in rr.users_being_worked_on.clone() {
                    let x1: Duration = Local::now().sub(x.1.clone());
                    if x1.num_minutes() > 5 {
                        rr.remove_id(&x.0);
                    }
                }
            }
            sleep(Duration::minutes(5).to_std().unwrap())
        }
    });
    let mut server = HttpServer::new(move || {
        let result1 = Tera::new(concat!(env!("CARGO_MANIFEST_DIR"), "/site/templates/**/*"));
        if result1.is_err() {
            println!("{}", result1.err().unwrap());
            panic!("Unable to create Tera")
        }
        let mut tera =
            result1.unwrap();
        tera.register_function("url", url);

        App::new()
            .wrap(middleware::Logger::default())
            .data(pool.clone()).data(Arc::clone(&reddit_royalty)).data(tera).
            service(favicon).
            service(index).
            service(install).
            service(controllers::user::get_login).
            service(controllers::user::post_login).
            service(controllers::user::key_login).
            service(controllers::user::me).
            service(controllers::user::submit).
            service(controllers::moderator::review_users).
            service(controllers::moderator::user_page).
            service(controllers::moderator::mod_index).
            service(api::admin::change_level).
            service(api::moderator::change_status).
            service(api::moderator::get_user).
            service(api::user::submit_user).
            service(api::user::user_login).
            service(api::user::validate_key).
            service(api::user::change_property).
            service(api::get_moderators).
            service(api::admin::change_level).
            service(api::moderator::next_user).
            service(fs::Files::new("/", "site/static").show_files_listing()).
            service(fs::Files::new("/", "site/node_modules").show_files_listing())
    });
    if std::env::var("PRIVATE_KEY").is_ok() {
        let mut builder = SslAcceptor::mozilla_intermediate(SslMethod::tls()).unwrap();
        builder
            .set_private_key_file(std::env::var("PRIVATE_KEY").unwrap(), SslFiletype::PEM)
            .unwrap();
        builder.set_certificate_chain_file(std::env::var("CERT_KEY").unwrap()).unwrap();

        server.bind_openssl("0.0.0.0:6742", builder)?
            .run()
            .await
    } else {
        server.bind("0.0.0.0:6742")?.run().await
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Moderator {
    pub user: User,
    pub avatar: String,
}

#[get("/")]
pub async fn index(pool: web::Data<DbPool>, tera: web::Data<Tera>, req: HttpRequest) -> HttpResponse {
    let mut ctx = tera::Context::new();
    let conn = pool.get().expect("couldn't get db connection from pool");
    let option = action::get_setting("installed".to_string(), &conn);
    if option.is_err() {
        return SiteError::DBError(option.err().unwrap()).site_error(tera);
    }
    if option.unwrap().is_none() {
        let result = tera.get_ref().render("install.html", &ctx);
        if result.is_err() {
            let error = result.err().unwrap();
            println!("{}", &error);
            return SiteError::TeraError(error).site_error(tera);
        }
        return HttpResponse::Ok().content_type("text/html").body(&result.unwrap());
    }
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
    let mut moderators = Vec::new();
    let vec = action::get_moderators(&conn).unwrap();
    for x in vec {
        let avatar = utils::get_avatar(&x);
        let moderator = Moderator {
            user: x,
            avatar: avatar,
        };
        moderators.push(moderator)
    }
    ctx.insert("moderators", &moderators);
    let result = tera.get_ref().render("index.html", &ctx);
    if result.is_err() {
        return SiteError::TeraError(result.err().unwrap()).site_error(tera);
    }
    return HttpResponse::Ok().content_type("text/html").body(&result.unwrap());
}

#[derive(Serialize, Deserialize)]
pub struct InstallRequest {
    pub username: String,
    pub password: String,
}

#[post("/install")]
pub async fn install(pool: web::Data<DbPool>, form: Form<InstallRequest>, tera: web::Data<Tera>, req: HttpRequest) -> HttpResponse {
    let conn = pool.get().expect("couldn't get db connection from pool");
    let option = action::get_setting("installed".to_string(), &conn);
    if option.is_err() {
        return SiteError::DBError(option.err().unwrap()).site_error(tera);
    }
    if option.unwrap().is_some() {
        return HttpResponse::Found().header(http::header::LOCATION, "/").finish().into_body();
    }
    let properties = UserProperties {
        avatar: None,
        description: Some("OG User".to_string()),
        title: utils::is_valid(form.username.clone()),
    };
    let user = User {
        id: 0,
        username: form.username.clone(),
        password: hash(&form.password.clone(), DEFAULT_COST).unwrap(),
        level: Level::Admin,
        status: Status::Approved,
        status_changed: utils::get_current_time(),
        discoverer: "OG".to_string(),
        moderator: "OG".to_string(),
        properties,
        created: utils::get_current_time(),
    };
    action::add_new_user(&user, &conn).unwrap();
    let st = Setting {
        id: 0,
        setting_key: "installed".to_string(),
        value: "true".to_string(),
        updated: utils::get_current_time(),
    };
    action::add_new_setting(&st, &conn);
    return HttpResponse::Found().header(http::header::LOCATION, "/").finish().into_body();
}

#[get("/favicon.ico")]
async fn favicon() -> actix_web::Result<actix_files::NamedFile> {
    Ok(actix_files::NamedFile::open("site/static/favicon.ico")?)
}
