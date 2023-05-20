use std::fmt;

use async_trait::async_trait;
use chrono::Utc;
use serde::{Deserialize, Deserializer, de::{Visitor, self}};
use sqlx::{pool::PoolConnection, MySql, Type};

use crate::types::{SportMonks};

use super::Teams;

// Some GoalCount are strings :(
#[derive(Debug)]
pub struct GoalCount(u64);

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
                Ok(GoalCount(id))
            }

            fn visit_str<E>(self, id: &str) -> Result<Self::Value, E>
                where E: de::Error
            {
                id.parse().map(GoalCount).map_err(de::Error::custom)
            }
        }

        deserializer.deserialize_any(IdVisitor)
    }
}

#[derive(Debug, Deserialize)]
pub struct Goal {
    goals: GoalCount,
    participant: String 
}

#[derive(Debug, Deserialize, Type)]
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

#[derive(Debug, Deserialize)]
pub struct Score {
    id: u32,
    fixture_id: u32,
    type_id: u32,
    participant_id: u32,
    score: Goal,
    description: DescriptionType
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
        unimplemented!()
    }
}
