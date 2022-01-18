use actix_web::web::{Json, Path};
use actix_web::{post, HttpRequest};
use bcrypt::{hash, DEFAULT_COST};
use rraw::utils::error::APIError;

use crate::api_response::{APIResponse, SiteResponse};
use crate::error::internal_error::InternalError::Error;
use crate::error::response::{already_exists, bad_request, not_found, unauthorized};
use crate::user::action::{get_user_by_name, update_properties};
use crate::user::utils::{get_user_by_header, quick_add};
use crate::{Database, RedditClient, TitleData};

#[post("/api/submit/{username}")]
pub async fn submit_user(
    pool: Database,
    suggest: Path<String>,
    r: HttpRequest,
    redditClient: RedditClient,
    titles: TitleData,
) -> SiteResponse {
    let conn = pool.get()?;
    let option = get_user_by_header(r.headers(), &conn)?;
    if option.is_none() {
        return unauthorized();
    }

    let discoverer = option.unwrap();
    let result1 = get_user_by_name(&suggest, &conn)?;
    if result1.is_some() {
        return already_exists();
    }
    let user_reddit = redditClient.user(suggest.to_string()).about().await;
    if let Err(error) = user_reddit {
        return match error {
            APIError::HTTPError(_) => not_found(),
            _ => Err(error.into()),
        };
    }
    quick_add(&suggest, &discoverer.username, &conn, &titles)?;
    let result1 = get_user_by_name(&suggest, &conn)?;
    if result1.is_none() {
        return Err(Error("Bad Creation?".to_string()));
    }
    if discoverer.permissions.submit {
        return APIResponse::respond_new(result1, &r);
    }
    APIResponse {
        success: true,
        data: Some(true),
        status_code: Some(201),
    }
    .respond(&r)
}

#[derive(serde::Deserialize)]
pub struct ChangeRequest {
    pub value: String,
}

#[post("/api/me/update/{key}")]
pub async fn change_property(
    database: Database,
    request: Json<ChangeRequest>,
    key: Path<String>,
    r: HttpRequest,
) -> SiteResponse {
    let conn = database.get()?;
    let option = get_user_by_header(r.headers(), &conn)?;
    if option.is_none() {
        return unauthorized();
    }
    let mut user = option.unwrap();
    let value = request.0.value;
    match key.as_str() {
        "avatar" => {
            user.properties.set_avatar(value);
        }
        "description" => {
            user.properties.set_description(value);
        }
        _ => {
            return bad_request("You can only change your Avatar or Description");
        }
    }
    update_properties(&user.id, user.properties, &conn)?;
    APIResponse::new(true, Some(true)).respond(&r)
}

#[post("/api/me/password/change")]
pub async fn update_password(
    database: Database,
    request: Json<ChangeRequest>,
    r: HttpRequest,
) -> SiteResponse {
    let conn = database.get()?;
    let option = get_user_by_header(r.headers(), &conn)?;
    if option.is_none() {
        return unauthorized();
    }
    let user = option.unwrap();
    let result = hash(&request.0.value, DEFAULT_COST)?;
    crate::user::action::update_password(&user.id, result, &conn)?;
    APIResponse::new(true, Some(true)).respond(&r)
}
