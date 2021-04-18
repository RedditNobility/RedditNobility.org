use std::time::{Duration, Instant};


use actix::prelude::*;
use actix_files as fs;
use actix_web::{middleware, get, web, App, Error, HttpRequest, HttpResponse, HttpServer};
use actix_web_actors::ws;
use crate::{DbPool, RedditRoyalty, action};
use tera::Tera;
use new_rawr::responses::listing::SubmissionData;
use serde::{Serialize, Deserialize};
use diesel::{MysqlConnection, Connection};
use actix_session::{Session, CookieSession};
use std::rc::Rc;
use std::sync::{Mutex, Arc};
use std::cell::RefCell;
use actix_web_actors::ws::{CloseReason, CloseCode};
use crate::schema::users::dsl::created;
use new_rawr::client::RedditClient;
use new_rawr::auth::AnonymousAuthenticator;
use crate::models::User;
use new_rawr::structures::submission::Submission;
use new_rawr::traits::{Votable, Content};
use rand::Rng;
use rand::distributions::Alphanumeric;
use serde_json::Value;
use actix::prelude::*;
use std::borrow::BorrowMut;
use std::collections::HashMap;

/// How often heartbeat pings are sent
const HEARTBEAT_INTERVAL: Duration = Duration::from_secs(5);
/// How long before lack of client response causes a timeout
const CLIENT_TIMEOUT: Duration = Duration::from_secs(10);

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

#[derive(Deserialize)]
pub struct WebsocketRequest {
    moderator: String,
}

/// do websocket handshake and start `MyWebSocket` actor
pub async fn ws_index(r: HttpRequest, rr: web::Data<Arc<Mutex<RedditRoyalty>>>, stream: web::Payload) -> Result<HttpResponse, Error> {
    let data = rr.as_ref().clone();

    let res = ws::start(MyWebSocket::new(data), &r, stream);
    res
}

#[get("/moderator")]
pub async fn moderator_index(pool: web::Data<DbPool>, mut rr: web::Data<Arc<Mutex<RedditRoyalty>>>, session: Session, tera: web::Data<Tera>, r: HttpRequest) -> Result<HttpResponse, Error> {
    let mut ctx = tera::Context::new();
    let conn = pool.get().expect("couldn't get db connection from pool");

    let result2: Option<String> = session.get("moderator").unwrap();
    if result2.is_none() {
        return Err(HttpResponse::Unauthorized().into());
    }
    let s: String = rand::thread_rng()
        .sample_iter(&Alphanumeric)
        .take(25)
        .map(char::from)
        .collect();
    let mut data = rr.as_ref().clone();

    let result1: String = session.get("moderator").unwrap().unwrap();
    let moderator = action::get_moderator(result1, &conn).unwrap().unwrap();
    data.borrow_mut().lock().unwrap().add_key(s.clone(), moderator.id);
    ctx.insert("mod_key", &s);
    ctx.insert("moderator", &moderator.username.clone());
    let string = std::env::var("WEBSOCKET_URL").unwrap();
    println!("{}", &string);
    ctx.insert("web_socket_url", &format!("{}/ws/moderator", string));
    let result = tera.get_ref().render("moderator.html", &ctx);
    Ok(HttpResponse::Ok().content_type("text/html").body(&result.unwrap()))
}


impl Actor for MyWebSocket {
    type Context = ws::WebsocketContext<Self>;

    /// Method is called on actor start. We start the heartbeat process here.
    fn started(&mut self, ctx: &mut Self::Context) {
        self.hb(ctx);
    }
}

fn approve_user(user: &str, moderator: &str, client: &RedditClient, conn: &MysqlConnection) -> bool {
    let result1 = client.subreddit("RedditNobility").invite_member(user.parse().unwrap());
    if result1.is_err() {
        return false;
    }
    if result1.unwrap() == false {
        return false;
    }
    let result = action::get_fuser(user.parse().unwrap(), &conn);
    let option = result.unwrap();
    if option.is_none() {
        return false;
    }
    client.subreddit("RedditNobility").invite_member(user.parse().unwrap());
    print!("Updating!");
    action::update_fuser("Approved".to_string(), moderator.to_string(), user.to_string(), conn);
    return true;
}

fn deny_user(user: &str, moderator: &str, conn: &MysqlConnection) {
    let result = action::get_fuser(user.parse().unwrap(), &conn);
    let option = result.unwrap();
    if option.is_none() {
        return;
    }
    action::update_fuser("Denied".to_string(), moderator.to_string(), user.to_string(), conn);
}

struct MyWebSocket {
    /// Client must send ping at least once per 10 seconds (CLIENT_TIMEOUT),
    /// otherwise we drop connection.
    hb: Instant,
    conn: MysqlConnection,
    key: Option<String>,
    reddit_royalty: Arc<Mutex<RedditRoyalty>>,

}

