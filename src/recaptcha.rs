use hyper::client::Client;

use hyper::http::request::Builder;

use crate::error::internal_error::InternalError;
use hyper::{Body, Method};
use hyper_tls::HttpsConnector;
use serde::{Deserialize, Deserializer};
use std::collections::HashSet;

pub async fn validate(
    secret: String,
    response: String,
    remote_address: String,
) -> Result<bool, InternalError> {
    let https = HttpsConnector::new();
    let client = Client::builder().build::<_, hyper::Body>(https);

    let builder1 = Builder::new()
        .method(Method::GET)
        .uri(format!(
            "https://www.google.com/recaptcha/api/siteverify?secret={}&response={}&remoteip={}",
            secret, response, remote_address
        ))
        .body(Body::empty())
        .unwrap();
    let response1 = client.request(builder1).await?;
    let bytes = hyper::body::to_bytes(response1.into_body()).await;
    let string = String::from_utf8(bytes.unwrap().to_vec()).unwrap();
    let result1: RecaptchaResponse = serde_json::from_str(string.as_str())?;
    return Ok(result1.success);
}

#[derive(Debug, Deserialize)]
pub struct RecaptchaResponse {
    pub success: bool,
    #[serde(rename = "error-codes")]
    pub error_codes: Option<HashSet<Code>>,
}

#[derive(PartialEq, Eq, Hash, Debug)]
pub enum Code {
    MissingSecret,
    InvalidSecret,
    MissingResponse,
    InvalidResponse,
    BadRequest,
    Unknown(String),
}

impl<'de> Deserialize<'de> for Code {
    fn deserialize<D>(de: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let code = String::deserialize(de)?;
        Ok(match &*code {
            "missing-input-secret" => Code::MissingSecret,
            "invalid-input-secret" => Code::InvalidSecret,
            "missing-input-response" => Code::MissingResponse,
            "invalid-input-response" => Code::InvalidResponse,
            "bad-request" => Code::BadRequest,
            _ => Code::Unknown(code),
        })
    }
}
