use async_trait::async_trait;
use serde::Deserialize;
use sqlx::{pool::PoolConnection, MySql};

use crate::types::SportMonks;

#[derive(Deserialize, Debug)]
pub struct Leagues {
    pub id: u32,
    pub name: String,
    pub short_code: Option<String>,
    pub active: bool,
    pub image_path: String,
}

#[async_trait]
impl SportMonks for Leagues {
    async fn insert(
        self,
        conn: &mut PoolConnection<sqlx::MySql>,
    ) -> Result<<MySql as sqlx::Database>::QueryResult, sqlx::Error> {
        let query = "INSERT IGNORE INTO Leagues (id, name, short_code, active, image_path) VALUES (?, ?, ?, ?, ?)";
        sqlx::query(query)
            .bind(self.id)
            .bind(self.name)
            .bind(self.short_code)
            .bind(self.active)
            .bind(self.image_path)
            .execute(conn)
            .await
    }
}
