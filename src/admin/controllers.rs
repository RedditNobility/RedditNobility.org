use std::path::Path;
use actix_web::{get, web, HttpRequest};

use crate::api_response::{APIResponse, SiteResponse};
use crate::{Database, User, RN, utils};
use new_rawr::structures::submission::Submission;
use new_rawr::traits::{Content, Votable};

use crate::error::response::{bad_request, error, not_found, unauthorized};
use crate::user::action::{get_found_users, get_user_by_name, update_properties};
use crate::user::utils::get_user_by_header;
use serde::{Deserialize, Serialize};
use std::str::FromStr;
use actix_web::post;
use actix_web::http::StatusCode;
use actix_web::web::Json;
use strum::ParseError;
use crate::user::models::{Status};
use crate::utils::get_current_time;


