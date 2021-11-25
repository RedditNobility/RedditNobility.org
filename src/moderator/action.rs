use diesel::dsl::count;
use crate::user::models::{Status};
use diesel::prelude::*;

use diesel::MysqlConnection;
use crate::schema::users::username;

/// Updates the User Status, Moderator Who Changed it, and the time it was changed
pub fn update_status(
    user: &i64,
    ns: Status,
    md: &String,
    time: i64,
    conn: &MysqlConnection,
) -> Result<(), diesel::result::Error> {
    use crate::schema::users::dsl::*;

    diesel::update(users.filter(id.eq(user)))
        .set((status.eq(&ns),
              status_changed.eq(&time),
              moderator.eq(&md)))
        .execute(conn)?;
    Ok(())
}

pub fn get_discover_count(
    user: &String,
    after: i64,
    conn: &MysqlConnection,
) -> Result<i64, diesel::result::Error> {
    use crate::schema::users::dsl::*;
    let value: i64 = users.select(count(discoverer)).filter(discoverer.eq(user).and(created.ge(after))).first(conn)?;

    Ok(value)
}pub fn get_approve_count(
    user:  &String,
    after: i64,
    conn: &MysqlConnection,
) -> Result<i64, diesel::result::Error> {
    use crate::schema::users::dsl::*;
    let value: i64 = users.select(count(moderator)).filter(moderator.eq(user).and(status_changed.ge(after))).first(conn)?;

    Ok(value)
}
pub fn get_discover_count_total(
    after: i64,
    conn: &MysqlConnection,
) -> Result<i64, diesel::result::Error> {
    use crate::schema::users::dsl::*;
    let value: i64 = users.select(count(discoverer)).filter(created.ge(after)).first(conn)?;

    Ok(value)
}pub fn get_approve_count_total(
    after: i64,
    conn: &MysqlConnection,
) -> Result<i64, diesel::result::Error> {
    use crate::schema::users::dsl::*;
    let value: i64 = users.select(count(moderator)).filter(status.not_like(Status::Found).and(status_changed.ge(after))).first(conn)?;

    Ok(value)
}