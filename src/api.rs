use std::time::{Duration, Instant};
use diesel::prelude::*;

use diesel::MysqlConnection;

use actix::prelude::*;
use log::{error, info, warn};
use actix_files as fs;
use actix_web::{middleware, get, post, web, App, Error, HttpRequest, HttpResponse, HttpServer, http};
use actix_web_actors::ws;
use crate::{DbPool, RedditRoyalty, action, utils};
use tera::Tera;
use new_rawr::responses::listing::SubmissionData;
use serde::{Serialize, Deserialize};
use diesel::{Connection};
use actix_session::{Session, CookieSession};
use std::rc::Rc;
use std::sync::{Mutex, Arc};
use std::cell::RefCell;
use actix_web_actors::ws::{CloseReason, CloseCode};
use crate::schema::users::dsl::created;
use new_rawr::client::RedditClient;
use new_rawr::auth::AnonymousAuthenticator;
use crate::models::{User, Level, Status};
use new_rawr::structures::submission::Submission;
use new_rawr::traits::{Votable, Content};
use rand::Rng;
use rand::distributions::Alphanumeric;
use serde_json::Value;
use actix_web::web::Form;
use std::collections::HashMap;
use serde_json::Number;
use actix_web::error::ParseError::Header;
use actix_web::http::{HeaderName, HeaderMap};
use crate::websiteerror::WebsiteError;
use crate::siteerror::SiteError;
use bcrypt::verify;
use crate::usererror::UserError;
use crate::siteerror::SiteError::DBError;
use crate::apiresponse::{APIResponse, APIError};
use std::str::FromStr;

fn api_validate(header_map: &HeaderMap, level: Level, conn: &MysqlConnection) -> Result<bool, Box<dyn WebsiteError>> {
    let option = header_map.get("Authorization");
    if option.is_none() {
        return Ok(false);
    }
    let x = option.unwrap().to_str();
    if x.is_err() {}
    let header = x.unwrap().to_string();

    let split = header.split(" ").collect::<Vec<&str>>();
    let option = split.get(0);
    if option.is_none() {
        return Ok(false);
    }
    let value = split.get(1);
    if value.is_none() {
        return Ok(false);
    }
    let value = value.unwrap().to_string();
    let key = option.unwrap().to_string();
    if key.eq("Basic") {
        if level == Level::Client {
            let x1 = value.split(":").collect::<Vec<&str>>();
            let id = x1.get(0);
            if id.is_none() {
                return Ok(false);
            }
            let id = id.unwrap();
            let key = x1.get(1);
            if key.is_none() {
                return Ok(false);
            }
            let key = key.unwrap();
            let result = action::get_client_key_by_id(i64::from_str(id.clone()).unwrap(), conn);
            if result.is_err() {
                return Err(Box::new(SiteError::DBError(result.err().unwrap())));
            }
            let client = result.unwrap();
            if client.is_none() {
                return Ok(false);
            }
            return Ok(verify(&key, &client.unwrap().api_key).unwrap());
        } else {
            return Ok(false);
        }
    } else if key.eq("Bearer") {
        if level == Level::Client {
            return Ok(false);
        }
        let result1 = utils::is_authorized(key, level, conn);
        if (result1.is_err()) {
            return Err(result1.err().unwrap());
        }
        return Ok(result1.unwrap());
    }
    return Ok(false);
}

fn get_user_by_header(header_map: &HeaderMap, conn: &MysqlConnection) -> Result<Option<User>, Box<dyn WebsiteError>> {
    let option = header_map.get("Authorization");
    if option.is_none() {
        return Ok(None);
    }
    let x = option.unwrap().to_str();
    if x.is_err() {}
    let header = x.unwrap().to_string();

    let split = header.split(" ").collect::<Vec<&str>>();
    let option = split.get(0);
    if option.is_none() {
        return Ok(None);
    }
    let value = split.get(1);
    if value.is_none() {
        return Ok(None);
    }
    let value = value.unwrap().to_string();
    let key = option.unwrap().to_string();
    if key.eq("Bearer") {
        let result = action::get_user_from_auth_token(value, conn);
        if result.is_err() {
            return Err(Box::new(SiteError::DBError(result.err().unwrap())));
        }
        return Ok(result.unwrap());
    }
    Ok(None)
}

