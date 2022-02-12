use actix_web::{get, web};

use crate::api_response::{APIResponse, SiteResponse};

use crate::error::response::already_exists;

use crate::{utils, DbPool, TitleData};
use actix_web::{post, HttpRequest};

use bcrypt::{hash, DEFAULT_COST};
use serde::{Deserialize, Serialize};

use crate::settings::utils::quick_add;
use crate::user::action::add_new_user;
use crate::user::models::{Status, User, UserPermissions, UserProperties};

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
    titles: TitleData,
) -> SiteResponse {
    let conn = pool.get()?;
    let option = crate::settings::action::get_setting("installed", &conn)?;
    if option.is_some() {
        return already_exists();
    }
    let properties = UserProperties {
        avatar: None,
        description: Some("OG User".to_string()),
    };
    let user = User {
        id: 0,
        discord_id: 0,
        username: form.username.clone(),
        password: hash(&form.password.clone(), DEFAULT_COST).unwrap(),
        permissions: UserPermissions {
            admin: true,
            moderator: true,
            submit: true,
            review_user: true,
            login: true,
        },
        status: Status::Approved,
        status_changed: utils::get_current_time(),
        discoverer: "OG".to_string(),
        reviewer: "OG".to_string(),
        properties,
        title: utils::is_valid(&form.username, &titles)
            .unwrap_or_else(|| "No Title Identified".to_string()),
        created: utils::get_current_time(),
    };
    add_new_user(&user, &conn).unwrap();
    quick_add("installed", "true".to_string(), &conn)?;
    APIResponse::new(true, Some(true)).respond(&r)
}
