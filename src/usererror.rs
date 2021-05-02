use std::error::Error;
use std::fmt::{Display, Formatter, Result as FmtResult};

use actix_web::{ error, get, http::header, http::StatusCode, App, HttpResponse, HttpServer, web};
use derive_more::{Display, Error};
use serde_json;
use error::ResponseError;
use log::{error, info, warn};
use serde_json::Value;
use std::collections::HashMap;
use tera::Tera;
use crate::websiteerror::{WebsiteError, json_error_message};
use crate::apiresponse::{APIError, APIResponse};
use actix::fut::err;

/// Error type that occurs when an API request fails for some reason.
#[derive(Debug, Display)]
pub enum UserError {
    InvalidRequest,
    NotAuthorized,
    NotFound,

}
impl Error for UserError{

}
impl WebsiteError for UserError {
    fn status_code(&self) -> StatusCode {
        match *self {
            UserError::InvalidRequest => StatusCode::NOT_ACCEPTABLE,
            UserError::NotAuthorized => StatusCode::UNAUTHORIZED,
            UserError::NotFound => StatusCode::NOT_FOUND,
        }
    }
    fn user_message(&self) -> &str {
        match *self {
            UserError::InvalidRequest => "",
            UserError::NotAuthorized => "",
            UserError::NotFound => ""
        }
    }

    fn site_error(&self, tera: web::Data<Tera>) -> HttpResponse {
        let mut ctx = tera::Context::new();
        let x = json_error_message(Box::new(self));
        ctx.insert("error", x["user_message"].as_str().unwrap());
        let result = tera.get_ref().render("error.html", &ctx);
        if result.is_err() {
            let error = result.err().unwrap();
            error!("{}", error);
            return HttpResponse::InternalServerError().finish();
        }
        HttpResponse::Ok().status(self.status_code()).content_type("text/html").body(&result.unwrap())
    }
    fn api_error(&self) -> HttpResponse {
        let error = APIError {
            status_code: Some(self.status_code().as_u16()),
            user_friendly_message: Some(self.user_message().to_string()),
            error_code: None,
        };
        let response = APIResponse::<APIError> {
            success: false,
            data: Some(error),
        };
        HttpResponse::Ok().status(self.status_code()).content_type("application/json").body(serde_json::to_string(&response).unwrap())
    }
}