#[get("/api/user/{user}")]
pub async fn get_user(pool: web::Data<DbPool>, web::Path((user)): web::Path<( String)>, r: HttpRequest) -> HttpResponse {
    let conn = pool.get().expect("couldn't get db connection from pool");
    let result = api_validate(r.headers(), Level::Client, &conn);
    if result.is_err() {
        return result.err().unwrap().api_error();
    }
    if !result.unwrap() {
        return UserError::NotFound.api_error();
    }
    let result1 = action::get_user_by_name(user, &conn);
    if result1.is_err() {
        return DBError(result1.err().unwrap()).api_error();
    }
    let result1 = result1.unwrap();
    if result1.is_none() {
        return UserError::NotFound.api_error();
    }
    let user = result1.unwrap();
    let response = APIResponse::<User> {
        success: true,
        data: Some(user),
    };
    HttpResponse::Ok().content_type("application/json").body(serde_json::to_string(&response).unwrap())
}

#[derive(Serialize, Deserialize)]
pub struct UserSuggest {
    pub username: String,
}

#[post("/api/user/submit")]
pub async fn submit_user(pool: web::Data<DbPool>, suggest: web::Form<UserSuggest>, r: HttpRequest) -> HttpResponse {
    let conn = pool.get().expect("couldn't get db connection from pool");
    let result = api_validate(r.headers(), Level::User, &conn);
    if result.is_err() {
        return result.err().unwrap().api_error();
    }
    if !result.unwrap() {
        return UserError::NotFound.api_error();
    }
    let result1 = action::get_user_by_name(suggest.username.clone(), &conn);
    if result1.is_err() {
        return DBError(result1.err().unwrap()).api_error();
    }
    let mut map = HashMap::<String, Value>::new();

    let mut user = result1.unwrap();
    map.insert("success".to_string(), Value::from("true"));
    if user.is_none() {
        let discoverer = get_user_by_header(&r.headers(), &conn);
        if discoverer.is_err() {
            return discoverer.err().unwrap().api_error();
        }
        utils::quick_add(suggest.username.clone(), discoverer.unwrap().unwrap().username.clone(), &conn);
        let result1 = action::get_user_by_name(suggest.username.clone(), &conn);
        if result1.is_err() {
            return DBError(result1.err().unwrap()).api_error();
        }
        user = result1.unwrap();
        map.insert("status".to_string(), Value::from("added"));
    } else {
        map.insert("status".to_string(), Value::from("already_added"));
    }
    HttpResponse::Ok().content_type("application/json").body(serde_json::to_string(&map).unwrap())
}

#[derive(Serialize, Deserialize)]
pub struct APILoginRequest {
    pub username: String,
    pub password: Option<String>,
}

#[post("/api/login")]
pub async fn user_login(pool: web::Data<DbPool>, login: web::Form<APILoginRequest>, rr: web::Data<Arc<Mutex<RedditRoyalty>>>, r: HttpRequest) -> HttpResponse {
    let conn = pool.get().expect("couldn't get db connection from pool");
    let result = action::get_user_by_name(login.username.clone(), &conn);
    if result.is_err() {
        return SiteError::DBError(result.err().unwrap()).api_error();
    }
    let user = result.unwrap();
    if user.is_none() {
        return UserError::NotAuthorized.api_error();
    }
    let user = user.unwrap();
    if login.password.is_none() {
        utils::send_login(&user, &conn, rr.clone());
        let mut map = HashMap::<String, Value>::new();
        map.insert("success".to_string(), Value::from(true));
        map.insert("status".to_string(), Value::from("SENT"));
        return HttpResponse::Ok().content_type("application/json").body(serde_json::to_string(&map).unwrap());
    } else {
        let string = login.password.as_ref().unwrap();
        if verify(&string, &user.password).unwrap() {
            let x = utils::create_token(&user, &conn);
            let mut map = HashMap::<String, Value>::new();
            map.insert("success".to_string(), Value::from(true));
            map.insert("status".to_string(), Value::from("AUTHORIZED"));
            map.insert("token".to_string(), Value::from(x.unwrap().token.clone()));
            return HttpResponse::Ok().content_type("application/json").body(serde_json::to_string(&map).unwrap());
        }
    }

    return UserError::NotAuthorized.api_error();
}

