use async_trait::async_trait;
use serde::Deserialize;
use sqlx::{pool::PoolConnection, MySql};

use crate::types::{SportMonks, State};

#[derive(Deserialize, Debug)]
pub struct Rounds {
    pub id: u32,
    pub name: String,
    pub finished: bool,
    pub is_current: bool,
    pub starting_at: Option<chrono::NaiveDate>,
    pub ending_at: Option<chrono::NaiveDate>,
    pub stage_id: u32
}

#[async_trait]
impl SportMonks for Rounds {
    async fn insert(
        self,
        conn: &mut PoolConnection<sqlx::MySql>,
    ) -> Result<<MySql as sqlx::Database>::QueryResult, sqlx::Error> {
        let query = "INSERT IGNORE INTO Rounds \
        (id, name, state, starting_at, ending_at, stage_id) \
        VALUES (?, ?, ?, ?, ?, ?)";
        
        let mut state = None;
        if self.is_current {
            state = Some(State::Current);
        }
        if self.finished {
            state = Some(State::Finished);
        }

        sqlx::query(query)
            .bind(self.id)
            .bind(self.name)
            .bind(state)
            .bind(self.starting_at)
            .bind(self.ending_at)
            .bind(self.stage_id)
            .execute(conn)
            .await
    }
}
