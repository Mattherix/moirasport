use client::ApiClient;
use dotenvy::dotenv;
use sqlx::pool::PoolConnection;
use sqlx::MySqlPool;
use std::env;
use std::error::Error;
use types::Teams;

mod client;
mod types;

fn get_env(variable: &str) -> String {
    env::var(variable).expect(&format!(
        "Can't get {} env variable, please set it up in the .env file or as a variable",
        variable
    ))
}

async fn fetch_and_insert_all<T: types::SportMonks>(
    conn: &mut PoolConnection<sqlx::MySql>,
    token: &str,
) -> Result<(), Box<dyn Error>> {
    let sport = ApiClient::new(token).await?;
    let url = "https://api.sportmonks.com/v3/football/teams";
    let data: Vec<T> = sport.all(url).await?;

    // TODO: find a way to do it concurrently
    for team in data {
        team.insert(conn).await?;
    }

    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    dotenv()?;
    let token = get_env("TOKEN");
    let pool = MySqlPool::connect(&format!(
        "mysql://{}:{}@{}/{}",
        get_env("MYSQL_USER"),
        get_env("MYSQL_PASSWORD"),
        get_env("MYSQL_HOST"),
        get_env("MYSQL_DATABASE")
    ))
    .await?;

    let conn = &mut pool.acquire().await?;

    fetch_and_insert_all::<Teams>(conn, &token).await?;

    Ok(())
}
