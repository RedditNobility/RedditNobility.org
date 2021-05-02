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
use crate::usererror::UserError;

#[get("/moderator/review/{user}")]
pub async fn review_users(pool: web::Data<DbPool>, mut rr: web::Data<Arc<Mutex<RedditRoyalty>>>, web::Path((user)): web::Path<( String)>, tera: web::Data<Tera>, req: HttpRequest) -> HttpResponse {
    let mut ctx = tera::Context::new();
    let conn = pool.get().expect("couldn't get db connection from pool");

    let option1 = req.cookie("auth_token");
    let result2 = utils::is_authorized(option1.unwrap().value().to_string(), Level::Moderator, &conn);
    if result2.is_err() {
        return result2.err().unwrap().site_error(tera);
    }
    if !result2.unwrap() {
        return UserError::NotAuthorized.site_error(tera);
    }

    ctx.insert("user", &user);

    let result = tera.get_ref().render("review-users.html", &ctx);
    return HttpResponse::Ok().content_type("text/html").body(&result.unwrap());
}