use serde::{Serialize, Deserialize};
use diesel::{Queryable};
use crate::schema::*;
#[derive(Debug, Clone, Serialize, Deserialize, Queryable, Insertable)]
pub struct Moderator {
    pub id: i64,
    pub username: String,
    pub password: String,
    pub admin: bool


}
#[derive(Debug, Clone, Serialize, Deserialize, Queryable, Insertable)]
pub struct Member {
    pub id: i64,
    pub username: String,
    pub moderator: String,
    pub created_on: i64,



}
#[derive(Debug, Clone, Serialize, Deserialize, Queryable, Insertable)]
pub struct Fuser {
    pub id: i64,
    pub username: String,
}
