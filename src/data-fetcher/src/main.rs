use client::ApiClient;
use dotenvy::dotenv;
use sqlx::pool::PoolConnection;
use sqlx::MySqlPool;
use std::env;
use std::error::Error;

mod client;
mod models;
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
    url: &str
) -> Result<(), Box<dyn Error>> {
    let sport = ApiClient::new(token).await?;
    let data: Vec<T> = sport.all(url).await?;

    // TODO: find a way to do it concurrently
    for entity in data {
        entity.insert(conn).await?;
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
    let url = "https://api.sportmonks.com/v3/football/teams";
    fetch_and_insert_all::<models::Teams>(conn, &token, &url).await;

    let conn = &mut pool.acquire().await?;
    let url = "https://api.sportmonks.com/v3/football/coaches?include=teams";
    fetch_and_insert_all::<models::Coachs>(conn, &token, &url).await?;

    Ok(())
}
