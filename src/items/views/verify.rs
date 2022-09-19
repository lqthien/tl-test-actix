use jelly::accounts::User;
use jelly::actix_web::{web::Path, HttpRequest};
use jelly::prelude::*;
use jelly::request::DatabasePool;
use jelly::Result;

use crate::accounts::views::utils::validate_token;
use crate::accounts::Account;

