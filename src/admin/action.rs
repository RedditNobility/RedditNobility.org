use crate::user::models::{AuthToken, User, UserProperties, OTP, Status, UserPermissions};
use diesel::prelude::*;
use diesel::result::Error as DieselError;
use diesel::MysqlConnection;

pub fn set_permissions(
    user: &i64,
    perms: UserPermissions,
    conn: &MysqlConnection,
) -> Result<(), diesel::result::Error> {
    use crate::schema::users::dsl::*;

    diesel::update(users.filter(id.eq(user)))
        .set((permissions.eq(&perms)))
        .execute(conn)?;
    Ok(())
}