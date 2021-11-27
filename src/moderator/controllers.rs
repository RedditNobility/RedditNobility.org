use actix_web::{get, post, web, HttpRequest};

use crate::api_response::{APIResponse, SiteResponse};
use crate::{Database, User, RN, utils};

use crate::error::response::{bad_request, not_found, unauthorized};
use crate::user::action::{delete_user, get_found_users, get_user_by_name, update_properties};
use crate::user::utils::get_user_by_header;
use serde::{Deserialize, Serialize};
use std::str::FromStr;
use actix_web::http::StatusCode;
use actix_web::web::Json;
use chrono::{Datelike, NaiveDate, NaiveDateTime, NaiveTime, Utc};
use log::{debug, error, trace};
use rraw::utils::error::APIError;

use strum::ParseError;
use crate::moderator::action::{get_approve_count, get_approve_count_total, get_discover_count, get_discover_count_total};

use crate::user::models::{Status};
use crate::utils::{get_current_time, yeet};

#[derive(Debug, Serialize, Deserialize)]
pub struct UserStats {
    pub users_discovered: i64,
    pub users_discovered_this_month: i64,
    pub users_reviewed: i64,
    pub users_reviewed_this_month: i64,

}

#[get("/moderator/user/{user}")]
pub async fn user_page(
    database: Database,
    path: web::Path<String>,
    req: HttpRequest,
) -> SiteResponse {
    let username = path.into_inner();
    let connection = database.get()?;
    let user = get_user_by_header(req.headers(), &connection)?;
    if user.is_none() {
        return unauthorized();
    }
    let user = user.unwrap();
    if !user.permissions.moderator {
        return unauthorized();
    }
    let lookup = get_user_by_name(&username, &connection)?;
    return APIResponse::<User>::respond_new(lookup, &req);
}

#[get("/moderator/user/{user}/stats")]
pub async fn user_stats(
    database: Database,
    path: web::Path<String>,
    req: HttpRequest,
) -> SiteResponse {
    let username = path.into_inner();
    let connection = database.get()?;
    let me = get_user_by_header(req.headers(), &connection)?;

    if me.is_none() {
        return unauthorized();
    }
    let me = me.unwrap();
    let lookup = get_user_by_name(&username, &connection)?;
    if lookup.is_none() {
        return not_found();
    }
    let lookup = lookup.unwrap();
    if !me.username.eq(&lookup.username) {
        if !me.permissions.moderator {
            return unauthorized();
        }
    }
    let lookup = get_user_by_name(&username, &connection)?.unwrap();
    let i = get_month_timestamp();

    let user_stats = UserStats {
        users_discovered: get_discover_count(&lookup.username, 0, &connection)?,
        users_discovered_this_month: get_discover_count(&lookup.username, i.clone(), &connection)?,
        users_reviewed: get_approve_count(&lookup.username, 0, &connection)?,
        users_reviewed_this_month: get_approve_count(&lookup.username, i.clone(), &connection)?,
    };

    return APIResponse::<UserStats>::respond_new(Some(user_stats), &req);
}

fn get_month_timestamp() -> i64 {
    let date = Utc::today();
    let new_month = NaiveDate::from_ymd(date.year(), date.month(), 1);
    let time = NaiveDateTime::new(new_month, NaiveTime::from_hms(0, 0, 0));
    debug!("Month Value {} UnixTime {}", &time, time.timestamp_millis());

    return time.timestamp_millis();
}


