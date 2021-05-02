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

pub trait WebsiteError: Error {
    fn status_code(&self) -> StatusCode;

    fn user_message(&self) -> &str;

    fn site_error(&self, tera: web::Data<Tera>) -> HttpResponse;
    fn api_error(&self) -> HttpResponse;
}

pub fn json_error_message(error: Box<&dyn WebsiteError>) -> Value {
    let mut values = HashMap::<String, Value>::new();
    values.insert("status".parse().unwrap(), Value::Number(error.status_code().as_u16().into()));
    values.insert("user_message".parse().unwrap(), Value::String(error.user_message().to_string()));
    serde_json::to_value(values).unwrap()
}
