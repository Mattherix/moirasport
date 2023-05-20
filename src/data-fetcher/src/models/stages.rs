use async_trait::async_trait;
use serde::Deserialize;
use sqlx::{pool::PoolConnection, MySql, Type};

use crate::types::{SportMonks, State};

#[derive(Debug, Deserialize, Type)]
#[sqlx(type_name = "stage_type")]
#[sqlx(rename_all = "lowercase")]
pub enum StageType {
    #[sqlx(rename = "group-stage")]
    GroupStage = 223,
    #[sqlx(rename = "knock-out")]
    KnockOut = 224,
    #[sqlx(rename = "qualifying")]
    Qualifying = 225
}

#[derive(Deserialize, Debug)]
pub struct Stage {
    pub id: u32,
    pub name: String,
    pub stage_type_id: Option<u32>,
    pub sort_order: Option<u32>,
    pub finished: bool,
    pub is_current: bool,
    pub starting_at: Option<chrono::NaiveDate>,
    pub ending_at: Option<chrono::NaiveDate>,
    pub season_id: u32
}

#[async_trait]
impl SportMonks for Stage {
    async fn insert(
        self,
        conn: &mut PoolConnection<sqlx::MySql>,
    ) -> Result<<MySql as sqlx::Database>::QueryResult, sqlx::Error> {
        let query = "INSERT IGNORE INTO Stages \
        (id, name, type, sort_order, state, starting_at, ending_at, season_id) \
        VALUES (?, ?, ?, ?, ?, ?, ?, ?)";
        
        let mut state = None;
        if self.is_current {
            state = Some(State::Current);
        }
        if self.finished {
            state = Some(State::Finished);
        }

        let stage_type = match self.stage_type_id {
            Some(223) => Some(StageType::GroupStage),
            Some(224) => Some(StageType::KnockOut),
            Some(225) => Some(StageType::Qualifying),
            _   => None
        };

        sqlx::query(query)
            .bind(self.id)
            .bind(self.name)
            .bind(stage_type)
            .bind(self.sort_order)
            .bind(state)
            .bind(self.starting_at)
            .bind(self.ending_at)
            .bind(self.season_id)
            .execute(conn)
            .await
    }
}
