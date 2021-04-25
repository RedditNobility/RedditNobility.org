use std::fmt::{Display, Error, Formatter};
use std::str::FromStr;

use diesel::Queryable;
use serde::{Deserialize, Serialize};
use strum_macros::EnumString;
use strum_macros::Display;
use crate::schema::*;

#[derive(Debug, Clone, Serialize, Deserialize, Queryable, Insertable)]
pub struct AuthToken {
    pub id: i64,
    pub user: i64,
    pub token: String,
    pub created: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize, Queryable, Insertable)]
pub struct ClientKey {
    pub id: i64,
    pub api_key: String,
    pub created: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize, Queryable, Insertable)]
pub struct User {
    pub id: i64,
    //The Reddit Username
    pub username: String,
    // The users password. If they are just going to use the Reddit login feature. This will be changed to the latest login token
    pub password: String,
    //USER, MODERATOR, ADMIN
    pub level: String,
    //FOUND, DENIED, APPROVED, BANNED
    pub status: String,
    //When was their status changed from FOUND to DENIED or APPROVED
    pub status_changed: i64,
    //Who found the user BOT if bot
    pub discoverer: String,
    //The Moderator who approved them or denied them. If the user was banned it will still be set to who approved them
    pub moderator: String,
    //When the data was created
    pub created: i64,
}

#[derive(Debug, Display, PartialEq, EnumString)]
pub enum Status {
    FOUND,
    DENIED,
    APPROVED,
    BANNED,
}

//Found, Approved, Denied, Banned
#[derive(Debug, PartialEq, EnumString)]
pub enum Level {
    Admin,
    Moderator,
    User,
    Client,
}


impl Level {
    pub fn name(&self) -> &str {
        match *self {
            Level::Admin => "ADMIN",
            Level::Moderator => "MODERATOR",
            Level::User => "USER",
            Level::Client => "CLIENT",
        }
    }
    pub fn level(&self) -> i32 {
        match *self {
            Level::Admin => 3,
            Level::Moderator => 2,
            Level::User => 1,
            Level::Client => 4,
        }
    }
}

impl User {
    pub fn set_status(&mut self, status: String) {
        self.status = status;
    }

    pub fn set_moderator(&mut self, moderator: String) {
        self.moderator = moderator;
    }
    pub fn set_password(&mut self, password: String) {
        self.password = password;
    }
}

impl Display for User {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}