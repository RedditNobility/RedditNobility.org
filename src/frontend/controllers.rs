use std::fs::read_to_string;
use std::path::Path;

use actix_web::{get, HttpRequest, HttpResponse};

use crate::api_response::SiteResponse;

#[get("/")]
pub async fn index(_r: HttpRequest) -> SiteResponse {
    get_file()
}

#[get("/user/{file:.*}")]
pub async fn user(_r: HttpRequest) -> SiteResponse {
    get_file()
}

#[get("/")]
pub async fn moderator(_r: HttpRequest) -> SiteResponse {
    get_file()
}

#[get("/review")]
pub async fn review(_r: HttpRequest) -> SiteResponse {
    get_file()
}
#[get("/about")]
pub async fn about(_r: HttpRequest) -> SiteResponse {
    get_file()
}

#[get("/review/{file:.*}")]
pub async fn review_with(_r: HttpRequest) -> SiteResponse {
    get_file()
}

#[get("/login")]
pub async fn login(_r: HttpRequest) -> SiteResponse {
    get_file()
}

#[get("/install")]
pub async fn install(_r: HttpRequest) -> SiteResponse {
    get_file()
}

fn get_file() -> SiteResponse {
    //TODO cache this value at runtime
    let content =
        read_to_string(Path::new(&std::env::var("SITE_DIR").unwrap()).join("index.html"))?;
    return Ok(HttpResponse::Ok().content_type("text/html").body(content));
}
