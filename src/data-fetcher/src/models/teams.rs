use async_trait::async_trait;
use serde::Deserialize;
use sqlx::{pool::PoolConnection, MySql, Type};

use crate::types::{Gender, SportMonks};

#[derive(Debug, Deserialize, Type)]
#[sqlx(type_name = "teams_type")]
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
    pub gender: Gender,
    #[serde(rename = "type")]
    pub teams_type: TeamsType,
    pub image_path: String,
    pub founded: Option<u32>,
}

#[async_trait]
impl SportMonks for Teams {
    async fn insert(
        self,
        conn: &mut PoolConnection<sqlx::MySql>,
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
