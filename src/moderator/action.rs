use crate::user::models::Status;
use diesel::dsl::count;
use diesel::prelude::*;

use diesel::MysqlConnection;

/// Updates the User Status, Moderator Who Changed it, and the time it was changed
pub fn update_status(
    user: &i64,
    ns: Status,
    md: &str,
    time: i64,
    conn: &MysqlConnection,
) -> Result<(), diesel::result::Error> {
    use crate::schema::users::dsl::*;

    diesel::update(users.filter(id.eq(user)))
        .set((status.eq(&ns), status_changed.eq(&time), reviewer.eq(&md)))
        .execute(conn)?;
    Ok(())
}

pub fn get_discover_count(
    user: &str,
    after: i64,
    conn: &MysqlConnection,
) -> Result<i64, diesel::result::Error> {
    use crate::schema::users::dsl::*;
    let value: i64 = users
        .select(count(discoverer))
        .filter(discoverer.eq(user).and(created.ge(after)))
        .first(conn)?;

    Ok(value)
}
pub fn get_approve_count(
    user: &str,
    after: i64,
    conn: &MysqlConnection,
) -> Result<i64, diesel::result::Error> {
    use crate::schema::users::dsl::*;
    let value: i64 = users
        .select(count(reviewer))
        .filter(reviewer.eq(user).and(status_changed.ge(after)))
        .first(conn)?;

    Ok(value)
}
pub fn get_discover_count_total(
    after: i64,
    conn: &MysqlConnection,
) -> Result<i64, diesel::result::Error> {
    use crate::schema::users::dsl::*;
    let value: i64 = users
        .select(count(discoverer))
        .filter(created.ge(after))
        .first(conn)?;

    Ok(value)
}
pub fn get_approve_count_total(
    after: i64,
    conn: &MysqlConnection,
) -> Result<i64, diesel::result::Error> {
    use crate::schema::users::dsl::*;
    let value: i64 = users
        .select(count(reviewer))
        .filter(status.not_like(Status::Found).and(status_changed.ge(after)))
        .first(conn)?;

    Ok(value)
}
