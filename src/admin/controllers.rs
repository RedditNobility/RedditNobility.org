use std::path::Path;
use actix_web::{get, web, HttpRequest};

use crate::api_response::{APIResponse, SiteResponse};
use crate::{Database, User, RN, utils};
use new_rawr::structures::submission::Submission;
use new_rawr::traits::{Content, Votable};

use crate::error::response::{bad_request, error, not_found, unauthorized};
use crate::user::action::{get_found_users, get_user_by_id, get_user_by_name, update_properties};
use crate::user::utils::get_user_by_header;
use serde::{Deserialize, Serialize};
use std::str::FromStr;
use actix_web::post;
use actix_web::http::StatusCode;
use actix_web::web::Json;
use strum::ParseError;
use crate::user::models::{Status};
use crate::utils::get_current_time;


#[post("/api/admin/user/{user}/permission/{key}/{value}")]
pub async fn update_permission(
    database: Database,
    r: HttpRequest,
    web::Path((user, key, value)): web::Path<(i64, String, bool)>,
) -> SiteResponse {
    let connection = database.get()?;

    let admin = get_user_by_header(r.headers(), &connection)?;
    if admin.is_none() || !admin.unwrap().permissions.admin {
        return unauthorized();
    }
    let option = get_user_by_id(&user, &connection)?;
    if option.is_none() {
        return not_found();
    }

    let mut permissions = option.unwrap().permissions;
    match key.as_str() {
        "admin" => {
            permissions.admin = value;
        }
        "modify_user" => {
            permissions.modify_user = value;
        }
        "submit" => {
            permissions.submit = value;
        }
        "approve_user" => {
            permissions.approve_user = value;
        }
        "login" => {
            permissions.login = value;
        }
        _ => {
            return bad_request("Invalid Permission");
        }
    };
    crate::admin::action::set_permissions(&user, permissions, &connection)?;
    return APIResponse::respond_new(Some(true), &r);
}
