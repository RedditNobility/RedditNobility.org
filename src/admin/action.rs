use crate::user::models::{AuthToken, User, UserProperties, OTP, Status, Level};
use diesel::prelude::*;
use diesel::result::Error as DieselError;
use diesel::MysqlConnection;

pub fn set_level(
    user: &i64,
    level: Level,
    conn: &MysqlConnection,
) -> Result<(), diesel::result::Error> {
    use crate::schema::users::dsl::*;

    diesel::update(users.filter(id.eq(user)))
        .set((level.eq(&level)))
        .execute(conn)?;
    Ok(())
}