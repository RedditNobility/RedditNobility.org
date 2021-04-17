use crate::action;
use diesel::MysqlConnection;
use crate::models::User;
use std::time::{SystemTime, UNIX_EPOCH};

pub fn quick_add(username: String, conn: &MysqlConnection) {
    let mut status = "Found";
    if username.contains("=T") {
        status = "Approved";
    }else if username.contains("=F"){
        status = "Denied";
    }
    let username = username.replace("=T", "").replace("=F", "").replace("\r","");
    if action::get_fuser(username.clone(), &conn).unwrap().is_none() {
        let fuser = User {
            id: 0,
            username: username.clone(),
            moderator: "".to_string(),
            status: status.to_string(),
            created: SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_millis() as i64,
        };
        println!("Adding {}", &fuser);
        action::add_new_fuser(&fuser, &conn);
    }
}