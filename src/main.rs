#[macro_use]
extern crate diesel;
#[macro_use]
extern crate diesel_migrations;
extern crate dotenv;
extern crate strum;
extern crate strum_macros;

use std::collections::HashMap;
use std::ops::Sub;

use std::sync::{Arc, Mutex};
use std::thread::sleep;

use actix_cors::Cors;
use std::thread;

use actix_files::Files;

use actix_web::web::{Data, PayloadConfig};
use actix_web::{get, middleware, web, App, HttpRequest, HttpServer};

use chrono::{DateTime, Duration, Local};
use diesel::prelude::*;
use diesel::r2d2::{self, ConnectionManager};
use hyper::{Body, Client, Method, Request};

use hyper_tls::HttpsConnector;

use log::{debug, error, info};

use crate::user::models::User;
use nitro_log::config::Config;
use nitro_log::NitroLogger;
use rraw::auth::PasswordAuthenticator;
use rraw::me::Me;

use crate::api_response::{APIResponse, SiteResponse};
use serde::{Deserialize, Serialize};

use crate::user::title::Titles;

use crate::utils::{get_current_time, installed, Resources};

mod admin;
mod api_response;
mod error;
mod frontend;
mod install;
mod moderator;
pub mod schema;
mod settings;
pub mod user;
pub mod utils;

type DbPool = r2d2::Pool<ConnectionManager<MysqlConnection>>;
pub type Database = web::Data<DbPool>;
pub type RN = web::Data<Arc<Mutex<RNCore>>>;
pub type RedditClient = web::Data<Me>;
pub type TitleData = web::Data<Titles>;

pub struct RNCore {
    pub users_being_worked_on: HashMap<i64, DateTime<Local>>,
}

impl RNCore {
    fn new() -> RNCore {
        RNCore {
            users_being_worked_on: HashMap::new(),
        }
    }
    pub fn add_id(&mut self, id: i64) {
        self.users_being_worked_on.insert(id, Local::now());
    }
    fn remove_id(&mut self, i: &i64) {
        self.users_being_worked_on.remove(i);
    }
}

