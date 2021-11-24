use crate::error::internal_error::InternalError;

use crate::settings::action::{add_new_setting, get_setting, get_settings, update_setting};
use crate::settings::settings::{
    DBSetting, EmailSetting, GeneralSettings, SecuritySettings, SettingManager, SettingReport,
    SettingVec,
};
use crate::utils::get_current_time;
use diesel::MysqlConnection;

pub fn quick_add(key: &str, value: String, conn: &MysqlConnection) -> Result<(), InternalError> {
    let result = get_setting(key, conn)?;
    if let Some(mut setting) = result {
        setting.set_value(value.clone());
        update_setting(&setting, conn)?;
    }
    let setting = DBSetting {
        id: 0,
        setting: key.into(),
        value,
        updated: get_current_time(),
    };
    add_new_setting(&setting, conn)?;
    Ok(())
}
pub fn get_setting_or_empty(
    string: &str,
    connection: &MysqlConnection,
) -> Result<DBSetting, InternalError> {
    let result = get_setting(string.clone(), connection)?;
    if let Some(some) = result {
        Ok(some)
    } else {
        default_setting(string)
    }
}

pub fn default_string() -> String {
    "".to_string()
}

pub fn default_setting(string: &str) -> Result<DBSetting, InternalError> {
    let setting = SettingManager::get_setting(string.to_string())
        .ok_or(InternalError::Error("Unable to find setting".to_string()))
        .unwrap();
    Ok(DBSetting {
        id: 0,
        setting: setting.clone(),
        value: setting.default.unwrap_or_else(default_string),
        updated: get_current_time(),
    })
}
pub fn get_setting_report(connection: &MysqlConnection) -> Result<SettingReport, InternalError> {
    let vec = get_settings(connection)?;
    let email = EmailSetting {
        email_username: vec
            .get_setting_by_key("email.username")
            .unwrap_or(&default_setting("email.username")?)
            .clone(),
        email_password: vec
            .get_setting_by_key("email.password")
            .unwrap_or(&default_setting("email.password")?)
            .clone(),
        email_host: vec
            .get_setting_by_key("email.host")
            .unwrap_or(&default_setting("email.host")?)
            .clone(),
        encryption: vec
            .get_setting_by_key("email.encryption")
            .unwrap_or(&default_setting("email.encryption")?)
            .clone(),
        from: vec
            .get_setting_by_key("email.from")
            .unwrap_or(&default_setting("email.from")?)
            .clone(),
        port: vec
            .get_setting_by_key("email.port")
            .unwrap_or(&default_setting("email.port")?)
            .clone(),
    };
    let general = GeneralSettings {
        name: vec
            .get_setting_by_key("name.public")
            .unwrap_or(&default_setting("name.public")?)
            .clone(),
        installed: vec
            .get_setting_by_key("installed")
            .unwrap_or(&default_setting("installed")?)
            .clone(),
        version: vec
            .get_setting_by_key("version")
            .unwrap_or(&default_setting("version")?)
            .clone(),
    };
    let security = SecuritySettings {};
    Ok(SettingReport {
        email,
        general,
        security,
    })
}
