use crate::user::models::{TeamMember, UserPermissions};
use diesel::prelude::*;
use diesel::result::Error as DieselError;
use diesel::MysqlConnection;

pub fn set_permissions(
    user: &i64,
    perms: UserPermissions,
    conn: &MysqlConnection,
) -> Result<(), DieselError> {
    use crate::schema::users::dsl::*;

    diesel::update(users.filter(id.eq(user)))
        .set(permissions.eq(&perms))
        .execute(conn)?;
    Ok(())
}

pub fn delete_team(i: &i64, conn: &MysqlConnection) -> Result<(), DieselError> {
    use crate::schema::team_members::dsl::*;

    diesel::delete(team_members)
        .filter(id.eq(i))
        .execute(conn)?;
    Ok(())
}
pub fn delete_team_user(i: &i64, conn: &MysqlConnection) -> Result<(), DieselError> {
    use crate::schema::team_members::dsl::*;

    diesel::delete(team_members)
        .filter(user.eq(i))
        .execute(conn)?;
    Ok(())
}

pub fn add_new_team_member(
    value: &TeamMember,
    conn: &MysqlConnection,
) -> Result<(), diesel::result::Error> {
    use crate::schema::team_members::dsl::*;
    diesel::insert_into(team_members)
        .values(value)
        .execute(conn)
        .unwrap();
    Ok(())
}
