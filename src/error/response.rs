use crate::api_response::{APIResponse, RequestErrorResponse, SiteResponse};
use actix_web::http::StatusCode;

pub fn not_found() -> SiteResponse {
    APIResponse::<bool>::new(false, None).error(StatusCode::NOT_FOUND)
}

pub fn unauthorized() -> SiteResponse {
    APIResponse::<bool>::new(false, None).error(StatusCode::UNAUTHORIZED)
}

pub fn already_exists() -> SiteResponse {
    APIResponse::new(
        false,
        Some(RequestErrorResponse {
            user_friendly_message: None,
            error_code: Some("ALREADY_EXISTS".to_string()),
        }),
    )
    .error(StatusCode::CONFLICT)
}

pub fn bad_request<S: Into<String>>(value: S) -> SiteResponse {
    APIResponse::new(
        false,
        Some(RequestErrorResponse {
            user_friendly_message: Some(value.into()),
            error_code: None,
        }),
    )
    .error(StatusCode::BAD_REQUEST)
}

pub fn error<S: Into<String>>(value: S, status: Option<StatusCode>) -> SiteResponse {
    APIResponse::new(
        false,
        Some(RequestErrorResponse {
            user_friendly_message: Some(value.into()),
            error_code: None,
        }),
    )
    .error(status.unwrap_or(StatusCode::BAD_REQUEST))
}
