


use crate::api::api_validate;
use crate::api::apiresponse::{APIError, APIResponse};
use crate::api::get_user_by_header;
use crate::models::{Level, Status, User};


use crate::siteerror::SiteError::DBError;
use crate::usererror::UserError;
use crate::websiteerror::WebsiteError;
use crate::{action, utils, DbPool, RedditRoyalty};
use actix::prelude::*;




use actix_web::{
    get, http, middleware, post, web, App, Error, HttpRequest, HttpResponse, HttpServer,
};



use new_rawr::auth::AnonymousAuthenticator;
use new_rawr::client::RedditClient;

use new_rawr::structures::submission::Submission;
use new_rawr::traits::{Content, Votable};


use serde::{Deserialize, Serialize};





use std::str::FromStr;
use std::sync::{Arc, Mutex};


#[derive(Serialize, Deserialize)]
pub struct GetUser {
    pub username: String,
}

#[get("/api/user/{user}")]
pub async fn get_user(
    pool: web::Data<DbPool>,
    web::Path(user): web::Path<String>,
    r: HttpRequest,
) -> HttpResponse {
    let conn = pool.get().expect("couldn't get db connection from pool");
    let result = api_validate(r.headers(), Level::Moderator, &conn);
    if result.is_err() {
        return result.err().unwrap().api_error();
    }

    if !result.unwrap() {
        return UserError::NotFound.api_error();
    }
    let result1 = action::get_user_by_name(user.clone(), &conn);
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
    HttpResponse::Ok()
        .content_type("application/json")
        .body(serde_json::to_string(&response).unwrap())
}

#[derive(Deserialize)]
pub struct ChangeStatus {
    pub username: String,
    pub status: String,
}

#[post("/api/moderator/change/status")]
pub async fn change_status(
    pool: web::Data<DbPool>,
    suggest: web::Form<ChangeStatus>,
    rr: web::Data<Arc<Mutex<RedditRoyalty>>>,
    r: HttpRequest,
) -> HttpResponse {
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

    let mut user = result1.unwrap();
    if status == Status::Approved {
        let rr = rr.lock();
        if rr.is_err() {
            panic!("The Site Core has been poisoned. Tux you dumb fuck!");
        }
        let user1 = utils::approve_user(&user, &rr.unwrap().reddit);
        if !user1 {
            let error = APIError {
                status_code: None,
                user_friendly_message: None,
                error_code: Some("FAILED_APPROVE".to_string()),
            };
            let response = APIResponse::<APIError> {
                success: true,
                data: Some(error),
            };
            return HttpResponse::Ok()
                .content_type("application/json")
                .body(serde_json::to_string(&response).unwrap());
        }
    }
    user.set_status(status);
    user.set_moderator(moderator.username.clone());
    let result = action::update_user(&user, &conn);
    if result.is_err() {
        return DBError(result.err().unwrap()).api_error();
    }
    let response = APIResponse::<User> {
        success: true,
        data: None,
    };
    return HttpResponse::Ok()
        .content_type("application/json")
        .body(serde_json::to_string(&response).unwrap());
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

#[get("/api/moderator/review/{user}")]
pub async fn next_user(
    pool: web::Data<DbPool>,
    r: HttpRequest,
    web::Path(user): web::Path<String>,
    rr: web::Data<Arc<Mutex<RedditRoyalty>>>,
) -> HttpResponse {
    let conn = pool.get().expect("couldn't get db connection from pool");
    let result = api_validate(r.headers(), Level::Moderator, &conn);
    if result.is_err() {
        return result.err().unwrap().api_error();
    }
    if !result.unwrap() {
        println!("GHey");
        return UserError::NotFound.api_error();
    }
    let rr = rr.lock();
    if rr.is_err() {
        actix::System::current().stop();
        panic!("The Site Core has been poisoned. Tux you dumb fuck!");
    }
    let mut rr = rr.unwrap();
    let client = RedditClient::new(
        "RedditNobility bot(by u/KingTuxWH)",
        AnonymousAuthenticator::new(),
    );
    let mut option: Option<User> = Option::None;
    if user.eq("next") {
        let result = action::get_found_users(&conn);
        if result.is_err() {}
        let mut vec = result.unwrap();
        vec.sort_by_key(|x| x.created);

        for x in vec {
            if !rr.users_being_worked_on.contains_key(&x.id) {
                option = Some(x.clone());
            }
        }
    } else {
        let result1 = action::get_user_by_name(user.clone(), &conn);
        if result1.is_err() {
            return DBError(result1.err().unwrap()).api_error();
        }
        let result1 = result1.unwrap();
        if result1.is_none() {
            return UserError::NotFound.api_error();
        }
        option = result1;
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
        return HttpResponse::Ok()
            .content_type("application/json")
            .body(serde_json::to_string(&response).unwrap());
    }
    let final_user = result1.unwrap();
    let user = client.user(x1.username.as_str());

    let submissions = user
        .submissions()
        .unwrap()
        .take(5)
        .collect::<Vec<Submission>>();
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
    return HttpResponse::Ok()
        .content_type("application/json")
        .body(serde_json::to_string(&response).unwrap());
}
