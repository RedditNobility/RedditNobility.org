use std::fmt::{Display, Error, Formatter};
use std::str::FromStr;

use diesel::Queryable;
use serde::{Deserialize, Serialize};

use crate::schema::*;

#[derive(Debug, Clone, Serialize, Deserialize, Queryable, Insertable)]
pub struct AuthToken {
    pub id: i64,
    pub user: i64,
    pub token: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Queryable, Insertable)]
pub struct APIKey {
    pub id: i64,
    pub api_key: String,
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
    pub status_changed: u64,
    //Who found the user BOT if bot
    pub discoverer: String,
    //The Moderator who approved them or denied them. If the user was banned it will still be set to who approved them
    pub moderator: String,
    //When the data was created
    pub created: u64,
}
//Found, Approved, Denied, Banned


impl User {
    pub fn set_status(&mut self, status: String) {
        self.status = status;
    }

    pub fn set_moderator(&mut self, moderator: String) {
        self.moderator = moderator;
    }
}

impl Display for User {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}