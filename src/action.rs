use diesel::prelude::*;
use diesel::MysqlConnection;

use crate::models::{AuthToken, ClientKey, Setting, User};
use crate::{models, utils};

//User
pub fn add_new_user(user: &User, conn: &MysqlConnection) -> Result<(), diesel::result::Error> {
    use crate::schema::users::dsl::*;
    diesel::insert_into(users)
        .values(user)
        .execute(conn)
        .unwrap();
    Ok(())
}

pub fn get_user_by_name(
    user: String,
    conn: &MysqlConnection,
) -> Result<Option<models::User>, diesel::result::Error> {
    use crate::schema::users::dsl::*;
    let found_user = users
        .filter(username.eq(user))
        .first::<models::User>(conn)
        .optional()?;
    Ok(found_user)
}

pub fn get_user_by_id(
    l_id: i64,
    conn: &MysqlConnection,
) -> Result<Option<models::User>, diesel::result::Error> {
    use crate::schema::users::dsl::*;
    println!("{}", l_id);
    let found_user = users
        .filter(id.eq(l_id))
        .first::<models::User>(conn)
        .optional()?;
    Ok(found_user)
}

pub fn get_found_users(conn: &MysqlConnection) -> Result<Vec<models::User>, diesel::result::Error> {
    use crate::schema::users::dsl::*;
    let values = users
        .filter(status.eq("Found"))
        .load::<models::User>(conn)
        .expect("Error loading mods");

    Ok(values)
}
pub fn get_users(conn: &MysqlConnection) -> Result<Vec<models::User>, diesel::result::Error> {
    use crate::schema::users::dsl::*;

    let values = users.load::<models::User>(conn).expect("Error loading mods");

    Ok(values)
}

pub fn get_moderators(conn: &MysqlConnection) -> Result<Vec<models::User>, diesel::result::Error> {
    use crate::schema::users::dsl::*;
    let values = users
        .filter(level.eq_any(vec!["Moderator", "Admin"]))
        .load::<models::User>(conn)
        .expect("Error loading mods");

    Ok(values)
}

pub fn delete_user(us: String, conn: &MysqlConnection) -> Result<(), diesel::result::Error> {
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
            password.eq(user.password.clone()),
            status.eq(user.status.clone()),
            status_changed.eq(user.status_changed.clone()),
            level.eq(user.level.clone()),
            moderator.eq(user.moderator.clone()),
            properties.eq(user.properties.clone()),
            discoverer.eq(user.discoverer.clone()),
        ))
        .execute(conn)
        .unwrap();
    Ok(())
}

pub fn get_user_from_auth_token(
    token: String,
    conn: &MysqlConnection,
) -> Result<Option<models::User>, diesel::result::Error> {
    let result = get_auth_token(token, conn);
    if result.is_err() {
        return Err(result.err().unwrap());
    }
    let result = result.unwrap();
    if result.is_none() {
        return Ok(None);
    }
    let result = result.unwrap();
    return get_user_by_id(result.user, conn);
}

//Auth Token
pub fn get_auth_token(
    a_token: String,
    conn: &MysqlConnection,
) -> Result<Option<models::AuthToken>, diesel::result::Error> {
    use crate::schema::auth_tokens::dsl::*;
    let found_token = auth_tokens
        .filter(token.eq(a_token))
        .first::<models::AuthToken>(conn)
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

//API Key
pub fn get_client_key_by_id(
    key: i64,
    conn: &MysqlConnection,
) -> Result<Option<models::ClientKey>, diesel::result::Error> {
    use crate::schema::client_keys::dsl::*;
    let found_key = client_keys
        .filter(id.eq(key))
        .first::<models::ClientKey>(conn)
        .optional()?;
    Ok(found_key)
}

pub fn get_client_key_by_key(
    key: String,
    conn: &MysqlConnection,
) -> Result<Option<models::ClientKey>, diesel::result::Error> {
    use crate::schema::client_keys::dsl::*;
    let found_key = client_keys
        .filter(api_key.eq(key))
        .first::<models::ClientKey>(conn)
        .optional()?;
    Ok(found_key)
}

pub fn add_client_key(
    key: &ClientKey,
    conn: &MysqlConnection,
) -> Result<(), diesel::result::Error> {
    use crate::schema::client_keys::dsl::*;
    diesel::insert_into(client_keys)
        .values(key)
        .execute(conn)
        .unwrap();
    Ok(())
}
