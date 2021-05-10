use crate::models::Level;

use crate::siteerror::SiteError;
use crate::usererror::UserError;
use crate::websiteerror::WebsiteError;
use crate::{action, utils, DbPool, RedditRoyalty};

use actix_web::{
    get, http, middleware, post, web, App, Error, HttpMessage, HttpRequest, HttpResponse,
    HttpServer,
};

use new_rawr::traits::Content;

use std::sync::{Arc, Mutex};
use tera::Tera;

#[get("/moderator")]
pub async fn mod_index(
    pool: web::Data<DbPool>,
    _rr: web::Data<Arc<Mutex<RedditRoyalty>>>,
    tera: web::Data<Tera>,
    req: HttpRequest,
) -> HttpResponse {
    let ctx = tera::Context::new();
    let conn = pool.get().expect("couldn't get db connection from pool");

    let option1 = req.cookie("auth_token");
    let result2 = utils::is_authorized(
        option1.unwrap().value().to_string(),
        Level::Moderator,
        &conn,
    );
    if result2.is_err() {
        return result2.err().unwrap().site_error(tera);
    }
    if !result2.unwrap() {
        return UserError::NotAuthorized.site_error(tera);
    }

    let result = tera.get_ref().render("moderator.html", &ctx);
    return HttpResponse::Ok()
        .content_type("text/html")
        .body(&result.unwrap());
}

#[get("/moderator/review/{user}")]
pub async fn review_users(
    pool: web::Data<DbPool>,
    _rr: web::Data<Arc<Mutex<RedditRoyalty>>>,
    web::Path(user): web::Path<String>,
    tera: web::Data<Tera>,
    req: HttpRequest,
) -> HttpResponse {
    let mut ctx = tera::Context::new();
    let conn = pool.get().expect("couldn't get db connection from pool");

    let option1 = req.cookie("auth_token");
    let result2 = utils::is_authorized(
        option1.unwrap().value().to_string(),
        Level::Moderator,
        &conn,
    );
    if result2.is_err() {
        return result2.err().unwrap().site_error(tera);
    }
    if !result2.unwrap() {
        return UserError::NotAuthorized.site_error(tera);
    }

    ctx.insert("user", &user);

    let result = tera.get_ref().render("review-users.html", &ctx);
    return HttpResponse::Ok()
        .content_type("text/html")
        .body(&result.unwrap());
}

#[get("/moderator/user/{user}")]
pub async fn user_page(
    pool: web::Data<DbPool>,
    _rr: web::Data<Arc<Mutex<RedditRoyalty>>>,
    web::Path(username): web::Path<String>,
    tera: web::Data<Tera>,
    req: HttpRequest,
) -> HttpResponse {
    let mut ctx = tera::Context::new();
    let conn = pool.get().expect("couldn't get db connection from pool");

    let option1 = req.cookie("auth_token");
    if option1.is_none() {
        return UserError::NotAuthorized.site_error(tera);
    }
    let string = option1.unwrap().value().to_string();
    let result2 = utils::is_authorized(string.clone(), Level::Moderator, &conn);
    if result2.is_err() {
        return result2.err().unwrap().site_error(tera);
    }
    if !result2.unwrap() {
        return UserError::NotAuthorized.site_error(tera);
    }
    let user = action::get_user_from_auth_token(string, &conn);
    if user.is_err() {
        return SiteError::DBError(user.err().unwrap()).site_error(tera);
    }
    let user = user.unwrap();
    if user.is_none() {
        return UserError::NotAuthorized.site_error(tera);
    }
    let user = user.unwrap();
    let lookup = action::get_user_by_name(username.clone(), &conn);
    if lookup.is_err() {
        return SiteError::DBError(lookup.err().unwrap()).site_error(tera);
    }
    let lookup = lookup.unwrap();
    if lookup.is_none() {
        return UserError::NotFound.site_error(tera);
    }
    let lookup = lookup.unwrap();
    ctx.insert("user", &lookup);
    ctx.insert("level", &user.level.to_string());
    ctx.insert("avatar", &utils::get_avatar(&lookup));
    ctx.insert("created", &utils::to_date(lookup.created));
    ctx.insert("status_changed", &utils::to_date(lookup.status_changed));
    let result = tera.get_ref().render("user.html", &ctx);
    if result.is_err() {
        return SiteError::TeraError(result.err().unwrap()).site_error(tera);
    }
    return HttpResponse::Ok()
        .content_type("text/html")
        .body(&result.unwrap());
}