#[post("/api/validate/key")]
pub async fn validate_key(pool: web::Data<DbPool>, r: HttpRequest) -> Result<HttpResponse, Error> {
    let conn = pool.get().expect("couldn't get db connection from pool");
    let result = api_validate(r.headers(), Level::User, &conn);
    if !result.unwrap() {
        return Ok(UserError::NotAuthorized.api_error());
    }
    let mut map = HashMap::<String, Value>::new();
    map.insert("success".to_string(), Value::from(true));
    return Ok(HttpResponse::Ok().content_type("application/json").body(serde_json::to_string(&map).unwrap()));
}

#[derive(Deserialize)]
pub struct ChangeStatus {
    pub username: String,
    pub status: String,
}

#[post("/api/moderator/change/status")]
pub async fn change_status(pool: web::Data<DbPool>, suggest: web::Form<ChangeStatus>, r: HttpRequest) -> HttpResponse {
    let conn = pool.get().expect("couldn't get db connection from pool");
    let result = api_validate(r.headers(), Level::Moderator, &conn);
    if result.is_err() {
        return result.err().unwrap().api_error();
    }
    if !result.unwrap() {
        return UserError::NotFound.api_error();
    }
    let moderator = get_user_by_header(&r.headers(), &conn);
    if moderator.is_err() {
        return moderator.err().unwrap().api_error();
    }
    let moderator = moderator.unwrap().unwrap();
    let result1 = action::get_user_by_name(suggest.username.clone(), &conn);
    if result1.is_err() {
        return DBError(result1.err().unwrap()).api_error();
    }
    let result1 = result1.unwrap();
    if result1.is_none() {
        return UserError::NotFound.api_error();
    }
    let str = Status::from_str(suggest.status.as_str());
    if str.is_err() {
        return UserError::InvalidRequest.api_error();
    }
    let status: Status = str.unwrap();
    if status == Status::APPROVED {
        //TODO approve user
    }
    let mut user = result1.unwrap();
    user.set_status(status.to_string());
    user.set_moderator(moderator.username.clone());
    let result = action::update_user(&user, &conn);
    if result.is_err() {
        return DBError(result.err().unwrap()).api_error();
    }
    let response = APIResponse::<User> {
        success: true,
        data: None,
    };
    return HttpResponse::Ok().content_type("application/json").body(serde_json::to_string(&response).unwrap());
}

#[derive(Deserialize)]
pub struct ChangeLevel {
    pub username: String,
    pub level: String,
}

