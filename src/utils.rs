use chrono::{DateTime, Utc};
use diesel::MysqlConnection;

use new_rawr::auth::AnonymousAuthenticator;
use new_rawr::client::RedditClient;
use rand::distributions::Alphanumeric;
use rand::Rng;
use std::fs;
use std::fs::{read, File};
use std::io::{BufRead, BufReader};
use std::path::PathBuf;
use std::path::{Path as SysPath, Path};
use std::str::FromStr;

use crate::error::internal_error::InternalError;
use crate::settings::action::get_setting;
use crate::User;
use rust_embed::RustEmbed;
use std::time::{Duration, SystemTime, UNIX_EPOCH};

#[derive(RustEmbed)]
#[folder = "$CARGO_MANIFEST_DIR/resources"]
pub struct Resources;

impl Resources {
    pub fn file_get(file: &str) -> Vec<u8> {
        let buf = Path::new("resources").join(file);
        if buf.exists() {
            read(buf).unwrap()
        } else {
            Resources::get(file).unwrap().data.to_vec()
        }
    }
    pub fn file_get_string(file: &str) -> String {
        let vec = Resources::file_get(file);
        String::from_utf8(vec).unwrap()
    }
}

pub fn installed(conn: &MysqlConnection) -> Result<bool, InternalError> {
    let installed: bool = bool::from_str(std::env::var("INSTALLED").unwrap().as_str()).unwrap();
    if installed {
        return Ok(true);
    }
    let option = get_setting("INSTALLED", conn)?;
    if option.is_none() {
        return Ok(false);
    }
    std::env::set_var("INSTALLED", "true");
    Ok(true)
}

pub(crate) fn get_current_time() -> i64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_millis() as i64
}

pub fn send_login(user: &String, password: String, rr: &RedditClient) -> Result<(), InternalError> {
    let string = build_message(user, password)?;
    rr.messages()
        .compose(user.as_str(), "RedditNobility Login", string.as_str())?;
    return Ok(());
}

fn build_message(user: &String, password: String) -> Result<String, InternalError> {
    let string = fs::read_to_string(PathBuf::new().join("resources").join("login-message"))?;
    let string = string
        .replace("{{PASSWORD}}", &password)
        .replace("{{USERNAME}}", user);
    return Ok(string);
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



pub fn is_valid(username: &String) -> Option<String> {
    let string1 = Resources::file_get_string("names.txt");
    let split = string1.split(",");
    let vec :Vec<&str>= split.collect();
    let string = username.to_lowercase();
    for x in vec {
        if string.contains(&x.to_string()) {
            return Some(x.to_string());
        }
    }
    return None;
}
#[test]
fn valid_test(){
    let option = is_valid(&"KingTuxWH".to_string());
    assert_eq!(option.unwrap(), "king")
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
