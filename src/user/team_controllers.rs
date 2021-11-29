use std::collections::HashMap;
use actix_web::{web, HttpRequest};

use crate::api_response::{APIResponse, SiteResponse};
use crate::{Database};
use crate::error::response::{bad_request, not_found, unauthorized};
use crate::user::action::{get_team_members, get_team_user, get_user_by_id};
use crate::user::utils::get_user_by_header;
use actix_web::get;
use crate::user::models::{Level, TeamResponse};
use crate::utils::get_avatar;


#[get("/team/get")]
pub async fn get_team(
    database: Database,
    req: HttpRequest,
) -> SiteResponse {
    let connection = database.get()?;
    let mut response = HashMap::<Level, Vec<TeamResponse>>::new();
    let vec = get_team_members(&connection)?;
    for x in vec {
        if response.get(&x.level).is_none() {
            response.insert(x.level.clone(), Vec::new());
        }
        let mut team = response.get_mut(&x.level).unwrap();
        let mut user = get_team_user(&x.user, &connection)?.unwrap();
        let avatar = get_avatar(&user.username, &user.properties).await?;
        user.properties.avatar = Some(avatar);
        team.push(TeamResponse {
            user: user,
            description: x.description,
            level: x.level,
            created: x.created,
        });
    }
    return APIResponse::respond_new(Some(response), &req);
}

#[get("/team/get/list")]
pub async fn get_team_as_list(
    database: Database,
    req: HttpRequest,
) -> SiteResponse {
    let connection = database.get()?;
    let vec: Vec<TeamResponse> = get_team_members(&connection)?.iter().map(|x| {
        let user = get_team_user(&x.user, &connection).unwrap().unwrap();
        //TODO send avatar data correctly
        return TeamResponse {
            user: user,
            description: x.description.clone(),
            level: x.level.clone(),
            created: x.created.clone(),
        };
    }).collect();
    return APIResponse::respond_new(Some(vec), &req);
}