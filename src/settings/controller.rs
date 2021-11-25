use actix_web::{get, post, web, HttpRequest};
use serde::{Deserialize, Serialize};

use crate::api_response::{APIResponse, SiteResponse};


use crate::error::response::unauthorized;
use crate::settings::action::get_setting;
use crate::settings::utils::{get_setting_or_empty, get_setting_report};
use crate::{settings, DbPool};
use crate::user::utils::get_user_by_header;


#[get("/api/setting/{setting}")]
pub async fn about_setting(
    pool: web::Data<DbPool>,
    r: HttpRequest,
    setting: web::Path<String>,
) -> SiteResponse {
    let connection = pool.get()?;

    let option = get_setting_or_empty(setting.as_str(), &connection)?;
    if !option.setting.public.unwrap_or(false) {
        //TODO check if admin
        return unauthorized();
    }
    APIResponse::from(Some(option)).respond(&r)
}

#[get("/api/settings/report")]
pub async fn setting_report(pool: web::Data<DbPool>, r: HttpRequest) -> SiteResponse {
    let connection = pool.get()?;

    let user = get_user_by_header(r.headers(), &connection)?;
    if user.is_none() || !user.unwrap().permissions.admin {
        return unauthorized();
    }
    let report = get_setting_report(&connection)?;
    APIResponse::from(Some(report)).respond(&r)
}

#[derive(Serialize, Deserialize, Debug)]
pub struct UpdateSettingRequest {
    pub value: String,
}

#[post("/api/admin/setting/{setting}/update")]
pub async fn update_setting(
    pool: web::Data<DbPool>,
    r: HttpRequest,
    request: web::Json<UpdateSettingRequest>,
    setting: web::Path<String>,
) -> SiteResponse {
    let connection = pool.get()?;

    let user = get_user_by_header(r.headers(), &connection)?;
    if user.is_none() || !user.unwrap().permissions.admin {
        return unauthorized();
    }
    let mut option = get_setting_or_empty(setting.as_str(), &connection)?;
    option.set_value(request.value.clone());
    settings::action::update_setting(&option, &connection)?;
    let option = get_setting(setting.as_str(), &connection)?;
    APIResponse::respond_new(option, &r)
}