embed_migrations!();
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    if let Err(error) = dotenv::dotenv() {
        println!("Unable to load dotenv {}", error);
        return Ok(());
    }
    let file = match std::env::var("MODE")
        .expect("MODE must be RELEASE OR DEBUG")
        .as_str()
    {
        "DEBUG" => "log-debug.json",
        "RELEASE" => "log-release.json",
        _ => {
            panic!("Must be Release or Debug")
        }
    };
    let config: Config = serde_json::from_str(Resources::file_get_string(file).as_str()).unwrap();
    NitroLogger::load(config, None).unwrap();
    info!("Initializing Database");
    let connspec = std::env::var("DATABASE_URL").expect("DATABASE_URL");
    let manager = ConnectionManager::<MysqlConnection>::new(connspec);
    let pool = r2d2::Pool::builder()
        .build(manager)
        .expect("Failed to create pool.");
    let connection = pool.get().unwrap();
    info!("Checking and running Migrations");
    embedded_migrations::run_with_output(&connection, &mut std::io::stdout()).unwrap();
    std::env::set_var("INSTALLED", "false".to_string());
    info!("Loading Title Info From");
    let https = HttpsConnector::new();
    let hyper = Client::builder().build::<_, hyper::Body>(https);

    let request = Request::builder()
        .method(Method::GET)
        .uri(std::env::var("TITLES").unwrap_or_else(|_| "https://raw.githubusercontent.com/RedditNobility/Titles/master/titles.json".to_string()))
        .body(Body::empty())
        .unwrap();
    let result = hyper.request(request).await;
    if let Err(error) = result {
        error!("Unable to Load Titles File: {}", error);
        return Ok(());
    }
    let response = result.unwrap();
    let bytes = hyper::body::to_bytes(response.into_body()).await;
    if let Err(error) = bytes {
        error!("Unable to Load Titles File: {}", error);
        return Ok(());
    }
    let string = String::from_utf8(bytes.unwrap().to_vec()).unwrap();
    let result1: Result<Titles, serde_json::Error> = serde_json::from_str(string.as_str());
    if let Err(error) = result1 {
        error!("Unable to Load Titles File: {}", error);
        return Ok(());
    }
    let titles_data = result1.unwrap();

    if !installed(&connection).unwrap() {
        info!("Initializing In Installer");
        return HttpServer::new(move || {
            App::new()
                .wrap(
                    Cors::default()
                        .allow_any_header()
                        .allow_any_method()
                        .allow_any_origin(),
                )
                .wrap(middleware::Logger::default())
                .app_data(Data::new(pool.clone()))
                .app_data(Data::new(titles_data.clone()))
                .app_data(PayloadConfig::new(1024 * 1024 * 1024))
                .configure(frontend::init)
                .configure(install::init)
                .service(Files::new("/", std::env::var("SITE_DIR").unwrap()).show_files_listing())
        })
        .workers(2)
        .bind(std::env::var("ADDRESS").unwrap())?
        .run()
        .await;
    }
    info!("Initializing Reddit Controller");
    let arc = PasswordAuthenticator::new(
        std::env::var("CLIENT_KEY").unwrap().as_str(),
        std::env::var("CLIENT_SECRET").unwrap().as_str(),
        std::env::var("REDDIT_USER").unwrap().as_str(),
        std::env::var("PASSWORD").unwrap().as_str(),
    );
    let client = Me::login(arc, "RedditNobility bot(by u/KingTuxWH)".to_string())
        .await
        .unwrap();
    let site_core = Arc::new(Mutex::new(RNCore::new()));
    let reference = site_core.clone();

    thread::spawn(move || {
        let site_core = reference;
        loop {
            info!("Starting Core Cleanup!");
            let result = site_core.lock();
            if result.is_err() {
                panic!("Oh NO!.... The Site Core..... It's Broken")
            }
            let mut rr = result.unwrap();
            for x in rr.users_being_worked_on.clone() {
                let x1: Duration = Local::now().sub(x.1);
                if x1.num_minutes() > 3 {
                    debug!("Removing User {}", &x.0);
                    rr.remove_id(&x.0);
                }
            }
            drop(rr);
            info!("Core Cleanup Over!");
            sleep(Duration::minutes(5).to_std().unwrap())
        }
    });
    info!("Initializing Web Server");

    let server = HttpServer::new(move || {
        App::new()
            .wrap(
                Cors::default()
                    .allow_any_header()
                    .allow_any_method()
                    .allow_any_origin(),
            )
            .wrap(middleware::Logger::default())
            .app_data(Data::new(pool.clone()))
            .app_data(Data::new(site_core.clone()))
            .app_data(Data::new(client.clone()))
            .app_data(Data::new(titles_data.clone()))
            .app_data(Data::new(PayloadConfig::new(1024 * 1024 * 1024)))
            .service(titles)
            .configure(error::handlers::init)
            .configure(user::init)
            .configure(moderator::init)
            .configure(frontend::init)
            .configure(settings::init)
            .configure(admin::init)
            // TODO Make sure this is the correct way of handling vue and actix together. Also learn about packaging the website.
            .service(Files::new("/", std::env::var("SITE_DIR").unwrap()).show_files_listing())
    })
    .workers(4);

    // I am pretty sure this is correctly working
    // If I am correct this will only be available if the feature ssl is added
    #[cfg(feature = "ssl")]
    {
        if std::env::var("PRIVATE_KEY").is_ok() {
            use openssl::ssl::{SslAcceptor, SslFiletype, SslMethod};

            let mut builder = SslAcceptor::mozilla_intermediate(SslMethod::tls()).unwrap();
            builder
                .set_private_key_file(std::env::var("PRIVATE_KEY").unwrap(), SslFiletype::PEM)
                .unwrap();
            builder
                .set_certificate_chain_file(std::env::var("CERT_KEY").unwrap())
                .unwrap();
            return server
                .bind_openssl(std::env::var("ADDRESS").unwrap(), builder)?
                .run()
                .await;
        }
    }

    return server.bind(std::env::var("ADDRESS").unwrap())?.run().await;
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Moderator {
    pub user: User,
    pub avatar: String,
}

#[derive(Serialize, Deserialize)]
pub struct InstallRequest {
    pub username: String,
    pub password: String,
}

#[get("/titles")]
async fn titles(req: HttpRequest, title: TitleData) -> SiteResponse {
    APIResponse::respond_new(Some(title), &req)
}
