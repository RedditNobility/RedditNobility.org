use diesel::MysqlConnection;
use diesel::prelude::*;
use crate::user::models::{AuthToken, OTP, User};
use diesel::result::Error as DieselError;
pub fn add_new_user(user: &User, conn: &MysqlConnection) -> Result<(), diesel::result::Error> {
    use crate::schema::users::dsl::*;
    diesel::insert_into(users)
        .values(user)
        .execute(conn)
        .unwrap();
    Ok(())
}

pub fn get_user_by_name(
    user: &String,
    conn: &MysqlConnection,
) -> Result<Option<User>, diesel::result::Error> {
    use crate::schema::users::dsl::*;
    return users
        .filter(username.eq(user))
        .first::<User>(conn)
        .optional();
}

pub fn get_user_by_id(
    l_id: &i64,
    conn: &MysqlConnection,
) -> Result<Option<User>, diesel::result::Error> {
    use crate::schema::users::dsl::*;
    return users
        .filter(id.eq(l_id))
        .first::<User>(conn)
        .optional();
}

pub fn get_found_users(conn: &MysqlConnection) -> Result<Vec<User>, diesel::result::Error> {
    use crate::schema::users::dsl::*;
    return users
        .filter(status.eq("Found"))
        .load::<User>(conn);
}

pub fn get_users(conn: &MysqlConnection) -> Result<Vec<User>, diesel::result::Error> {
    use crate::schema::users::dsl::*;

    return users.load::<User>(conn);
}

pub fn get_moderators(conn: &MysqlConnection) -> Result<Vec<User>, diesel::result::Error> {
    use crate::schema::users::dsl::*;
    return users
        .filter(level.eq_any(vec!["Moderator", "Admin"]))
        .load::<User>(conn);
}

pub fn delete_user(us: &String, conn: &MysqlConnection) -> Result<(), diesel::result::Error> {
    use crate::schema::users::dsl::*;

    diesel::delete(users.filter(username.eq(us)))
        .execute(conn)
        .unwrap();
    Ok(())
}

pub fn update_user(user: &User, conn: &MysqlConnection) -> Result<(), diesel::result::Error> {
    use crate::schema::users::dsl::*;

    diesel::update(users.filter(id.eq(user.id)))
        .set((
            password.eq(&user.password),
            status.eq(&user.status),
            status_changed.eq(&user.status_changed),
            level.eq(&user.level),
            moderator.eq(&user.moderator),
            properties.eq(&user.properties),
            discoverer.eq(&user.discoverer),
        ))
        .execute(conn)
        .unwrap();
    Ok(())
}

pub fn get_user_from_auth_token(
    token: String,
    conn: &MysqlConnection,
) -> Result<Option<User>, diesel::result::Error> {
    let result = get_auth_token(token, conn);
    if result.is_err() {
        return Err(result.err().unwrap());
    }
    let result = result.unwrap();
    if result.is_none() {
        return Ok(None);
    }
    let result = result.unwrap();
    return get_user_by_id(&result.user, conn);
}

//Auth Token
pub fn get_auth_token(
    a_token: String,
    conn: &MysqlConnection,
) -> Result<Option<AuthToken>, diesel::result::Error> {
    use crate::schema::auth_tokens::dsl::*;
    let found_token = auth_tokens
        .filter(token.eq(a_token))
        .first::<AuthToken>(conn)
        .optional()?;
    Ok(found_token)
}

pub fn add_new_auth_token(
    t: &AuthToken,
    conn: &MysqlConnection,
) -> Result<(), diesel::result::Error> {
    use crate::schema::auth_tokens::dsl::*;
    diesel::insert_into(auth_tokens)
        .values(t)
        .execute(conn)
        .unwrap();
    Ok(())
}


pub fn get_opt(value: &String, conn: &MysqlConnection) -> Result<Option<OTP>, diesel::result::Error> {
    use crate::schema::otps::dsl::*;
    let x: Option<OTP> = otps.filter(password.eq(value)).first::<OTP>(conn).optional()?;
    return Ok(x);
}
pub fn delete_otp(id: i64, conn: &MysqlConnection)->Result<(), DieselError>{
    use crate::schema::otps::dsl::*;
    diesel::delete(otps).filter(id.eq(id)).execute(conn)?;
    return Ok(())
}
pub fn opt_exist(value: &String, conn: &MysqlConnection) -> Result<bool, diesel::result::Error> {
    use crate::schema::otps::dsl::*;
    let x: Option<i64> = otps.select(id).filter(password.eq(value)).first(conn).optional()?;
    return Ok(x.is_some());
}
pub fn add_opt(value: &OTP, conn: &MysqlConnection)->Result<(), DieselError>{
    use crate::schema::otps::dsl::*;

    diesel::insert_into(otps).values(value).execute(conn)?;
    return Ok(());

}