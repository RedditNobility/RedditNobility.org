use std::time::{Duration, Instant};


use actix::prelude::*;
use actix_files as fs;
use actix_web::{middleware, get, web, App, Error, HttpRequest, HttpResponse, HttpServer};
use actix_web_actors::ws;
use crate::{DbPool, RedditRoyalty};
use tera::Tera;
use new_rawr::responses::listing::SubmissionData;
use serde::{Serialize, Deserialize};
use diesel::{MysqlConnection, Connection};
use actix_session::{Session, CookieSession};
use std::rc::Rc;
use std::sync::{Mutex, Arc};
use std::cell::RefCell;

/// How often heartbeat pings are sent
const HEARTBEAT_INTERVAL: Duration = Duration::from_secs(5);
/// How long before lack of client response causes a timeout
const CLIENT_TIMEOUT: Duration = Duration::from_secs(10);

#[derive(Debug, Serialize, Deserialize)]
pub struct RedditUser {
    pub name: String,
    pub avatar: String,
    pub commentKarma: i64,
    pub linkKarma: i64,
    pub created: i64,
    pub topFivePosts: Vec<RedditPost>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RedditPost {
    pub subreddit: String,
    pub url: String,
    pub id: String,
    pub title: String,
    pub ups: i64,
    pub downs: i64,

}

#[derive(Deserialize)]
pub struct WebsocketRequest {
    moderator: String,
}

/// do websocket handshake and start `MyWebSocket` actor
pub async fn ws_index(r: HttpRequest, rr: web::Data<Rc<RefCell<RedditRoyalty>>>, stream: web::Payload, info: web::Query<WebsocketRequest>) -> Result<HttpResponse, Error> {
    if !rr.borrow().active_keys.contains_key(&*info.moderator) {
        return Ok(HttpResponse::Unauthorized().into());
    }
    let data = rr.as_ref().clone();
    let res = ws::start(MyWebSocket::new(data), &r, stream);
    res
}

#[get("/moderator")]
pub async fn moderator_index(pool: web::Data<DbPool>, rr: web::Data<Rc<RefCell<RedditRoyalty>>>,tera: web::Data<Tera>, r: HttpRequest) -> Result<HttpResponse, Error> {
    let mut ctx = tera::Context::new();
rr.borrow().add_key()

    let result = tera.get_ref().render("moderator.html", &ctx);
    if result.is_err() {
        let error = result.err().unwrap();
        return Err(HttpResponse::InternalServerError().into());
    }
    Ok(HttpResponse::Ok().content_type("text/html").body(&result.unwrap()))
}

/// websocket connection is long running connection, it easier

/// websocket connection is long running connection, it easier
/// to handle with an actor
struct MyWebSocket {
    /// Client must send ping at least once per 10 seconds (CLIENT_TIMEOUT),
    /// otherwise we drop connection.
    hb: Instant,
    conn: MysqlConnection,
    reddit_royalty: Rc<RefCell<RedditRoyalty>>
}

impl Actor for MyWebSocket {
    type Context = ws::WebsocketContext<Self>;

    /// Method is called on actor start. We start the heartbeat process here.
    fn started(&mut self, ctx: &mut Self::Context) {
        self.hb(ctx);
    }
}

/// Handler for `ws::Message`
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
                let value = serde_json::json!(text);
                let value1 = value["type"].as_str();
                if value1.unwrap().eq("approve") {
                    let value2 = value["user"].as_str();
                    if value2.is_some() {
                        approve_user(value2.unwrap(), value["moderator"].as_str().unwrap(), &self.conn);
                    }
                } else if value1.unwrap().eq("disapprove") {
                    //Deny a user and drop from database
                } else if value1.unwrap().eq("users") {
                    //TODO send 10 users
                }
            }
            Ok(ws::Message::Binary(bin)) => ctx.binary(bin),
            Ok(ws::Message::Close(reason)) => {
                ctx.close(reason);
                ctx.stop();
            }
            _ => ctx.stop(),
        }
    }
}

fn approve_user(user: &str, moderator: &str, conn: &MysqlConnection) {}

impl MyWebSocket {
    fn new(rr: Rc<RefCell<RedditRoyalty>>) -> Self {
        let string = std::env::var("DATABASE_URL").expect("DATABASE_URL");
        Self {
            hb: Instant::now(),
            conn: MysqlConnection::establish(&*string).unwrap(),
            reddit_royalty: rr
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
}
