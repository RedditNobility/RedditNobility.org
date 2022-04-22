use diesel::MysqlConnection;

use std::fs::read;

use std::path::Path;
use std::str::FromStr;

use crate::error::internal_error::InternalError;
use crate::settings::action::get_setting;
use crate::{Titles, User};
use rust_embed::RustEmbed;
use std::time::{SystemTime, UNIX_EPOCH};

use crate::user::models::UserProperties;
use log::error;
use rraw::auth::{AnonymousAuthenticator, PasswordAuthenticator};
use rraw::Client;
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

pub async fn send_login(user: &str, password: String, rr: &Client<PasswordAuthenticator>) -> Result<(), InternalError> {
    let string = build_message(user, password)?;
    rr.subreddit("RedditNobility").await?
        .compose(
            user.to_string(),
            "RedditNobility Login".to_string(),
            string,
        )
        .await?;
    Ok(())
}

fn build_message(user: &str, password: String) -> Result<String, InternalError> {
    let string = Resources::file_get_string("login-message");
    let string = string
        .replace("{{PASSWORD}}", &password)
        .replace("{{USERNAME}}", user);
    Ok(string)
}

pub async fn approve_user(user: &User, client: &Client<PasswordAuthenticator>) -> bool {
    let subreddit = client
        .subreddit("RedditNobility").await.unwrap();
    let result = subreddit.add_friend(user.username.clone(), FriendType::Contributor)
        .await;
    if let Err(error) = result {
        error!("Unable to approve User {}", error);
        false
    } else if let Ok(friend) = result {
        friend.success
    }else{
        false
    }
}

pub fn yeet<T>(_drop: T) {}

pub fn is_valid(username: &str, titles: &Titles) -> Option<String> {
    let username = username.to_lowercase();
    for title in &titles.titles {
        if username.contains(&title.value) {
            if let Some(possibles) = &title.possible_titles {
                for possible in possibles {
                    if username.contains(possible) {
                        return Some(possible.clone());
                    }
                }
            }
            return Some(title.value.clone());
        }
    }
    None
}

#[tokio::test]
async fn valid_test() {
    use hyper::{Body, Client, Method, Request};

    let https = hyper_tls::HttpsConnector::new();
    let client = Client::builder().build::<_, hyper::Body>(https);

    let request = Request::builder()
        .method(Method::GET)
        .uri("https://raw.githubusercontent.com/RedditNobility/Titles/master/titles.json")
        .body(Body::empty())
        .unwrap();
    let response = client.request(request).await.unwrap();
    let bytes = hyper::body::to_bytes(response.into_body()).await.unwrap();
    let string = String::from_utf8(bytes.to_vec()).unwrap();
    let titles: Titles = serde_json::from_str(string.as_str()).unwrap();
    let _option = is_valid(&"KingTuxWH".to_string(), &titles);
    assert_eq!(is_valid(&"KingTuxWH".to_string(), &titles).unwrap(), "king");
    assert_eq!(is_valid(&"QueenTux".to_string(), &titles).unwrap(), "queen");
    assert_eq!(
        is_valid(&"VikingTux".to_string(), &titles).unwrap(),
        "viking"
    );
    assert_eq!(is_valid(&"LordTux".to_string(), &titles).unwrap(), "lord");
    assert_eq!(is_valid(&"CzArTux".to_string(), &titles).unwrap(), "czar");
}

pub async fn get_avatar(username: &str, user: &UserProperties) -> Result<String, InternalError> {
    let option1 = user.avatar.as_ref();
    if option1.is_some() && !option1.unwrap().is_empty() {
        return Ok(option1.unwrap().clone());
    }

    let client = rraw::Client::login(
        AnonymousAuthenticator::new(),
        "Robotic Monarch by u/KingTuxWH".to_string(),
    )
        .await?;
    let reddit_user = client.user(username).await?;

    let avatar = reddit_user.user.snoovatar_img;
    if !avatar.is_empty() {
        return Ok(avatar);
    }

    Ok(reddit_user.user.icon_img)
}
