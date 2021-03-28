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

#[derive(Debug, PartialEq)]
pub enum Status {
    Found,
    Approved,
    Denied,
    Banned,
}

#[derive(Debug, Clone, Serialize, Deserialize, Queryable, Insertable)]
pub struct Fuser {
    pub id: i64,
    pub username: String,
    pub status: String,
    pub moderator: String,
}

impl FromStr for Status {
    type Err = ();

    fn from_str(input: &str) -> Result<Status, Self::Err> {
        match input {
            "Found" => Ok(Status::Found),
            "Approved" => Ok(Status::Approved),
            "Denied" => Ok(Status::Denied),
            "Banned" => Ok(Status::Banned),
            _ => Err(())
        }
    }
}

impl Fuser {
    pub fn get_status(&self) -> Status {
        Status::from_str(&*self.status).unwrap()
    }
}