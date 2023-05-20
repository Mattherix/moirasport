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
    url: &str,
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
    /*
    let conn = &mut pool.acquire().await?;
    let url = "https://api.sportmonks.com/v3/football/teams";
    fetch_and_insert_all::<models::Teams>(conn, &token, &url).await?;

    let conn = &mut pool.acquire().await?;
    let url = "https://api.sportmonks.com/v3/football/coaches?include=teams";
    fetch_and_insert_all::<models::Coachs>(conn, &token, &url).await?;

    let conn = &mut pool.acquire().await?;
    let url = "https://api.sportmonks.com/v3/football/players?include=position;detailedPosition";
    fetch_and_insert_all::<models::Players>(conn, &token, &url).await?;

    let conn = &mut pool.acquire().await?;
    let url = "https://api.sportmonks.com/v3/football/leagues";
    fetch_and_insert_all::<models::Leagues>(conn, &token, &url).await?;
    
    let conn = &mut pool.acquire().await?;
    let url = "https://api.sportmonks.com/v3/football/seasons";
    fetch_and_insert_all::<models::Seasons>(conn, &token, &url).await?;
    
    let conn = &mut pool.acquire().await?;
    let url = "https://api.sportmonks.com/v3/football/stages";
    fetch_and_insert_all::<models::Stage>(conn, &token, &url).await?;
    
    let conn = &mut pool.acquire().await?;
    let url = "https://api.sportmonks.com/v3/football/rounds";
    fetch_and_insert_all::<models::Rounds>(conn, &token, &url).await?;

    let conn = &mut pool.acquire().await?;
    let url = "https://api.sportmonks.com/v3/football/referees";
    fetch_and_insert_all::<models::Referees>(conn, &token, &url).await?;
    */
    let conn = &mut pool.acquire().await?;
    let url = "https://api.sportmonks.com/v3/football/fixtures?include=participants;scores;";
    fetch_and_insert_all::<models::Fixtures>(conn, &token, &url).await?;

    Ok(())
}
