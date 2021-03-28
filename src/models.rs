use serde::{Serialize, Deserialize};
use diesel::{Queryable};
use crate::schema::*;
use std::str::FromStr;

#[derive(Debug, Clone, Serialize, Deserialize, Queryable, Insertable)]
pub struct Moderator {
    pub id: i64,
    pub username: String,
    pub password: String,
    pub admin: bool,
}


#[derive(Debug, Clone, Serialize, Deserialize, Queryable, Insertable)]
pub struct Fuser {
    pub id: i64,
    pub username: String,
    pub status: String,
    pub moderator: String,
    pub created: i64,
}
//Found, Approved, Denied, Banned


impl Fuser {
    pub fn set_status(&mut self, status: String) {
        self.status = status;
    }

    pub fn set_moderator(&mut self, moderator: String) {
        self.moderator = moderator;
    }
}