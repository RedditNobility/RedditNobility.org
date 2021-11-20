use crate::action::{get_user_by_name, update_user};
use crate::api::api_validate;
use crate::api::apiresponse::APIResponse;
use crate::api::get_user_by_header;
use crate::models::{ClientKey, Level, Status, User, AuthToken};

use crate::siteerror::SiteError;
use crate::siteerror::SiteError::DBError;
use crate::usererror::UserError;
use crate::websiteerror::WebsiteError;
use crate::{action, utils, DbPool, RedditRoyalty};

use actix_web::{
    get, http, middleware, post, web, App, Error, HttpRequest, HttpResponse, HttpServer,
};
use bcrypt::verify;

use new_rawr::auth::AnonymousAuthenticator;
use new_rawr::client::RedditClient;

use serde::{Deserialize, Serialize};

use serde_json::Value;

use std::collections::HashMap;

use std::sync::{Arc, Mutex};


