use crate::action;
use diesel::MysqlConnection;
use crate::models::User;
use std::time::{SystemTime, UNIX_EPOCH};
use crate::siteerror::SiteError;
use actix_session::Session;
use dotenv::Error;

pub fn quick_add(username: String, conn: &MysqlConnection) {
    let mut status = "Found";
    if username.contains("=T") {
        status = "Approved";
    } else if username.contains("=F") {
        status = "Denied";
    }
    let username = username.replace("=T", "").replace("=F", "").replace("\r", "");
    if action::get_fuser(username.clone(), &conn).unwrap().is_none() {
        let fuser = User {
            id: 0,
            username: username.clone(),
            moderator: "".to_string(),
            status: status.to_string(),
            created: SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_millis() as i64,
        };
        println!("Adding {}", &fuser);
        action::add_new_fuser(&fuser, &conn);
    }
}

pub fn is_authorized(session: Session, conn: &MysqlConnection, admin: bool) -> Result<bool, SiteError> {
    let result = action::get_moderators(&conn);
    if result.is_err() {
        return Err(SiteError::DBError(result.err().unwrap()));
    }
    if admin && result.unwrap().is_empty() {
        return Ok(true);
    }
    let result = session.get("moderator");
    if result.is_err() {}
    let option: Option<String> = result.unwrap();
    if option.is_none() {
        return Ok(false);
    }
    let result = action::get_moderator(option.unwrap(), conn);
    if result.is_err() {
        return Err(SiteError::DBError(result.err().unwrap()));
    }
    return Ok(result.unwrap().is_some());
}