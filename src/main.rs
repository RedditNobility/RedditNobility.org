#[macro_use]
extern crate diesel;
#[macro_use]
extern crate diesel_migrations;
extern crate dotenv;
extern crate bcrypt;
use log::{error, info, warn};
use dotenv::dotenv;
use actix_web::{get, middleware, post, web, App, Error, HttpResponse, HttpServer, HttpRequest, error, Responder};
use std::{env, thread};
use diesel::prelude::*;
use diesel::r2d2::{self, ConnectionManager};
use tera::Tera;
use actix_files as fs;
use actix_web::web::{Form, BytesMut};
use crate::models::{User};
use serde::{Serialize, Deserialize};
use core::time;
use new_rawr::client::RedditClient;
use new_rawr::auth::{AnonymousAuthenticator, PasswordAuthenticator};
use new_rawr::options::ListingOptions;
use new_rawr::structures::submission::Submission;
use new_rawr::traits::{Content, Commentable, Votable};
use std::fs::File;
use std::io::{BufReader, BufRead};
use std::path::Path;
use actix_session::{CookieSession, Session};
use actix_web::http::header::LOCATION;
use actix_web::http::StatusCode;
use actix_web::body::Body;
use bcrypt::{DEFAULT_COST, hash, verify};
use std::collections::HashMap;
use std::rc::Rc;
use std::cell::RefCell;
use std::time::{SystemTime, UNIX_EPOCH};
use crate::controllers::{RedditPost, RedditUser};
use std::sync::{Mutex, Arc};
use actix_multipart_derive::MultipartForm;
use openssl::ssl::{SslAcceptor, SslMethod, SslFiletype};

pub mod models;
pub mod schema;
mod action;
mod controllers;
mod api;
mod morecontrollers;
mod utils;
mod siteerror;
mod admincontrollers;
mod usercontrollers;
mod moderatorcontrollers;
mod usererror;
mod websiteerror;
mod apiresponse;

type DbPool = r2d2::Pool<ConnectionManager<MysqlConnection>>;

pub struct RedditRoyalty {
    pub active_keys: HashMap<String, i64>,
    pub reddit: RedditClient,
}

impl RedditRoyalty {
    fn new(client: RedditClient) -> RedditRoyalty {
        RedditRoyalty {
            active_keys: HashMap::new(),
            reddit: client,
        }
    }
    pub fn add_key(&mut self, key: String, moderator: i64) {
        self.active_keys.insert(key, moderator);
    }

    pub fn drop_key(&mut self, key: String) {
        self.active_keys.remove(&*key);
    }

    pub fn is_key_valid(&self, key: String) -> bool {
        self.active_keys.contains_key(&*key)
    }
}
embed_migrations!();
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("{}", is_valid("KingTux".parse().unwrap()));
    println!("{}", is_valid("Lord_Darth_Dan".parse().unwrap()));
    println!("{}", is_valid("LordZorthan".parse().unwrap()));
    println!("{}", is_valid("PrinceCow".parse().unwrap()));
    println!("{}", is_valid("PrincessCow".parse().unwrap()));
    println!("{}", is_valid("KingTux".parse().unwrap()));
    std::env::set_var("RUST_LOG", "actix_web=debug");
    log4rs::init_file( Path::new("resources").join("log.yml"), Default::default()).unwrap();
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

    let mut server = HttpServer::new(move || {
        let tera =
            Tera::new(concat!(env!("CARGO_MANIFEST_DIR"), "/templates/**/*")).unwrap();

        App::new()
            .wrap(middleware::Logger::default())
            .wrap(CookieSession::signed(&[0; 32]).secure(false))
            .data(pool.clone()).data(Arc::clone(&reddit_royalty)).data(tera).
            service(fs::Files::new("static", "static").show_files_listing()).
            service(favicon).
            service(controllers::moderator_index).
            service(api::user).
            service(morecontrollers::file_upload).
            service(web::resource("/ws/moderator").route(web::get().to(controllers::ws_index)))
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


fn is_valid(username: String) -> bool {
    let vec = lines_from_file(Path::new("resources").join("names.txt"));
    let string = username.to_lowercase();
    for x in vec {
        if string.contains(&x) {
            return true;
        }
    }
    return false;
}

fn lines_from_file(filename: impl AsRef<Path>) -> Vec<String> {
    let file = File::open(filename).expect("no such file");
    let buf = BufReader::new(file);
    buf.lines()
        .map(|l| l.expect("Could not parse line"))
        .collect()
}


#[get("/favicon.ico")]
async fn favicon() -> actix_web::Result<actix_files::NamedFile> {
    Ok(actix_files::NamedFile::open("static/favicon.ico")?)
}
