use serde::{Deserialize, Serialize, Deserializer};
use std::collections::HashSet;
use hyper_tls::HttpsConnector;
use hyper::{Body, Method, Request, StatusCode};
use hyper::client::{Client, HttpConnector};
use hyper::header::USER_AGENT;
use hyper::http::request::Builder;
use hyper::Uri;
use crate::websiteerror::WebsiteError;
use crate::siteerror::SiteError;

pub async fn validate(secret: String, response: String, remote_address: String) -> Result<bool, Box<dyn WebsiteError>> {
    let https = HttpsConnector::new();
    let client = Client::builder().build::<_, hyper::Body>(https);


    let builder1 = Builder::new().method(Method::GET).uri(format!("https://www.google.com/recaptcha/api/siteverify?secret={}&response={}&remoteip={}", secret, response, remote_address)).body(Body::empty()).unwrap();
    let result = client.request(builder1).await;
    if result.is_err() {
        return Err(Box::new(SiteError::Other("Unable to make request to google".to_string())));
    }
    let response1 = result.unwrap();
    let bytes = hyper::body::to_bytes(response1.into_body()).await;
    let string = String::from_utf8(bytes.unwrap().to_vec()).unwrap();
    let result1: Result<RecaptchaResponse, serde_json::Error> = serde_json::from_str(string.as_str());
    if result1.is_err() {
        return Err(Box::new(SiteError::JSONError(result1.err().unwrap())));
    }
    return Ok(result1.unwrap().success);
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
    fn deserialize<D>(de: D) -> Result<Self, D::Error> where
        D: Deserializer<'de>
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