#[get("/moderator/stats")]
pub async fn system_stats(
    database: Database,
    req: HttpRequest,
) -> SiteResponse {
    let connection = database.get()?;
    let me = get_user_by_header(req.headers(), &connection)?;
    if me.is_none() {
        // return unauthorized();
    }
    //let me = me.unwrap();

    // if !me.permissions.moderator {
    //    return unauthorized();
    // }
    let i = get_month_timestamp();
    let users_stats = UserStats {
        users_discovered: get_discover_count_total(0, &connection)?,
        users_discovered_this_month: get_discover_count_total(i.clone(), &connection)?,
        users_reviewed: get_approve_count_total(0, &connection)?,
        users_reviewed_this_month: get_approve_count_total(i.clone(), &connection)?,
    };

    return APIResponse::<UserStats>::respond_new(Some(users_stats), &req);
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RedditUser {
    pub name: String,
    pub avatar: String,
    pub comment_karma: i64,
    pub total_karma: i64,
    pub created: i64,
    pub top_posts: Vec<RedditPost>,
    pub top_comments: Vec<Comment>,
    pub user: User,

}

#[derive(Debug, Serialize, Deserialize)]
pub struct RedditPost {
    pub subreddit: String,
    pub url: String,
    pub id: String,
    pub title: String,
    pub content: RedditContent,
    pub score: i64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Comment {
    pub subreddit: String,
    pub url: String,
    pub id: String,
    pub og_post_title: String,
    pub content: String,
    pub score: i64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RedditContent {
    pub content: Option<String>,
    pub url: Option<String>,
    pub over_18: bool,

}

#[get("/api/moderator/review/{user}")]
pub async fn review_user(
    database: Database,
    path: web::Path<String>,
    req: HttpRequest,
    rr: RN,
) -> SiteResponse {
    let username = path.into_inner();
    let conn = database.get()?;
    let user = get_user_by_header(req.headers(), &conn)?;
    if user.is_none() {
        return unauthorized();
    }
    let user = user.unwrap();
    if !user.permissions.review_user {
        return unauthorized();
    }
    let rn = rr.lock()?;
    let user = if username.eq("next") {
        trace!("Looking for Next User");
        let mut result = get_found_users(&conn)?;
        result.sort_by_key(|x| x.created);
        let mut v = None;
        for i in 0..result.len() {
            let user = result.remove(i);
            if !rn.users_being_worked_on.contains_key(&user.id) {
                trace!("User Found {}", &user.username);
                v = Some(user);
                break;
            }
        }
        if v.is_none() {
            trace!("Unable to find User {}", &username);
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
//TODO re-add this one line
    // rn.add_id(user.id);
    let r_user = rn.reddit.user(user.username.clone());
    trace!("Grabbing About Data for {}", &&user.username);
    let about = rn.reddit.user(user.username.clone()).about().await;

    if let Err(error) = about {
        error!("Failed to grab about data for {} error {}", &user.username, &error);
        match error {
            APIError::HTTPError(http) => {
                if http.eq(&StatusCode::NOT_FOUND) {
                    delete_user(&user.username, &conn)?;
                    return bad_request("We have fixed the issue please try again");
                }
            }
            _ => {}
        }
        return Err(error.into());
    }
    let about = about.unwrap();
    let mut user_posts = Vec::<RedditPost>::new();
    let mut user_comments = Vec::<Comment>::new();
    if !about.data.is_suspended.unwrap_or(false) {
        let submissions = r_user
            .submissions(None).await?;
        let comments = r_user
            .comments(None).await?;
        yeet(rn);


        for x in comments.data.children {
            let x = x.data;
            let post = Comment {
                subreddit: x.subreddit,
                url: format!("https://reddit.com{}", x.permalink.unwrap()),
                id: x.id.clone(),
                og_post_title: x.link_title.clone(),
                content: x.body,
                score: x.score as i64,
            };
            user_comments.push(post);
        }
        for x in submissions.data.children {
            let x = x.data;
            let text = x.selftext;
            let content = if text.is_empty() {
                RedditContent { content: None, url: x.url, over_18: x.over_18 }
            } else {
                RedditContent { content: Some(text), url: None, over_18: x.over_18 }
            };
            let post = RedditPost {
                subreddit: x.subreddit,
                url: format!("https://reddit.com{}", x.permalink),
                id: x.id.clone(),
                title: x.title.clone(),
                content: content,
                score: x.score as i64,
            };
            user_posts.push(post);
        }
    }
    let user = RedditUser {
        name: about.data.name,
        avatar: about.data.icon_img.unwrap_or("".parse().unwrap()),
        comment_karma: about.data.comment_karma.unwrap_or(0),
        total_karma: about.data.total_karma.unwrap_or(0),
        created: about.data.created.unwrap_or(0.0) as i64,
        top_posts: user_posts,
        top_comments: user_comments,
        user,
    };
    let response = APIResponse::<RedditUser> {
        success: true,
        data: Some(user),
        status_code: Some(200),
    };
    response.respond(&req)
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ApproveRequest {
    pub title: Option<String>,
}

#[post("/api/moderator/review/{username}/{status}")]
pub async fn review_user_update(
    database: Database,
    value: web::Path<(String, String)>,
    req: HttpRequest,
    rn: RN,
) -> SiteResponse {
    let (username, status) = value.into_inner();
    let conn = database.get()?;
    let user = get_user_by_header(req.headers(), &conn)?;
    if user.is_none() {
        return unauthorized();
    }

    let reviewer = user.unwrap();
    if !reviewer.permissions.review_user {
        return unauthorized();
    }
    trace!("Setting the User: {} Status {}", &username, &status);

    let option = get_user_by_name(&username, &conn)?;
    if option.is_none() {
        return not_found();
    }

    let str: Result<Status, ParseError> = Status::from_str(status.as_str());
    if str.is_err() {
        return bad_request("Approved or Denied".to_string());
    }
    let user2 = option.unwrap();

    let status = str.unwrap();
    if status == Status::Approved {
        trace!("Attempting to Approve User {} on Reddit", &user2.username);

        let rr = rn.lock()?;
        let user1 = utils::approve_user(&user2, &rr.reddit).await;
        yeet(rr);
        if !user1 {
            error!("Approval Failure");
            return crate::error::response::error("Unable to Process Approve Request Currently", Some(StatusCode::INTERNAL_SERVER_ERROR));
        }
    }
    let mut properties = user2.properties;
    let x: ApproveRequest = serde_qs::from_str(req.query_string()).unwrap();
    if let Some(title) = x.title {
        debug!("Changing {} title to {}", &user2.username, &title);
        properties.title = Some(title);
        update_properties(&user2.id, properties, &conn)?;
    }
    crate::moderator::action::update_status(&user2.id, status, &reviewer.username, get_current_time(), &conn)?;
    return APIResponse::new(true, Some(true)).respond(&req);
}


#[derive(serde::Deserialize)]
pub struct ChangeRequest {
    pub value: String,
}

#[post("/api/moderator/update/{user}/{key}")]
pub async fn moderator_update_properties(
    database: Database,
    request: Json<ChangeRequest>,
    path: web::Path<(String, String)>,
    r: HttpRequest,
) -> SiteResponse {
    let (username, key) = path.into_inner();

    let conn = database.get()?;
    let option = get_user_by_header(r.headers(), &conn)?;
    if option.is_none() {
        return unauthorized();
    }
    let modetator = option.unwrap();
    if !modetator.permissions.moderator {
        return unauthorized();
    }
    // Update User
    let option = get_user_by_name(&username, &conn)?;
    if option.is_none() {
        return not_found();
    }
    let mut user = option.unwrap();
    let value = request.0.value;
    match key.as_str() {
        "avatar" => {
            user.properties.set_avatar(value);
        }
        "description" => {
            user.properties.set_description(value);
        }
        _ => {
            return bad_request("You can only change your Avatar or Description");
        }
    }
    update_properties(&user.id, user.properties, &conn)?;
    return APIResponse::new(true, Some(true)).respond(&r);
}
