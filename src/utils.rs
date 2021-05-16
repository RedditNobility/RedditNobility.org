use crate::action;
use crate::models::{AuthToken, Level, Status, User, UserProperties, SubmitUser};
use crate::siteerror::SiteError;
use crate::websiteerror::WebsiteError;

use bcrypt::{hash, DEFAULT_COST};
use chrono::{DateTime, Utc};
use diesel::MysqlConnection;
use dotenv::Error;
use new_rawr::auth::AnonymousAuthenticator;
use new_rawr::client::RedditClient;
use rand::distributions::Alphanumeric;
use rand::Rng;
use std::fs;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path as SysPath;
use std::path::PathBuf;
use log::{info, warn};

use std::time::{Duration, SystemTime, UNIX_EPOCH};

pub fn quick_add(username: String, discoverer: String, conn: &MysqlConnection) {
    info!("Adding user {}", &username);

    let mut status = Status::Found;
    if username.contains("=T") {
        status = Status::Approved;
    } else if username.contains("=F") {
        status = Status::Denied
    }
    let username = username
        .replace("=T", "")
        .replace("=F", "")
        .replace("\r", "");

    if action::get_user_by_name(username.clone(), &conn)
        .unwrap()
        .is_none()
    {
        let properties = UserProperties {
            avatar: None,
            description: None,
            title: is_valid(username.clone()),
        };
        let user = User {
            id: 0,
            username: username.clone(),
            password: "".to_string(),
            moderator: "".to_string(),
            status,
            status_changed: 0,
            created: SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_millis() as i64,
            level: Level::User,
            discoverer,
            properties,
        };
        action::add_new_user(&user, &conn);
    }
}

pub fn submit_add(sub: SubmitUser, discoverer: String, conn: &MysqlConnection) {
    info!("Adding user {}", &sub.username);
    if action::get_user_by_name(sub.username.clone(), &conn)
        .unwrap()
        .is_none()
    {
        let user = User::new(sub, discoverer);
        action::add_new_user(&user, &conn);
    }
}

///A standardized method for deciding rather a user is allowed to be where they are
pub fn is_authorized(
    api_token: String,
    target_level: Level,
    conn: &MysqlConnection,
) -> Result<bool, Box<dyn WebsiteError>> {
    let result = action::get_user_from_auth_token(api_token, &conn);
    if result.is_err() {
        return Err(Box::new(SiteError::DBError(result.err().unwrap())));
    }
    let user = result.unwrap();
    if user.is_none() {
        return Ok(false);
    }

    let user = user.unwrap();
    if user.status != Status::Approved {
        return Ok(false);
    }
    println!("HEY");
    if user.level.level() >= target_level.level() {
        return Ok(true);
    }
    return Ok(false);
}

pub fn create_token(user: &User, connection: &MysqlConnection) -> Result<AuthToken, Error> {
    let s: String = rand::thread_rng()
        .sample_iter(&Alphanumeric)
        .take(25)
        .map(char::from)
        .collect();
    let token = AuthToken {
        id: 0,
        user: user.id.clone(),
        token: s.clone(),
        created: get_current_time(),
    };
    let _result = action::add_new_auth_token(&token, connection);

    return Ok(token);
}

pub(crate) fn get_current_time() -> i64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_millis() as i64
}

pub fn send_login(user: &User, conn: &MysqlConnection, rr: &RedditClient) {
    let password: String = rand::thread_rng()
        .sample_iter(&Alphanumeric)
        .take(10)
        .map(char::from)
        .collect();
    let mut user = user.clone();
    user.set_password(hash(&password.clone(), DEFAULT_COST).unwrap());
    let _result = action::update_user(&user, &conn);
    let token = create_token(&user, &conn).unwrap();
    let string = build_message(&user, &password, &token);

    let x = user.username.as_str();
    let _result1 = rr
        .messages()
        .compose(x, "RedditNobility Login", string.as_str());
}

fn build_message(user: &User, password: &String, token: &AuthToken) -> String {
    let url = std::env::var("URL").unwrap();
    let string = fs::read_to_string(PathBuf::new().join("resources").join("login-message"));
    if string.is_err() {
        todo!();
    }
    let string = string
        .unwrap()
        .replace(
            "{{URL}}",
            &*format!("{}/login/key?token={}", url, token.token),
        )
        .replace("{{PASSWORD}}", &password)
        .replace("{{USERNAME}}", &*user.username.clone());
    return string;
}

pub fn approve_user(user: &User, client: &RedditClient) -> bool {
    let result1 = client
        .subreddit("RedditNobility")
        .invite_member(user.username.clone());
    if result1.is_err() {
        return false;
    }
    return result1.unwrap();
}

fn lines_from_file(filename: impl AsRef<SysPath>) -> Vec<String> {
    let file = File::open(filename).expect("no such file");
    let buf = BufReader::new(file);
    buf.lines()
        .map(|l| l.expect("Could not parse line"))
        .collect()
}

pub fn is_valid(username: String) -> Option<String> {
    let vec = lines_from_file(SysPath::new("resources").join("names.txt"));
    let string = username.to_lowercase();
    for x in vec {
        if string.contains(&x) {
            return Some(x);
        }
    }
    return None;
}

pub fn to_date(time: i64) -> String {
    let d = UNIX_EPOCH + Duration::from_millis(time as u64);
    let datetime = DateTime::<Utc>::from(d);
    return datetime.format("%m/%d/%Y").to_string();
}

pub fn get_avatar(user: &User) -> String {
    let option1 = user.properties.avatar.as_ref();
    if option1.is_some() {
        if !option1.unwrap().is_empty() {
            return option1.unwrap().clone();
        }
    }

    let client = RedditClient::new(
        "Robotic Monarch by u/KingTuxWH",
        AnonymousAuthenticator::new(),
    );
    let user1 = client.user(user.username.as_str());
    let result = user1.about();
    if result.is_err() {
        return "".to_string();
    }
    let about = result.unwrap();
    let option = about.data.snoovatar_img;
    if let Some(avatar) = option {
        if !avatar.is_empty() {
            return avatar.clone();
        }
    }
    let option = about.data.icon_img;
    if option.is_some() {
        return option.unwrap().clone();
    }
    return "".to_string();
}

pub fn gen_client_key() -> String {
    rand::thread_rng()
        .sample_iter(&Alphanumeric)
        .take(25)
        .map(char::from)
        .collect()
}
