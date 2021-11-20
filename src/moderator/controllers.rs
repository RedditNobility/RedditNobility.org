use actix_web::{get, web, HttpRequest};

use crate::api_response::{APIResponse, SiteResponse};
use crate::{Database, User, RN};
use new_rawr::structures::submission::Submission;
use new_rawr::traits::{Content, Votable};

use crate::error::response::{not_found, unauthorized};
use crate::user::action::{get_found_users, get_user_by_name};
use crate::user::utils::get_user_by_header;
use serde::{Deserialize, Serialize};

use crate::user::models::Level;

#[get("/moderator/user/{user}")]
pub async fn user_page(
    database: Database,
    web::Path(username): web::Path<String>,
    req: HttpRequest,
) -> SiteResponse {
    let connection = database.get()?;
    let user = get_user_by_header(req.headers(), &connection)?;
    if user.is_none() {
        return unauthorized();
    }
    let user = user.unwrap();
    if user.level == Level::User {
        return unauthorized();
    }
    let lookup = get_user_by_name(&username, &connection)?;
    return APIResponse::<User>::respond_new(lookup, &req);
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
pub async fn review_user(
    database: Database,
    web::Path(username): web::Path<String>,
    req: HttpRequest,
    rr: RN,
) -> SiteResponse {
    let conn = database.get()?;
    let user = get_user_by_header(req.headers(), &conn)?;
    if user.is_none() {
        return unauthorized();
    }
    let user = user.unwrap();
    if user.level == Level::User {
        return unauthorized();
    }
    let mut rn = rr.lock()?;
    let user = if username.eq("next") {
        let mut result = get_found_users(&conn)?;
        result.sort_by_key(|x| x.created);
        let mut v = None;
        for i in 0..result.len() {
            let user = result.remove(i);
            if !rn.users_being_worked_on.contains_key(&user.id) {
                v = Some(user);
                break;
            }
        }
        if v.is_none() {
            return not_found();
        }
        v.unwrap()
    } else {
        let user = get_user_by_name(&username, &conn)?;
        if user.is_none() {
            return not_found();
        }
        user.unwrap()
    };

    rn.add_id(user.id);
    let r_user = rn.reddit.user(&user.username);
    let about = rn.reddit.user(&user.username).about()?;

    let submissions = r_user
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
            title: x.data.title.clone(),
            content: x.data.selftext.clone().to_string(),
            score: x.score(),
        };
        user_posts.push(post);
    }
    let user = RedditUser {
        name: about.data.name,
        avatar: about.data.icon_img.unwrap_or("".parse().unwrap()),
        commentKarma: about.data.comment_karma,
        total_karma: about.data.total_karma,
        created: about.data.created as i64,
        topFivePosts: user_posts,
    };
    let response = APIResponse::<RedditUser> {
        success: true,
        data: Some(user),
        status_code: Some(200),
    };
    response.respond(&req)
}
