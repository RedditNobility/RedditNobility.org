use std::path::Path;
use actix_web::{get, web, HttpRequest};

use crate::api_response::{APIResponse, SiteResponse};
use crate::{Database, User, RN, utils};
use new_rawr::structures::submission::Submission;
use new_rawr::traits::{Content, Votable};

use crate::error::response::{bad_request, error, not_found, unauthorized};
use crate::user::action::{get_found_users, get_user_by_name, update_properties};
use crate::user::utils::get_user_by_header;
use serde::{Deserialize, Serialize};
use std::str::FromStr;
use actix_web::post;
use actix_web::http::StatusCode;
use actix_web::web::Json;
use strum::ParseError;
use crate::admin::action::set_level;
use crate::user::models::{Level, Status};
use crate::utils::get_current_time;

#[post("/api/admin/change/{user}/{level}")]
pub async fn change_level(
    database: Database,
    value: web::Path<(String, String)>,
    r: HttpRequest,
) -> SiteResponse {
    let conn = database.get()?;
    let user = get_user_by_header(r.headers(), &conn)?;
    if user.is_none() {
        return unauthorized();
    }
    let user = user.unwrap();
    if user.level != Level::Admin {
        return unauthorized();
    }
    let result1 = get_user_by_name(&value.0.0, &conn)?;
    if result1.is_none(){
        return not_found();
    }
    let user = result1.unwrap();
    let level: Result<Level, strum::ParseError> = Level::from_str(&value.0.1);
    if level.is_err() {
        return bad_request("Approved or Denied".to_string());
    }
    let level1 = level.unwrap();

    set_level(&user.id, level1, &conn)?;
    return APIResponse::new(true, Some(true)).respond(&r);

}
