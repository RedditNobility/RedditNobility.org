use std::str::ParseBoolError;

use actix_web::HttpResponse;

use crate::RNCore;
use actix_web::http::StatusCode;
use base64::DecodeError;
use log::error;
use rraw::utils::error::APIError;
use std::error::Error;
use std::fmt::{Display, Formatter};
use std::string::FromUtf8Error;
use std::sync::PoisonError;

#[derive(Debug)]
pub enum InternalError {
    JSONError(serde_json::Error),
    IOError(std::io::Error),
    DBError(diesel::result::Error),
    ActixWebError(actix_web::Error),
    R2D2Error(r2d2::Error),
    BooleanParseError(ParseBoolError),
    DecodeError(DecodeError),
    ArgonError(argon2::Error),
    UTF8Error(FromUtf8Error),
    MissingArgument(String),
    Error(String),
}

impl InternalError {
    pub fn json_error(&self) -> HttpResponse {
        let result = HttpResponse::Ok()
            .status(StatusCode::INTERNAL_SERVER_ERROR)
            .content_type("application/json")
            .body("");
        result
    }
}

impl actix_web::error::ResponseError for InternalError {
    fn error_response(&self) -> HttpResponse {
        error!("Error Occurred {}", self);
        self.json_error()
    }
}

impl Display for InternalError {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        write!(f, "{}", self)
    }
}

impl Error for InternalError {}

//from<Error>
impl From<DecodeError> for InternalError {
    fn from(err: DecodeError) -> InternalError {
        InternalError::DecodeError(err)
    }
}
impl From<argon2::Error> for InternalError {
    fn from(err: argon2::Error) -> InternalError {
        InternalError::ArgonError(err)
    }
}
impl From<APIError> for InternalError {
    fn from(err: APIError) -> InternalError {
        InternalError::Error(err.to_string())
    }
}
impl From<PoisonError<std::sync::MutexGuard<'_, RNCore>>> for InternalError {
    fn from(_err: PoisonError<std::sync::MutexGuard<'_, RNCore>>) -> InternalError {
        InternalError::Error("Tux Broke Something really bad".to_string())
    }
}

impl From<hyper::Error> for InternalError {
    fn from(err: hyper::Error) -> InternalError {
        InternalError::Error(err.to_string())
    }
}

impl From<FromUtf8Error> for InternalError {
    fn from(err: FromUtf8Error) -> InternalError {
        InternalError::UTF8Error(err)
    }
}

impl From<diesel::result::Error> for InternalError {
    fn from(err: diesel::result::Error) -> InternalError {
        InternalError::DBError(err)
    }
}

impl From<r2d2::Error> for InternalError {
    fn from(err: r2d2::Error) -> InternalError {
        InternalError::R2D2Error(err)
    }
}

impl From<serde_json::Error> for InternalError {
    fn from(err: serde_json::Error) -> InternalError {
        InternalError::JSONError(err)
    }
}

impl From<actix_web::Error> for InternalError {
    fn from(err: actix_web::Error) -> InternalError {
        InternalError::ActixWebError(err)
    }
}

impl From<std::io::Error> for InternalError {
    fn from(err: std::io::Error) -> InternalError {
        InternalError::IOError(err)
    }
}

impl From<ParseBoolError> for InternalError {
    fn from(err: ParseBoolError) -> InternalError {
        InternalError::BooleanParseError(err)
    }
}