impl StreamHandler<Result<ws::Message, ws::ProtocolError>> for MyWebSocket {
    fn handle(&mut self, msg: Result<ws::Message, ws::ProtocolError>, ctx: &mut Self::Context) {

        // process websocket messages
        println!("WS: {:?}", msg);
        match msg {
            Ok(ws::Message::Ping(msg)) => {
                self.hb = Instant::now();
                ctx.pong(&msg);
            }
            Ok(ws::Message::Pong(_)) => {
                self.hb = Instant::now();
            }
            Ok(ws::Message::Text(text)) => {
                let value: Value = serde_json::from_str(&*text).unwrap();
                let value1 = value["type"].as_str().unwrap();
                if value1.eq("approve") {
                    let value2 = value["user"].as_str();
                    if value2.is_some() {
                        let user1 = approve_user(value2.unwrap(), value["moderator"].as_str().unwrap(), &self.reddit_royalty.lock().unwrap().reddit, &self.conn);
                        if !user1 {
                            let mut values = HashMap::<String, Value>::new();
                            values.insert("type".parse().unwrap(), "error".parse().unwrap());
                            values.insert("error".parse().unwrap(), Value::String(format!("Unable to approve user: {}", value2.unwrap())));
                            ctx.text(serde_json::to_string(&values).unwrap())
                        }
                    }
                } else if value1.eq("deny") {
                    let value2 = value["user"].as_str();
                    if value2.is_some() {
                        deny_user(value2.unwrap(), value["moderator"].as_str().unwrap(), &self.conn);
                    }
                } else if value1.eq("user") {
                    println!("Hey");
                    let result = action::get_found_fusers(&self.conn);
                    let mut vec = result.unwrap();
                    vec.sort_by_key(|x| x.created);
                    let client = RedditClient::new("RedditNobility bot(by u/KingTuxWH)", AnonymousAuthenticator::new());
                    let option = vec.get(0);
                    if option.is_none() {
                        return;
                    }
                    let x1: &User = option.unwrap();
                    let user = client.user(x1.username.as_str());
                    let result1 = user.about();
                    if result1.is_err() {
                        let mut values = HashMap::<String, Value>::new();
                        values.insert("type".parse().unwrap(), "error".parse().unwrap());
                        values.insert("error".parse().unwrap(), Value::String(format!("Unable to load user: {}", value2.unwrap())));
                        ctx.text(serde_json::to_string(&values).unwrap());
                        return;
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
                    let mut values = HashMap::<String, Value>::new();
                    values.insert("type".parse().unwrap(), Value::String("user".parse().unwrap()));
                    values.insert("data".parse().unwrap(), serde_json::to_value(&user).unwrap());
                    ctx.text(serde_json::to_string(&values).unwrap())
                } else if value1.eq("login") {
                    let x = self.set_key(value["key"].as_str().unwrap().to_string());
                    if !x {
                        ctx.close(Option::from(CloseReason::from(CloseCode::Invalid)));
                        ctx.stop();
                    } else {
                        println!("Logged in moderator")
                    }
                }
            }
            Ok(ws::Message::Binary(bin)) => ctx.binary(bin),
            Ok(ws::Message::Close(reason)) => {
                ctx.close(reason);
                ctx.stop();
                if self.key.is_some() {
                    self.reddit_royalty.lock().unwrap().drop_key(self.get_key())
                }
            }
            _ => {
                ctx.stop();
                if self.key.is_some() {
                    self.reddit_royalty.lock().unwrap().drop_key(self.get_key())
                }
            }
        }
    }
}


impl MyWebSocket {
    fn new(rr: Arc<Mutex<RedditRoyalty>>) -> Self {
        let string = std::env::var("DATABASE_URL").expect("DATABASE_URL");
        Self {
            hb: Instant::now(),
            conn: MysqlConnection::establish(&*string).unwrap(),
            reddit_royalty: rr,
            key: None,
        }
    }

    /// helper method that sends ping to client every second.
    ///
    /// also this method checks heartbeats from client
    fn hb(&self, ctx: &mut <Self as Actor>::Context) {
        ctx.run_interval(HEARTBEAT_INTERVAL, |act, ctx| {
            // check client heartbeats
            if Instant::now().duration_since(act.hb) > CLIENT_TIMEOUT {
                // heartbeat timed out
                println!("Websocket Client heartbeat failed, disconnecting!");

                // stop actor
                ctx.stop();

                // don't try to send a ping
                return;
            }

            ctx.ping(b"");
        });
    }
    fn get_key(&self) -> String {
        let string = self.key.as_ref().unwrap().clone();
        return string;
    }
    fn set_key(&mut self, key: String) -> bool {
        if !self.reddit_royalty.lock().unwrap().is_key_valid(key.clone()) {
            return false;
        }
        self.key = Option::from(key);
        return true;
    }
}
