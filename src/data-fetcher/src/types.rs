use std::fmt::Debug;
use async_trait::async_trait;

use serde::Deserialize;
use sqlx::{MySql, Type, pool::PoolConnection};

#[async_trait]
pub trait SportMonks: for<'a> Deserialize<'a> + Debug {
    async fn insert(
        self,
        conn: &mut PoolConnection<sqlx::MySql>
    ) -> Result<<MySql as sqlx::Database>::QueryResult, sqlx::Error>;
}

#[derive(Debug, Deserialize, Type)]
#[sqlx(type_name = "color")]
#[sqlx(rename_all = "lowercase")]
pub enum TeamsType {
    #[serde(rename = "domestic")]
    Domestic,
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

#[async_trait]
impl SportMonks for Teams {
    async fn insert(
        self,
        conn: &mut PoolConnection<sqlx::MySql>
    ) -> Result<<MySql as sqlx::Database>::QueryResult, sqlx::Error> {
        let query = "INSERT IGNORE INTO Teams (id, name, type, short_code, image_path, founded) VALUES (?, ?, ?, ?, ?, ?)";
        sqlx::query(query)
            .bind(self.id)
            .bind(self.name)
            .bind(self.teams_type)
            .bind(self.short_code)
            .bind(self.image_path)
            .bind(self.founded)
            .execute(conn)
            .await
    }
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
