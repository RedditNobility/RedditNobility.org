use chrono::{DateTime, Utc};
use diesel::MysqlConnection;

use rand::distributions::Alphanumeric;
use rand::Rng;

use std::fs::{read};
use std::io::{BufRead};

use std::path::{Path};
use std::str::FromStr;

use crate::error::internal_error::InternalError;
use crate::settings::action::get_setting;
use crate::User;
use rust_embed::RustEmbed;
use std::time::{Duration, SystemTime, UNIX_EPOCH};
use log::error;
use rraw::auth::AnonymousAuthenticator;
use rraw::me::Me;
use rraw::utils::options::FriendType;

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

pub async fn send_login(user: &String, password: String, rr: &Me) -> Result<(), InternalError> {
    let string = build_message(user, password)?;
    rr.inbox()
        .compose(user.clone(), "RedditNobility Login".to_string(), string, None).await?;
    return Ok(());
}

fn build_message(user: &String, password: String) -> Result<String, InternalError> {
    let string = Resources::file_get_string("login-message");
    let string = string
        .replace("{{PASSWORD}}", &password)
        .replace("{{USERNAME}}", user);
    return Ok(string);
}

pub async fn approve_user(user: &User, client: &Me) -> bool {
    let result1 =  client
        .subreddit("RedditNobility".to_string()).add_friend(user.username.clone(), FriendType::Contributor).await;
    if result1.is_err() {
        error!("Unable to approve User {}", result1.err().unwrap());
        return false;
    }
    return result1.unwrap().success;
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

pub async fn get_avatar(user: &User) -> Result<String, InternalError> {
    let option1 = user.properties.avatar.as_ref();
    if option1.is_some() {
        if !option1.unwrap().is_empty() {
            return Ok(option1.unwrap().clone());
        }
    }

    let client = Me::login(
        AnonymousAuthenticator::new(),
        "Robotic Monarch by u/KingTuxWH".to_string()
    ).await?;
    let user1 = client.user(user.username.clone());
    let about = user1.about().await?;

    let option = about.data.snoovatar_img;
    if let Some(avatar) = option {
        if !avatar.is_empty() {
            return Ok(avatar.clone());
        }
    }
    let option = about.data.icon_img;
    if option.is_some() {
        return Ok(option.unwrap().clone());
    }
    return Ok("".to_string());
}

