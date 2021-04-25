use actix_session::{CookieSession, Session};
use actix_web::http::header::LOCATION;
use actix_web::http::StatusCode;
use actix_web::body::Body;
use actix_web::web::BytesMut;
use crate::{DbPool, main};
use actix_multipart_derive::MultipartForm;
use actix_web::{get, middleware, post, web, App, Error, HttpResponse, HttpServer, HttpRequest, error, Responder};
use crate::utils::quick_add;

#[derive(Debug, Clone, Default, MultipartForm)]
struct Form {
    #[multipart()]
    file: BytesMut,
}

#[post("/admin/file/upload")]
pub async fn file_upload(pool: web::Data<DbPool>, file: Form, session: Session, req: HttpRequest) -> HttpResponse {
    let conn = pool.get().expect("couldn't get db connection from pool");

    let result = String::from_utf8(file.file.to_vec()).unwrap();
    let split = result.split("\n");
    for x in split {
        quick_add(x.to_string(), "KingTux".to_string(),&conn);
    }
    HttpResponse::Found().header("Location", "/admin").finish()
}