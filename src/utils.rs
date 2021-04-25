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
use std::path::PathBuf;
use std::str::FromStr;
use bcrypt::{hash, DEFAULT_COST};

pub fn quick_add(username: String, discoverer: String, conn: &MysqlConnection) {
    let mut status = "Found";
    if username.contains("=T") {
        status = "Approved";
    } else if username.contains("=F") {
        status = "Denied";
    }
    let username = username.replace("=T", "").replace("=F", "").replace("\r", "");
    if action::get_user_by_name(username.clone(), &conn).unwrap().is_none() {
        let user = User {
            id: 0,
            username: username.clone(),
            password: "".to_string(),
            moderator: "".to_string(),
            status: status.to_string(),
            status_changed: 0,
            created: SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_millis() as i64,
            level: Level::User.name().to_string(),
            discoverer,
        };
        action::add_new_user(&user, &conn);
    }
}

///A standardized method for deciding rather a user is allowed to be where they are
pub fn is_authorized(api_token: String, target_level: Level, conn: &MysqlConnection) -> Result<bool, Box<dyn WebsiteError>> {
    let result = action::get_user_from_auth_token(api_token, &conn);
    if result.is_err() {
        return Err(Box::new(SiteError::DBError(result.err().unwrap())));
    }
    let user = result.unwrap();
    if user.is_none() {
        return Ok(false);
    }
    let user = user.unwrap();


    let level: Result<Level, strum::ParseError> = Level::from_str(user.level.as_str());
    if level.unwrap().level() >= target_level.level() {
        return Ok(true);
    }
    return Ok(false);
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

pub fn send_login(user: &User, conn: &MysqlConnection, rr: Data<Arc<Mutex<RedditRoyalty>>>) {
    let password: String = rand::thread_rng().sample_iter(&Alphanumeric).take(10).map(char::from).collect();
    let mut user = user.clone();
    user.set_password(hash(&password.clone(), DEFAULT_COST).unwrap(),);
    let result = action::update_user(&user, &conn);
    let token = create_token(&user, &conn).unwrap();
    let result1 = rr.lock().unwrap().reddit.messages().compose(user.username.as_str(), "RedditNobility Login", build_message(&user, &password, &token).as_str());
}

fn build_message(user: &User, password: &String, token: &AuthToken) -> String {
    let url = std::env::var("URL").unwrap();
    let string = fs::read_to_string(PathBuf::new().join("resources").join("login-message"));
    if string.is_err() {
        todo!();
    }
    let string = string.unwrap().
        replace("{{URL}}", format!("{}/login/key?token={}", url, token.token.as_str()).as_str()).
        replace("{{PASSWORD}}", &password).
        replace("{{USERNAME}}", user.username.clone().as_str());
    return string;
}