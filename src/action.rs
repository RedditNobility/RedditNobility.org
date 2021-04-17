use diesel::MysqlConnection;
use diesel::prelude::*;

use crate::models;
use crate::models::{Moderator, User};

pub fn add_new_fuser(fuser: &User, conn: &MysqlConnection) -> Result<(), diesel::result::Error> {
    use crate::schema::users::dsl::*;

    diesel::insert_into(users).values(fuser).execute(conn).unwrap();

    Ok(())
}

pub fn get_fuser(fuser: String, conn: &MysqlConnection) -> Result<Option<models::User>, diesel::result::Error> {
    use crate::schema::users::dsl::*;

    let found_mod = users.filter(username.eq(fuser)).first::<models::User>(conn).optional()?;

    Ok(found_mod)
}

pub fn update_fuser(s: String, md: String, name: String, conn: &MysqlConnection) -> Result<(), diesel::result::Error> {
    use crate::schema::users::dsl::*;

    diesel::update(users.filter(username.eq(name)))
        .set((moderator.eq(md), status.eq(s)))
        .execute(conn);
    Ok(())
}

pub fn get_found_fusers(conn: &MysqlConnection) -> Result<Vec<models::User>, diesel::result::Error> {
    use crate::schema::users::dsl::*;

    let values = users.filter(status.eq("Found")).load::<models::User>(conn).expect("Error loading mods");

    Ok(values)
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