#[post("/api/admin/change/level")]
pub async fn change_level(pool: web::Data<DbPool>, suggest: web::Form<ChangeLevel>, r: HttpRequest) -> HttpResponse {
    let conn = pool.get().expect("couldn't get db connection from pool");
    let result = api_validate(r.headers(), Level::Admin, &conn);
    if result.is_err() {
        return result.err().unwrap().api_error();
    }
    if !result.unwrap() {
        return UserError::NotFound.api_error();
    }
    let moderator = get_user_by_header(&r.headers(), &conn);
    if moderator.is_err() {
        return moderator.err().unwrap().api_error();
    }
    let moderator = moderator.unwrap().unwrap();
    let result1 = action::get_user_by_name(suggest.username.clone(), &conn);
    if result1.is_err() {
        return DBError(result1.err().unwrap()).api_error();
    }
    let result1 = result1.unwrap();
    if result1.is_none() {
        return UserError::NotFound.api_error();
    }
    let level: Result<Level, strum::ParseError> = Level::from_str(suggest.level.as_str());
    if level.is_err() {
        return UserError::InvalidRequest.api_error();
    }
    let mut user = result1.unwrap();
    user.set_level(suggest.level.clone());
    let result = action::update_user(&user, &conn);
    if result.is_err() {
        return DBError(result.err().unwrap()).api_error();
    }
    let response = APIResponse::<User> {
        success: true,
        data: None,
    };
    info!("{}", format!("{} has changed the level of {} to {}", moderator.username.clone(), user.username.clone(), level.unwrap().name()));
    return HttpResponse::Ok().content_type("application/json").body(serde_json::to_string(&response).unwrap());
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RedditUser {
    pub name: String,
    pub avatar: String,
    pub commentKarma: i64,
    pub total_karma: i64,
    pub created: i64,
    pub topFivePosts: Vec<RedditPost>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RedditPost {
    pub subreddit: String,
    pub url: String,
    pub id: String,
    pub title: String,
    pub content: String,
    pub score: i64,

}


#[post("/api/moderator/next/user")]
pub async fn next_user(pool: web::Data<DbPool>, r: HttpRequest, rr: web::Data<Arc<Mutex<RedditRoyalty>>>) -> HttpResponse {
    let conn = pool.get().expect("couldn't get db connection from pool");
    let result = api_validate(r.headers(), Level::Moderator, &conn);
    if result.is_err() {
        return result.err().unwrap().api_error();
    }
    if !result.unwrap() {
        return UserError::NotFound.api_error();
    }
    let result = action::get_found_users(&conn);
    if result.is_err() {}
    let mut vec = result.unwrap();
    vec.sort_by_key(|x| x.created);
    let rr = rr.lock();
    if rr.is_err() {
        actix::System::current().stop();
        panic!("The Site Core has been poisoned. Tux you dumb fuck!");
    }
    let mut rr = rr.unwrap();
    let client = RedditClient::new("RedditNobility bot(by u/KingTuxWH)", AnonymousAuthenticator::new());
    let mut option: Option<User> = Option::None;
    for x in vec {
        if !rr.users_being_worked_on.contains_key(&x.id) {
            option = Some(x.clone());
        }
    }
    if option.is_none() {
        return UserError::NotFound.api_error();
    }
    let x1: User = option.unwrap();
    rr.add_id(x1.id.clone());
    let user = client.user(x1.username.as_str());

    let result1 = user.about();
    if result1.is_err() {
        let error = APIError {
            status_code: None,
            user_friendly_message: None,
            error_code: Some("USER_WAS_UNABLE_TO_BE_RETRIEVED".to_string()),
        };
        let response = APIResponse::<APIError> {
            success: false,
            data: Some(error),
        };
        action::delete_user(x1.username.clone(), &conn);
        return HttpResponse::Ok().content_type("application/json").body(serde_json::to_string(&response).unwrap());
    }
    let final_user = result1.unwrap();
    let user = client.user(x1.username.as_str());

    let submissions = user.submissions().unwrap().take(5).collect::<Vec<Submission>>();
    let mut user_posts = Vec::<RedditPost>::new();

    for x in submissions {
        let post = RedditPost {
            subreddit: x.subreddit().name,
            url: format!("https://reddit.com{}", x.data.permalink),
            id: x.data.id.clone(),
            title: x.title().clone().to_string(),
            content: x.data.selftext.clone().to_string(),
            score: x.score(),
        };
        user_posts.push(post);
    }
    let user = RedditUser {
        name: final_user.data.name,
        avatar: final_user.data.icon_img.unwrap_or("".parse().unwrap()),
        commentKarma: final_user.data.comment_karma,
        total_karma: final_user.data.total_karma,
        created: final_user.data.created as i64,
        topFivePosts: user_posts,
    };
    let response = APIResponse::<RedditUser> {
        success: true,
        data: Some(user),
    };
    return HttpResponse::Ok().content_type("application/json").body(serde_json::to_string(&response).unwrap());

}
