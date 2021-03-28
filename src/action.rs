use diesel::MysqlConnection;
use diesel::prelude::*;

use crate::models;
use crate::models::{Moderator, Fuser, Member};

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