use async_trait::async_trait;
use serde::Deserialize;
use sqlx::{pool::PoolConnection, MySql, Type};

use crate::types::{Gender, SportMonks};

// position field include
#[derive(Debug, Deserialize, Type)]
#[sqlx(type_name = "player_role")]
#[sqlx(rename_all = "lowercase")]
#[serde(rename_all = "lowercase")]
pub enum PlayerRole {
    Goalkeeper,
    Defender,
    Midfielder,
    Attacker,
    Coach,
    Unknown
}

// position field include
#[derive(Debug, Deserialize, Type)]
#[sqlx(type_name = "player_position")]
#[sqlx(rename_all = "lowercase")]
pub enum PlayerPosition {
    #[serde(rename = "goalkeeper")]
    Goalkeeper,
    #[serde(rename = "centre-back")]
    CentreBack,
    #[serde(rename = "defensive-midfied")]
    DefensiveMidfied,
    #[serde(rename = "attacking-midfied")]
    AttackingMidfied,
    #[serde(rename = "centre-forward")]
    CentreForward,
    #[serde(rename = "left-wing")]
    LeftWing,
    #[serde(rename = "central-midfied")]
    CentralMidfied,
    #[serde(rename = "right-back")]
    RightBack,
    #[serde(rename = "left-back")]
    LeftBack,
    #[serde(rename = "right-wing")]
    RightWing,
    #[serde(rename = "left-midfield")]
    LeftMidfield,
    #[serde(rename = "right-midfield")]
    RightMidfield,
    #[serde(rename = "coach")]
    Coach,
    #[serde(rename = "assistant-coach")]
    AssistantCoach,
    #[serde(rename = "secondary_striker")]
    SecondaryStriker,
    #[serde(rename = "goalkeeping-coach")]
    GoalkeepingCoach,
    #[serde(rename = "defender")]
    Defender,
}
#[derive(Deserialize, Debug)]
pub struct PlayerRoleResponse {
    pub code: PlayerRole
}

#[derive(Deserialize, Debug)]
pub struct PlayerPositionResponse {
    pub code: PlayerPosition
}

#[derive(Deserialize, Debug)]
pub struct Players {
    pub id: u32,
    // in db "role"
    pub position: Option<PlayerRoleResponse>,
    // in db "position"
    pub detailedposition: Option<PlayerPositionResponse>,
    pub firstname: String,
    pub lastname: String,
    pub height: Option<u32>,
    pub weight: Option<u32>,
    pub date_of_birth: Option<chrono::NaiveDate>,
    pub gender: Option<Gender>,
    pub image_path: String
}

#[async_trait]
impl SportMonks for Players {
    async fn insert(
        self,
        conn: &mut PoolConnection<sqlx::MySql>,
    ) -> Result<<MySql as sqlx::Database>::QueryResult, sqlx::Error> {
        unimplemented!()
    }
}
