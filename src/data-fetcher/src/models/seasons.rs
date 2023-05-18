use async_trait::async_trait;
use serde::Deserialize;
use sqlx::{pool::PoolConnection, MySql, Type};

use crate::types::{SportMonks, State};

#[derive(Debug, Deserialize, Type)]
#[sqlx(type_name = "tie_breaker_rule")]
#[sqlx(rename_all = "lowercase")]
pub enum TieBreakerRule {
    #[sqlx(rename = "points")]
    Points = 169,
    #[sqlx(rename = "head-to-head")]
    HeadToHead = 170,
    #[sqlx(rename = "goal-difference")]
    GoalDifference = 171,
    #[sqlx(rename = "goal-difference-goals-scored")]
    GoalDifferenceGoalsScored = 1526,
    #[sqlx(rename = "head-to-head-ranking-prev-stage")]
    HeadToHeadRankingPrevStage = 1678,
    #[sqlx(rename = "none")]
    Nothing = 573
}

#[derive(Deserialize, Debug)]
pub struct Seasons {
    pub id: u32,
    pub name: String,
    pub finished: bool,
    pub pending: bool,
    pub is_current: bool,
    pub tie_breaker_rule_id: u32,
    pub starting_at: Option<chrono::NaiveDate>,
    pub ending_at: Option<chrono::NaiveDate>,
    pub league_id: u32
}

#[async_trait]
impl SportMonks for Seasons {
    async fn insert(
        self,
        conn: &mut PoolConnection<sqlx::MySql>,
    ) -> Result<<MySql as sqlx::Database>::QueryResult, sqlx::Error> {
        let query = "INSERT IGNORE INTO Seasons \
        (id, name, state, starting_at, ending_at, tie_breaker_rule, league_id) \
        VALUES (?, ?, ?, ?, ?, ?, ?)";
        
        let mut state = None;
        if self.pending {
            state = Some(State::Pending);
        }
        if self.is_current {
            state = Some(State::Current);
        }
        if self.finished {
            state = Some(State::Finished);
        }

        let tie_breaker_rule = match self.tie_breaker_rule_id {
            169  => TieBreakerRule::Points,
            170  => TieBreakerRule::HeadToHead,
            171  => TieBreakerRule::GoalDifference,
            1526 => TieBreakerRule::GoalDifferenceGoalsScored,
            1678 => TieBreakerRule::HeadToHeadRankingPrevStage,
            _    => TieBreakerRule::Nothing
        };

        dbg!(&tie_breaker_rule);

        sqlx::query(query)
            .bind(self.id)
            .bind(self.name)
            .bind(state.expect("A Season can only be pending, current or finished"))
            .bind(self.starting_at)
            .bind(self.ending_at)
            .bind(tie_breaker_rule)
            .bind(self.league_id)
            .execute(conn)
            .await
    }
}
