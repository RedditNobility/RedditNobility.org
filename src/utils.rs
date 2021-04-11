use crate::action;
use diesel::MysqlConnection;
use crate::models::Fuser;
use std::time::{SystemTime, UNIX_EPOCH};

pub fn quick_add(username: String, conn: &MysqlConnection) {
    println!("Adding user {}", username);
    if action::get_fuser(username.clone(), &conn).unwrap().is_none() {
        let fuser = Fuser {
            id: 0,
            username: username.clone(),
            moderator: "".to_string(),
            status: "Found".to_string(),
            created: SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_millis() as i64,
        };
        action::add_new_fuser(&fuser, &conn);
    }
}