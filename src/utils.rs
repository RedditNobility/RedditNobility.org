use crate::{action, RedditRoyalty};
use diesel::MysqlConnection;
use crate::models::{User, Level, ClientKey, AuthToken};
use std::time::{SystemTime, UNIX_EPOCH};
use crate::siteerror::SiteError;
use actix_session::Session;
use dotenv::Error;
use rand::Rng;
use rand::distributions::Alphanumeric;
use actix_web::web::{Data, Path};
use std::sync::{Arc, Mutex};
use std::fs;
use crate::websiteerror::WebsiteError;

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
pub fn is_authorized(api_token: String, level: Level, conn: &MysqlConnection) -> Result<bool, dyn WebsiteError> {
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

pub fn create_token(user: &User, connection: &MysqlConnection) -> Result<AuthToken, Error> {
    let s: String = rand::thread_rng().sample_iter(&Alphanumeric)
        .take(25)
        .map(char::from)
        .collect();
    let token = AuthToken {
        id: 0,
        user: user.id.clone(),
        token: s.clone(),
        created: get_current_time(),
    };
    let result = action::add_new_auth_token(&token, connection);

    return Ok(token);
}

fn get_current_time() -> i64 {
    SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_millis() as i64
}

pub fn send_login(user: &User, conn: MysqlConnection, rr: Data<Arc<Mutex<RedditRoyalty>>>) {
    let password: String = rand::thread_rng().sample_iter(&Alphanumeric).take(10).map(char::from).collect();
    let mut user = user.clone();
    user.set_password(password);
    let result = action::update_user(&user, &conn);
    let token = create_token(&user, &conn).unwrap();
    let result1 = rr.lock().unwrap().reddit.messages().compose(user.username.as_str(), "RedditNobility Login", build_message(&user, &password, &token));
}

fn build_message(user: &User, password: &String, token: &AuthToken) -> &str {
    let url = std::env::var("URL").unwrap();
    let string = fs::read_to_string(Path::new("resources").join("login-message"));
    if string.is_err() {
        todo!();
    }
    let string = string.unwrap().
        replace("{{URL}}", format!("{}/login/key?token={}", url, token.token.as_str()).as_str()).
        replace("{{PASSWORD}}", &password).
        replace("{{USERNAME}}", user.username.clone().as_str());
    return string.as_str();
}