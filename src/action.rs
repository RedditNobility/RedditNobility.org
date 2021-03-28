use diesel::MysqlConnection;
use diesel::prelude::*;

use crate::models;
use crate::models::{Moderator, Fuser};

pub fn add_new_fuser(fuser: &Fuser, conn: &MysqlConnection) -> Result<(), diesel::result::Error> {
    use crate::schema::fusers::dsl::*;

    diesel::insert_into(fusers).values(fuser).execute(conn).unwrap();

    Ok(())
}

pub fn get_fuser(fuser: String, conn: &MysqlConnection) -> Result<Option<models::Fuser>, diesel::result::Error> {
    use crate::schema::fusers::dsl::*;

    let found_mod = fusers.filter(username.eq(fuser)).first::<models::Fuser>(conn).optional()?;

    Ok(found_mod)
}
pub fn get_moderator(moderator: String, conn: &MysqlConnection) -> Result<Option<models::Moderator>, diesel::result::Error> {
    use crate::schema::moderators::dsl::*;

    let found_mod = moderators.filter(username.eq(moderator)).first::<models::Moderator>(conn).optional()?;

    Ok(found_mod)
}

pub fn get_moderators(conn: &MysqlConnection) -> Result<Vec<models::Moderator>, diesel::result::Error> {
    use crate::schema::moderators::dsl::*;

    let values = moderators.load::<models::Moderator>(conn).expect("Error loading mods");

    Ok(values)
}

pub fn add_moderator(moderator: &Moderator, conn: &MysqlConnection) -> Result<(), diesel::result::Error> {
    use crate::schema::moderators::dsl::*;

    diesel::insert_into(moderators).values(moderator).execute(conn).unwrap();

    Ok(())
}