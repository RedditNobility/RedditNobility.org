use crate::schema::*;
use crate::utils;
use diesel::backend::Backend;
use diesel::deserialize::FromSql;
use diesel::mysql::Mysql;
use diesel::serialize::{Output, ToSql};
use diesel::sql_types::Text;
use diesel::{deserialize, serialize, Queryable};
use serde::{Deserialize, Serialize};

use crate::utils::is_valid;
use std::fmt::{Display, Error, Formatter};
use std::io::Write;
use std::str::FromStr;
use strum_macros::Display;
use strum_macros::EnumString;

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

#[derive(AsExpression, Debug, Deserialize, Serialize, FromSqlRow, Clone)]
#[sql_type = "Text"]
pub struct UserProperties {
    pub avatar: Option<String>,
    pub description: Option<String>,
    pub title: Option<String>,
}

impl UserProperties {
    pub fn set_avatar(&mut self, avatar: String) {
        self.avatar = Some(avatar);
    }
    pub fn set_description(&mut self, description: String) {
        self.description = Some(description);
    }
}

impl FromSql<Text, Mysql> for UserProperties {
    fn from_sql(
        bytes: Option<&<diesel::mysql::Mysql as Backend>::RawValue>,
    ) -> deserialize::Result<UserProperties> {
        let t = <String as FromSql<Text, Mysql>>::from_sql(bytes);
        if t.is_err() {
            //IDK break
        }
        let string = t.unwrap();
        let result: Result<UserProperties, serde_json::Error> =
            serde_json::from_str(string.as_str());
        if result.is_err() {
            //IDK break
        }
        return Ok(result.unwrap());
    }
}

impl ToSql<Text, Mysql> for UserProperties {
    fn to_sql<W: Write>(&self, out: &mut Output<W, Mysql>) -> serialize::Result {
        let s = serde_json::to_string(&self)?;
        <String as ToSql<Text, Mysql>>::to_sql(&s, out)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Queryable, Insertable)]
pub struct User {
    pub id: i64,
    //The Reddit Username
    pub username: String,
    // The users password. If they are just going to use the Reddit login feature. This will be changed to the latest login token
    #[serde(skip_serializing)]
    pub password: String,
    //USER, MODERATOR, ADMIN
    pub level: Level,
    //FOUND, DENIED, APPROVED, BANNED
    pub status: Status,
    //When was their status changed from FOUND to DENIED or APPROVED
    pub status_changed: i64,
    //Who found the user BOT if bot
    pub discoverer: String,
    //The Moderator who approved them or denied them. If the user was banned it will still be set to who approved them
    pub moderator: String,
    // Custom Properties done through json.
    pub properties: UserProperties,
    //When the data was created
    pub created: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SubmitUser {
    pub username: String,
    //FOUND, DENIED, APPROVED, BANNED
    pub status: Option<Status>,
    //The Moderator who approved them or denied them. If the user was banned it will still be set to who approved them
    pub moderator: Option<String>,
    //When the data was created
    pub created: Option<i64>,
}

#[derive(
    AsExpression, Debug, Deserialize, Serialize, FromSqlRow, Clone, Display, PartialEq, EnumString,
)]
#[sql_type = "Text"]
pub enum Status {
    Found,
    Denied,
    Approved,
    Banned,
}

//Found, Approved, Denied, Banned
#[derive(
    AsExpression, Debug, Deserialize, Serialize, FromSqlRow, Clone, Display, PartialEq, EnumString,
)]
#[sql_type = "Text"]
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

impl ToSql<Text, Mysql> for Level {
    fn to_sql<W: Write>(&self, out: &mut Output<W, Mysql>) -> serialize::Result {
        let s = self.to_string();
        <String as ToSql<Text, Mysql>>::to_sql(&s, out)
    }
}

impl FromSql<Text, Mysql> for Level {
    fn from_sql(
        bytes: Option<&<diesel::mysql::Mysql as Backend>::RawValue>,
    ) -> deserialize::Result<Level> {
        let t = <String as FromSql<Text, Mysql>>::from_sql(bytes);
        if t.is_err() {
            //IDK break
        }
        let string = t.unwrap();
        let result: Result<Level, strum::ParseError> = Level::from_str(string.as_str());
        if result.is_err() {
            //IDK break
        }
        return Ok(result.unwrap());
    }
}

impl ToSql<Text, Mysql> for Status {
    fn to_sql<W: Write>(&self, out: &mut Output<W, Mysql>) -> serialize::Result {
        let s = self.to_string();
        <String as ToSql<Text, Mysql>>::to_sql(&s, out)
    }
}

impl FromSql<Text, Mysql> for Status {
    fn from_sql(
        bytes: Option<&<diesel::mysql::Mysql as Backend>::RawValue>,
    ) -> deserialize::Result<Status> {
        let t = <String as FromSql<Text, Mysql>>::from_sql(bytes);
        if t.is_err() {
            //IDK break
        }
        let string = t.unwrap();
        let result: Result<Status, strum::ParseError> = Status::from_str(string.as_str());
        if result.is_err() {
            //IDK break
        }
        return Ok(result.unwrap());
    }
}

impl User {
    pub fn new(sub: SubmitUser, discoverer: String) -> User {
        let properties = UserProperties {
            avatar: None,
            description: None,
            title: is_valid(sub.username.clone()),
        };
        User {
            id: 0,
            username: sub.username.clone(),
            password: "".to_string(),
            level: Level::User,
            status: sub.status.unwrap_or_else(default_status),
            status_changed: utils::get_current_time(),
            discoverer,
            moderator: sub.moderator.unwrap_or_else(default_moderator),
            properties,
            created: sub.created.unwrap_or_else(utils::get_current_time),
        }
    }
    pub fn set_status(&mut self, status: Status) {
        self.status = status;
        self.status_changed = utils::get_current_time();
    }

    pub fn set_level(&mut self, level: Level) {
        self.level = level;
    }
    pub fn set_moderator(&mut self, moderator: String) {
        self.moderator = moderator;
    }
    pub fn set_password(&mut self, password: String) {
        self.password = password;
    }
}

fn default_status() -> Status {
    Status::Found
}
fn default_moderator() -> String {
    "".to_string()
}

impl Display for User {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}

impl Display for UserProperties {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
