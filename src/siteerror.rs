use std::error::Error;
use std::fmt::{Display, Formatter, Result as FmtResult};


use serde_json;
use actix_web::http::StatusCode;

/// Error type that occurs when an API request fails for some reason.
#[derive(Debug)]
pub enum SiteError {
    /// Occurs if JSON deserialization fails. This will always be a bug, so please report it
    /// if it does occur, but the error type is provided so you can fail gracefully.
    JSONError(serde_json::Error),
    DBError(diesel::result::Error),
    Other(String),
}

impl SiteError {}

impl Display for SiteError {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        match self {
            SiteError::JSONError(j) => {
                f.write_str("Unable to parse Json\n");
                f.write_str(j.to_string().as_str())
            }
            SiteError::Other(s) => {
                f.write_str(s.clone().as_str())
            }
            SiteError::DBError(error) => {
                f.write_str("Unable to execute query.\n");
                f.write_str(error.to_string().as_str())
            }

            _ => f.write_str("This error should not have occurred. Please file a bug"),
        }
    }
}

impl From<diesel::result::Error> for SiteError {
    fn from(err: diesel::result::Error) -> SiteError {
        SiteError::DBError(err)
    }
}


impl From<serde_json::Error> for SiteError {
    fn from(err: serde_json::Error) -> SiteError {
        SiteError::JSONError(err)
    }
} 