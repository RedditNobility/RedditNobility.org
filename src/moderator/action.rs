use crate::user::models::{AuthToken, User, UserProperties, OTP, Status};
use diesel::prelude::*;
use diesel::result::Error as DieselError;
use diesel::MysqlConnection;

/// Updates the User Status, Moderator Who Changed it, and the time it was changed
pub fn update_status(
    user: &i64,
    status: Status,
    moderator: &String,
    time: i64,
    conn: &MysqlConnection,
) -> Result<(), diesel::result::Error> {
    use crate::schema::users::dsl::*;

    diesel::update(users.filter(id.eq(user)))
        .set((status.eq(&status),
              status_changed.eq(&time),
              moderator.eq(&moderator)))
        .execute(conn)?;
    Ok(())
}
