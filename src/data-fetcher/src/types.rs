use async_trait::async_trait;
use std::fmt::Debug;

use serde::Deserialize;
use sqlx::{pool::PoolConnection, MySql, Type};

#[async_trait]
pub trait SportMonks: for<'a> Deserialize<'a> + Debug {
    async fn insert(
        self,
        conn: &mut PoolConnection<sqlx::MySql>,
    ) -> Result<<MySql as sqlx::Database>::QueryResult, sqlx::Error>;
}

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

#[derive(Debug, Deserialize, Type, Clone)]
#[sqlx(type_name = "gender")]
#[sqlx(rename_all = "lowercase")]
#[serde(rename_all = "lowercase")]
pub enum Gender {
    Male,
    Female,
    Neutral,
}
