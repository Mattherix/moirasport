use std::fmt::Debug;

use serde::Deserialize;
use sqlx::{Type};

pub trait SportMonks: for<'a> Deserialize<'a> + Debug {}

#[derive(Debug, Deserialize, Type)]
#[sqlx(type_name = "color")]
#[sqlx(rename_all = "lowercase")]
pub enum TeamsType {
    #[serde(rename = "domestic")]
    Domestic
}

#[derive(Deserialize, Debug)]
pub struct Teams {
    pub id: u32,
    pub name: String,
    pub short_code: Option<String>,
    #[serde(rename = "type")]
    pub teams_type: TeamsType,
    pub image_path: String,
    pub founded: Option<u32>,
}

impl SportMonks for Teams {}

#[derive(Deserialize, Debug, Clone)]
pub struct Response<T> {
    pub data: T,
    pub pagination: Pagination,
    pub rate_limit: RateLimit, /*
                               subscription: String,
                               timezone: String
                               */
}

#[derive(Deserialize, Debug, Clone)]
pub struct Pagination {
    pub count: u32,
    pub per_page: u32,
    pub current_page: u32,
    pub next_page: Option<String>,
    pub has_more: bool,
}

#[derive(Deserialize, Debug, Clone)]
pub struct RateLimit {
    pub resets_in_seconds: u32,
    pub remaining: u32,
    pub requested_entity: String,
}
