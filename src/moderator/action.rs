use crate::user::models::{Status};
use diesel::prelude::*;

use diesel::MysqlConnection;

/// Updates the User Status, Moderator Who Changed it, and the time it was changed
pub fn update_status(
    user: &i64,
    _status: Status,
    _moderator: &String,
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
