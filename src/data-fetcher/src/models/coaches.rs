use async_trait::async_trait;
use chrono;
use serde::Deserialize;
use sqlx::{pool::PoolConnection, MySql};

use crate::types::{Gender, SportMonks};

// https://api.sportmonks.com/v3/football/coaches?api_token={{api_token}}&include=teams;

#[derive(Deserialize, Debug, Clone)]
pub struct TeamOfTheCoach {
    id: u32,
    team_id: u32,
    coach_id: u32,
    position_id: u32,
    active: bool,
    start: Option<chrono::NaiveDate>,
    end: Option<chrono::NaiveDate>,
    temporary: bool,
}

#[derive(Deserialize, Debug, Clone)]
pub struct Coachs {
    pub id: u32,
    pub firstname: String,
    pub lastname: String,
    pub image_path: String,
    pub height: Option<u32>,
    pub weight: Option<u32>,
    pub date_of_birth: Option<chrono::NaiveDate>,
    pub gender: Gender,
    // Must use teams include
    pub teams: Vec<TeamOfTheCoach>,
}

#[async_trait]
impl SportMonks for Coachs {
    async fn insert(
        self,
        conn: &mut PoolConnection<sqlx::MySql>,
    ) -> Result<<MySql as sqlx::Database>::QueryResult, sqlx::Error> {
        let mut most_recent_team = None;
        // Find most recent team if it exist
        if self.teams.len() > 0 {
            let mut recent_team = self.teams[0].clone();
            for team in self.teams {
                if let Some(start) = team.start {
                    if recent_team.start == None {
                        recent_team = team;
                    } else {
                        if start > recent_team.start.expect("Start date is not None") {
                            recent_team = team;
                        }
                    }
                }
            }
            most_recent_team = Some(recent_team);
        }

        let mut team_id: Option<u32> = None;
        if let Some(team) = most_recent_team {
            team_id = Some(team.team_id);
        }

        let query = "INSERT IGNORE INTO Coachs \
        (id, firstname, lastname, image_path, height, weight, date_of_birth, gender, team_id) \
         VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?)";

        sqlx::query(query)
            .bind(self.id)
            .bind(self.firstname)
            .bind(self.lastname)
            .bind(self.image_path)
            .bind(self.height)
            .bind(self.weight)
            .bind(self.date_of_birth)
            .bind(self.gender)
            .bind(team_id)
            .execute(conn)
            .await
    }
}
