pub mod install;

use actix_web::{get, web};

use crate::api_response::{APIResponse, SiteResponse};

use crate::error::response::already_exists;

use crate::{utils, DbPool};
use actix_web::{post, HttpRequest};
use bcrypt::{hash, DEFAULT_COST};
use serde::{Deserialize, Serialize};

use crate::settings::utils::quick_add;
use crate::user::action::add_new_user;
use crate::user::models::{Level, Status, User, UserProperties};

pub fn init(cfg: &mut web::ServiceConfig) {
    cfg.service(install_post).service(installed);
}

#[get("/api/installed")]
pub async fn installed(pool: web::Data<DbPool>, r: HttpRequest) -> SiteResponse {
    let connection = pool.get()?;
    let result = utils::installed(&connection)?;
    APIResponse::new(true, Some(result)).respond(&r)
}

#[derive(Serialize, Deserialize)]
pub struct InstallRequest {
    pub username: String,
    pub password: String,
}

#[post("/install")]
pub async fn install_post(
    pool: web::Data<DbPool>,
    r: HttpRequest,
    form: web::Json<InstallRequest>,
) -> SiteResponse {
    let conn = pool.get()?;
    let option = crate::settings::action::get_setting("installed", &conn)?;
    if option.is_some() {
        return already_exists();
    }
    let properties = UserProperties {
        avatar: None,
        description: Some("OG User".to_string()),
        title: utils::is_valid(&form.username),
    };
    let user = User {
        id: 0,
        username: form.username.clone(),
        password: hash(&form.password.clone(), DEFAULT_COST).unwrap(),
        level: Level::Admin,
        status: Status::Approved,
        status_changed: utils::get_current_time(),
        discoverer: "OG".to_string(),
        moderator: "OG".to_string(),
        properties,
        created: utils::get_current_time(),
    };
    add_new_user(&user, &conn).unwrap();
    quick_add("installed", "true".to_string(), &conn)?;
    APIResponse::new(true, Some(true)).respond(&r)
}
