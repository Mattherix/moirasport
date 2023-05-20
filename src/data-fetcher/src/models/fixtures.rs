use std::fmt;

use async_trait::async_trait;
use chrono::{NaiveDateTime};
use serde::{Deserialize, Deserializer, de::{Visitor, self}};
use sqlx::{pool::PoolConnection, MySql, Type};

use crate::types::{SportMonks};

use super::{Teams};

// Some GoalCount are strings :(
#[derive(Debug, Type, Clone, PartialEq)]
#[sqlx(type_name = "goalcount")]
pub struct GoalCount {
    pub goal: u64
}

impl<'de> Deserialize<'de> for GoalCount {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where D: Deserializer<'de>
    {
        struct IdVisitor;

        impl<'de> Visitor<'de> for IdVisitor {
            type Value = GoalCount;

            fn expecting(&self, f: &mut fmt::Formatter) -> fmt::Result {
                f.write_str("Fixtures lenght as a u64 or a str")
            }

            fn visit_u64<E>(self, id: u64) -> Result<Self::Value, E>
                where E: de::Error
            {
                Ok(GoalCount{
                    goal: id
                })
            }

            fn visit_str<E>(self, id: &str) -> Result<Self::Value, E>
                where E: de::Error
            {
                id.parse().map(|val| GoalCount {goal: val}).map_err(de::Error::custom)
            }
        }

        deserializer.deserialize_any(IdVisitor)
    }
}

#[derive(Debug, Deserialize, Type, Clone, PartialEq)]
pub enum TeamsLocation {
    #[serde(rename = "home")]
    Home,
    #[serde(rename = "away")]
    Away
}

#[derive(Debug, Deserialize, Clone, PartialEq)]
pub struct Goal {
    goals: GoalCount,
    participant: TeamsLocation 
}

#[derive(Debug, Deserialize, Clone, PartialEq, Type)]
#[sqlx(type_name = "description_type")]
#[sqlx(rename_all = "lowercase")]
pub enum DescriptionType {
    #[serde(rename = "1ST_HALF")]
    FirstHalf,
    #[serde(rename = "2ND_HALF")]
    #[serde(alias = "2nd-half")] 
    SecondHalf,
    #[serde(rename = "CURRENT")]
    Current,
    #[serde(rename = "PENALTY_SHOOTOUT")]
    PenaltyShootout,
    #[serde(rename = "ET")]
    Et
}

#[derive(Debug, Deserialize, Clone, PartialEq)]
pub struct Score {
    pub id: u32,
    pub fixture_id: u32,
    pub type_id: u32,
    pub participant_id: u32,
    pub score: Goal,
    pub description: DescriptionType
}

// Mostly placeholder data (is this true for the other leagues ?)
#[derive(Deserialize, Debug)]
pub struct Fixtures {
    pub id: u32,
    pub name: Option<String>,
    pub starting_at: String, // YYYY-MM-DD hh:mm:ss
    pub length: Option<u32>,
    scores: Vec<Score>,
    participants: Vec<Teams>,
    pub stage_id: u32,
    pub round_id: Option<u32>
}

#[async_trait]
impl SportMonks for Fixtures {
    async fn insert(
        self,
        conn: &mut PoolConnection<sqlx::MySql>,
    ) -> Result<<MySql as sqlx::Database>::QueryResult, sqlx::Error> {    
        let currents_scores: Vec<Score> = self.scores.into_iter().filter(|score| {
            score.description == DescriptionType::Current
        }).collect();

        let home_score = currents_scores.clone()
            .into_iter()
            .find(|score| score.score.participant == TeamsLocation::Home);

        let away_score = currents_scores
            .into_iter()
            .find(|score| score.score.participant == TeamsLocation::Away);

        let mut home_score_goal = None;
        let mut home_id = None;
        let mut away_score_goal = None;
        let mut away_id = None;
        if let Some(score) = home_score {
            home_score_goal = Some(score.score.goals.goal);
            home_id = Some(score.participant_id);
        }
        if let Some(score) = away_score {
            away_score_goal = Some(score.score.goals.goal);
            away_id = Some(score.participant_id);
        }
        
        let starting_at = NaiveDateTime::parse_from_str(&self.starting_at, "%Y-%m-%d %H:%M:%S").unwrap();

        let query = "\
        INSERT IGNORE INTO Fixtures (id, name, starting_at, length, home_score, away_score, home_team_id, away_team_id, stage_id, round_id)\
        VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?)";

        sqlx::query(query)
            .bind(self.id)
            .bind(self.name)
            .bind(starting_at)
            .bind(self.length)
            .bind(home_score_goal)
            .bind(away_score_goal)
            .bind(home_id)
            .bind(away_id)
            .bind(self.stage_id)
            .bind(self.round_id)
            .execute(conn)
            .await
    }
}
