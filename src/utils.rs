use crate::action;
use diesel::MysqlConnection;
use crate::models::{User, Level};
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

///A standardized method for deciding rather a user is allowed to be where they are
pub fn is_authorized(api_token: String, level: Level, conn: &MysqlConnection) -> Result<bool, SiteError> {
    let result = action::get_user_from_auth_token(api_token, &conn);
    if result.is_err() {
        return Err(SiteError::DBError(result.err().unwrap()));
    }
    let user = result.unwrap();
    if user.is_none() {
        return Ok(false);
    }
    let user = user.unwrap();


    let user_level: Level = Level::from_str(user.level.as_str()).unwrap();
    if user_level.level() >= level.level() {
        Ok(true)
    }
    Ok(false)
}