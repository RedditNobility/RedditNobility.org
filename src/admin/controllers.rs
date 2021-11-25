use actix_web::{ web, HttpRequest};

use crate::api_response::{APIResponse, SiteResponse};
use crate::{Database};
use crate::error::response::{bad_request, not_found, unauthorized};
use crate::user::action::{get_user_by_id};
use crate::user::utils::get_user_by_header;
use actix_web::post;






#[post("/api/admin/user/{user}/permission/{key}/{value}")]
pub async fn update_permission(
    database: Database,
    r: HttpRequest,
    path: web::Path<(i64, String, bool)>,
) -> SiteResponse {
    let (user, key, value) = path.into_inner();
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
        "moderator" => {
            permissions.moderator = value;
        }
        "submit" => {
            permissions.submit = value;
        }
        "approve_user" => {
            permissions.review_user = value;
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
