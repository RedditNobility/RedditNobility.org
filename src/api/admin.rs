


use crate::api::api_validate;
use crate::api::apiresponse::{APIResponse};
use crate::api::get_user_by_header;
use crate::models::{ClientKey, Level, User};


use crate::siteerror::SiteError::DBError;
use crate::usererror::UserError;
use crate::websiteerror::WebsiteError;
use crate::{action, utils, DbPool};





use actix_web::{
      post, web,  HttpRequest, HttpResponse,
};


use log::{ info};




use new_rawr::traits::{Content};


use serde::{Deserialize};





use std::str::FromStr;



#[post("/api/admin/key/add")]
pub async fn new_key(pool: web::Data<DbPool>, r: HttpRequest) -> HttpResponse {
    let conn = pool.get().expect("couldn't get db connection from pool");
    let result = api_validate(r.headers(), Level::Admin, &conn);
    if result.is_err() {
        return result.err().unwrap().api_error();
    }
    if !result.unwrap() {
        return UserError::NotAuthorized.api_error();
    }
    let client_key = ClientKey {
        id: 0,
        api_key: utils::gen_client_key(),
        created: utils::get_current_time(),
    };
    let result2 = action::add_client_key(&client_key, &conn);
    if result2.is_err() {
        return DBError(result2.err().unwrap()).api_error();
    }

    let result1 = action::get_client_key_by_key(client_key.api_key.clone(), &conn);
    if result1.is_err() {
        return DBError(result1.err().unwrap()).api_error();
    }
    let result1 = result1.unwrap();

    let response = APIResponse::<ClientKey> {
        success: true,
        data: Some(result1.unwrap().clone()),
    };
    return HttpResponse::Ok()
        .content_type("application/json")
        .body(serde_json::to_string(&response).unwrap());
}

#[derive(Deserialize)]
pub struct ChangeLevel {
    pub username: String,
    pub level: String,
}

#[post("/api/admin/change/level")]
pub async fn change_level(
    pool: web::Data<DbPool>,
    suggest: web::Form<ChangeLevel>,
    r: HttpRequest,
) -> HttpResponse {
    println!("Test1");
    let conn = pool.get().expect("couldn't get db connection from pool");
    let result = api_validate(r.headers(), Level::Admin, &conn);
    if result.is_err() {
        return result.err().unwrap().api_error();
    }
    if !result.unwrap() {
        return UserError::NotFound.api_error();
    }
    let moderator = get_user_by_header(&r.headers(), &conn);
    if moderator.is_err() {
        return moderator.err().unwrap().api_error();
    }
    let moderator = moderator.unwrap().unwrap();
    let result1 = action::get_user_by_name(suggest.username.clone(), &conn);
    if result1.is_err() {
        return DBError(result1.err().unwrap()).api_error();
    }
    let result1 = result1.unwrap();
    if result1.is_none() {
        return UserError::NotFound.api_error();
    }
    let level: Result<Level, strum::ParseError> = Level::from_str(suggest.level.as_str());
    if level.is_err() {
        println!("{}", suggest.level.as_str());
        return UserError::InvalidRequest.api_error();
    }
    let mut user = result1.unwrap();
    let level1 = level.unwrap();
    user.set_level(level1.clone());
    let result = action::update_user(&user, &conn);
    if result.is_err() {
        return DBError(result.err().unwrap()).api_error();
    }
    let response = APIResponse::<User> {
        success: true,
        data: None,
    };
    info!(
        "{}",
        format!(
            "{} has changed the level of {} to {}",
            moderator.username.clone(),
            user.username.clone(),
            level1.name()
        )
    );
    return HttpResponse::Ok()
        .content_type("application/json")
        .body(serde_json::to_string(&response).unwrap());
}
