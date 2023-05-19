use async_trait::async_trait;
use serde::Deserialize;
use sqlx::{pool::PoolConnection, MySql};

use crate::types::{SportMonks, Gender};

// Mostly placeholder data (is this true for the other leagues ?)
#[derive(Deserialize, Debug)]
pub struct Referees {
    pub id: u32,
    pub name: String,
    pub image_path: String,
    pub height: Option<u32>,
    pub weight: Option<u32>,
    pub date_of_birth: Option<chrono::NaiveDate>,
    pub gender: Option<Gender>
}

#[async_trait]
impl SportMonks for Referees {
    async fn insert(
        self,
        conn: &mut PoolConnection<sqlx::MySql>,
    ) -> Result<<MySql as sqlx::Database>::QueryResult, sqlx::Error> {
        let query = "INSERT IGNORE INTO Referees \
        (id, name, image_path, height, weight, date_of_birth, gender) \
         VALUES (?, ?, ?, ?, ?, ?, ?)";

        sqlx::query(query)
            .bind(self.id)
            .bind(self.name)
            .bind(self.image_path)
            .bind(self.height)
            .bind(self.weight)
            .bind(self.date_of_birth)
            .bind(self.gender)
            .execute(conn)
            .await
    }
}
