use diesel::MysqlConnection;
use diesel::prelude::*;

use crate::models;
use crate::models::{User, AuthToken};

//User
pub fn add_new_user(user: &User, conn: &MysqlConnection) -> Result<(), diesel::result::Error> {
    use crate::schema::users::dsl::*;
    diesel::insert_into(users).values(user).execute(conn).unwrap();
    Ok(())
}

pub fn get_user_by_name(user: String, conn: &MysqlConnection) -> Result<Option<models::User>, diesel::result::Error> {
    use crate::schema::users::dsl::*;
    let found_user = users.filter(username.eq(user)).first::<models::User>(conn).optional()?;
    Ok(found_user)
}

pub fn get_user_by_id(l_id: i64, conn: &MysqlConnection) -> Result<Option<models::User>, diesel::result::Error> {
    use crate::schema::users::dsl::*;
    let found_user = users.filter(id.eq(l_id)).first::<models::User>(conn).optional()?;
    Ok(found_user)
}

pub fn update_user(user: &User, conn: &MysqlConnection) -> Result<(), diesel::result::Error> {
    use crate::schema::users::dsl::*;

    diesel::update(users.filter(id.eq(user.id)))
        .set(
            (password.eq(user.password.clone()),
             status.eq(user.status.clone()),
             status_changed.eq(user.status_changed.clone()),
             level.eq(user.level.clone()),
             discoverer.eq(user.discoverer.clone())))
        .execute(conn).unwrap();
    Ok(())
}

pub fn get_user_from_auth_token(token: String, conn: &MysqlConnection) -> Result<Option<models::User>, diesel::result::Error> {
    let result = get_auth_token(token, conn);
    if result.is_err() {
        return Err(result.err().unwrap());
    }
    return get_user_by_id(result.unwrap().unwrap().id, conn);
}

//Auth Token
pub fn get_auth_token(a_token: String, conn: &MysqlConnection) -> Result<Option<models::AuthToken>, diesel::result::Error> {
    use crate::schema::auth_tokens::dsl::*;
    let found_token = auth_tokens.filter(token.eq(a_token)).first::<models::AuthToken>(conn).optional()?;
    Ok(found_token)
}

pub fn add_new_auth_token(t: &AuthToken, conn: &MysqlConnection) -> Result<(), diesel::result::Error> {
    use crate::schema::auth_tokens::dsl::*;
    diesel::insert_into(auth_tokens).values(t).execute(conn).unwrap();
    Ok(())
}
//API Key

pub fn get_client_key(key: String, conn: &MysqlConnection) -> Result<Option<models::AuthToken>, diesel::result::Error> {
    use crate::schema::client_keys::dsl::*;
    let found_key = client_keys.filter(api_key.eq(key)).first::<models::ClientKey>(conn).optional()?;
    Ok(found_key)
}
