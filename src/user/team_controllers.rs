use std::collections::HashMap;
use actix_web::{web, HttpRequest};

use crate::api_response::{APIResponse, SiteResponse};
use crate::{Database};
use crate::error::response::{bad_request, not_found, unauthorized};
use crate::user::action::{get_team_members, get_team_user, get_user_by_id};
use crate::user::utils::get_user_by_header;
use actix_web::get;
use crate::user::models::{Level, TeamResponse};


#[get("/team/get")]
pub async fn get_team(
    database: Database,
    req: HttpRequest,
) -> SiteResponse {
    let connection = database.get()?;
    let mut response = HashMap::<Level, Vec<TeamResponse>>::new();
    let vec = get_team_members(&connection)?;
    for x in vec {
        if response.get(&x.level).is_none(){
            response.insert(x.level.clone(), Vec::new());
        }
        let mut team = response.get_mut(&x.level).unwrap();
        team.push(TeamResponse{
            user: get_team_user(&x.user, &connection)?.unwrap(),
            description: x.description,
            level: x.level,
            created: x.created
        });
    }
    return APIResponse::respond_new(Some(response), &req);
}