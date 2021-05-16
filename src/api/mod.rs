use std::str::FromStr;

use actix_web::http::HeaderMap;

use actix_web::{get, web, HttpRequest, HttpResponse};

use diesel::MysqlConnection;

use new_rawr::traits::Content;

use crate::api::apiresponse::APIResponse;
use crate::models::{Level, User};

use crate::siteerror::SiteError;

use crate::websiteerror::WebsiteError;
use crate::{action, utils, DbPool};

pub mod admin;
pub mod apiresponse;
pub mod moderator;
pub mod user;

pub fn api_validate(
    header_map: &HeaderMap,
    level: Level,
    conn: &MysqlConnection,
) -> Result<bool, Box<dyn WebsiteError>> {
    let option = header_map.get("Authorization");
    if option.is_none() {
        return Ok(false);
    }
    let x = option.unwrap().to_str();
    if x.is_err() {}
    let header = x.unwrap().to_string();

    let split = header.split(" ").collect::<Vec<&str>>();
    let option = split.get(0);
    if option.is_none() {
        return Ok(false);
    }
    let value = split.get(1);
    if value.is_none() {
        return Ok(false);
    }
    let value = value.unwrap().to_string();
    let key = option.unwrap().to_string();
    if key.eq("Basic") {
        if level == Level::Client {
            let x1 = value.split(":").collect::<Vec<&str>>();
            let id = x1.get(0);
            if id.is_none() {
                return Ok(false);
            }
            let id = id.unwrap();
            let key = x1.get(1);
            if key.is_none() {
                return Ok(false);
            }
            let key = key.unwrap();
            let result = action::get_client_key_by_id(i64::from_str(id.clone()).unwrap(), conn);
            if result.is_err() {
                return Err(Box::new(SiteError::DBError(result.err().unwrap())));
            }
            let client = result.unwrap();
            if client.is_none() {
                return Ok(false);
            }
            return Ok(key.eq(&client.unwrap().api_key));
        } else {
            return Ok(false);
        }
    } else if key.eq("Bearer") {
        if level == Level::Client {
            return Ok(false);
        }
        let result1 = utils::is_authorized(value, level, conn);
        if result1.is_err() {
            return Err(result1.err().unwrap());
        }
        return Ok(result1.unwrap());
    }
    return Ok(false);
}

pub fn get_user_by_header(
    header_map: &HeaderMap,
    conn: &MysqlConnection,
) -> Result<Option<User>, Box<dyn WebsiteError>> {
    let option = header_map.get("Authorization");
    if option.is_none() {
        return Ok(None);
    }
    let x = option.unwrap().to_str();
    if x.is_err() {}
    let header = x.unwrap().to_string();

    let split = header.split(" ").collect::<Vec<&str>>();
    let option = split.get(0);
    if option.is_none() {
        return Ok(None);
    }
    let value = split.get(1);
    if value.is_none() {
        return Ok(None);
    }
    let value = value.unwrap().to_string();
    let key = option.unwrap().to_string();
    if key.eq("Bearer") {
        let result = action::get_user_from_auth_token(value, conn);
        if result.is_err() {
            return Err(Box::new(SiteError::DBError(result.err().unwrap())));
        }
        return Ok(result.unwrap());
    }
    Ok(None)
}

#[get("/api/moderators")]
pub async fn get_moderators(pool: web::Data<DbPool>, _r: HttpRequest) -> HttpResponse {
    let conn = pool.get().expect("couldn't get db connection from pool");
    let result = action::get_moderators(&conn);
    if result.is_err() {
        return SiteError::DBError(result.err().unwrap()).api_error();
    }
    let response = APIResponse::<Vec<User>> {
        success: true,
        data: Some(result.unwrap()),
    };
    HttpResponse::Ok()
        .content_type("application/json")
        .body(serde_json::to_string(&response).unwrap())
}
