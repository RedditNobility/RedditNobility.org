use actix_web::{web, HttpRequest};

use crate::admin::action::{add_new_team_member, delete_team, delete_team_user};
use crate::api_response::{APIResponse, SiteResponse};
use crate::error::response::{bad_request, not_found, unauthorized};
use crate::user::action::{get_id_by_name, get_team_member, get_user_by_id};
use crate::user::models::{Level, TeamMember};
use crate::user::utils::get_user_by_header;
use crate::{get_current_time, Database};
use actix_web::{delete, post, put};
use serde::{Deserialize, Serialize};

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
        "review_user" => {
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
    APIResponse::respond_new(Some(true), &r)
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NewTeamMember {
    pub user: String,
    pub description: String,
    pub level: Level,
}

#[put("/api/admin/team/add")]
pub async fn add_team(
    database: Database,
    r: HttpRequest,
    data: web::Json<NewTeamMember>,
) -> SiteResponse {
    let connection = database.get()?;

    let admin = get_user_by_header(r.headers(), &connection)?;
    if admin.is_none() || !admin.unwrap().permissions.admin {
        return unauthorized();
    }
    let option = get_id_by_name(&data.user, &connection)?;
    if option.is_none() {
        return bad_request("Invalid Username");
    }
    let user = option.unwrap();
    delete_team_user(&user, &connection)?;
    let member = TeamMember {
        id: 0,
        user,
        description: data.description.clone(),
        level: data.level.clone(),
        created: get_current_time(),
    };
    add_new_team_member(&member, &connection)?;
    APIResponse::respond_new(get_team_member(&member.user, &connection)?, &r)
}

#[delete("/api/admin/team/{member}")]
pub async fn delete_team_member(
    database: Database,
    r: HttpRequest,
    path: web::Path<i64>,
) -> SiteResponse {
    let team = path.into_inner();
    let connection = database.get()?;

    let admin = get_user_by_header(r.headers(), &connection)?;
    if admin.is_none() || !admin.unwrap().permissions.admin {
        return unauthorized();
    }
    let option = get_user_by_id(&team, &connection)?;
    if option.is_none() {
        return not_found();
    }
    delete_team(&team, &connection)?;
    APIResponse::respond_new(Some(true), &r)
}
