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
#[table_name = "otps"]
pub struct OTP {
    pub id: i64,
    pub user: i64,
    pub password: String,
    pub expiration: i64,
    pub created: i64,
}

#[derive(AsExpression, Debug, Deserialize, Serialize, FromSqlRow, Clone)]
#[sql_type = "Text"]
pub struct UserProperties {
    pub avatar: Option<String>,
    pub description: Option<String>,
    pub title: Option<String>,
}

#[derive(AsExpression, Debug, Deserialize, Serialize, FromSqlRow, Clone)]
#[sql_type = "Text"]
pub struct UserPermissions {
    #[serde(default)]
    pub admin: bool,
    #[serde(default)]
    pub moderator: bool,
    #[serde(default)]
    pub submit: bool,
    #[serde(default)]
    pub review_user: bool,
    #[serde(default)]
    pub login: bool,
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
    pub discord_id: i64,
    //The Reddit Username
    pub username: String,
    // The users password. If they are just going to use the Reddit login feature. This will be changed to the latest login token
    #[serde(skip_serializing)]
    pub password: String,
    //USER, MODERATOR, ADMIN
    pub permissions: UserPermissions,
    //FOUND, DENIED, APPROVED, BANNED
    pub status: Status,
    //When was their status changed from FOUND to DENIED or APPROVED
    pub status_changed: i64,
    //Who found the user BOT if bot
    pub discoverer: String,
    //The Moderator who approved them or denied them. If the user was banned it will still be set to who approved them
    pub reviewer: String,
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

#[derive(Debug, Clone, Serialize, Deserialize,Queryable)]
pub struct TeamUser {
    pub username: String,
    pub properties: UserProperties,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TeamResponse {
    pub user: TeamUser,
    pub description: String,
    pub level: Level,
    pub created: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize, Queryable, Insertable)]
#[table_name = "team_members"]
pub struct TeamMember {
    pub id: i64,
    pub user: i64,
    pub description: String,
    pub level: Level,
    pub created: i64,
}

#[derive(
AsExpression, Debug, Deserialize, Serialize, FromSqlRow, Clone, Display, PartialEq, EnumString, Hash, Eq
)]
#[sql_type = "Text"]
pub enum Level {
    Moderator,
    Recruiter,
    Retired,
}


#[derive(
AsExpression, Debug, Deserialize, Serialize, FromSqlRow, Clone, Display, PartialEq, EnumString,
)]
#[sql_type = "Text"]
pub enum Status {
    Found,
    Denied,
    Approved,
}

impl ToSql<Text, Mysql> for UserPermissions {
    fn to_sql<W: Write>(&self, out: &mut Output<W, Mysql>) -> serialize::Result {
        let s = serde_json::to_string(self)?;
        <String as ToSql<Text, Mysql>>::to_sql(&s, out)
    }
}

impl FromSql<Text, Mysql> for UserPermissions {
    fn from_sql(
        bytes: Option<&<diesel::mysql::Mysql as Backend>::RawValue>,
    ) -> deserialize::Result<UserPermissions> {
        let t = <String as FromSql<Text, Mysql>>::from_sql(bytes)?;
        let result = serde_json::from_str(&t)?;
        return Ok(result);
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

impl User {
    pub fn new(sub: SubmitUser, discoverer: String) -> User {
        let properties = UserProperties {
            avatar: None,
            description: None,
            title: is_valid(&sub.username),
        };
        User {
            id: 0,
            discord_id: 0,
            username: sub.username.clone(),
            password: "".to_string(),
            status: sub.status.unwrap_or_else(default_status),
            status_changed: utils::get_current_time(),
            discoverer,
            reviewer: sub.moderator.unwrap_or_else(default_moderator),
            properties,
            created: sub.created.unwrap_or_else(utils::get_current_time),
            permissions: UserPermissions {
                admin: false,
                moderator: false,
                submit: true,
                review_user: false,
                login: true,
            },
        }
    }
    pub fn set_status(&mut self, status: Status) {
        self.status = status;
        self.status_changed = utils::get_current_time();
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
