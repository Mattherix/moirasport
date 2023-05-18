use async_recursion::async_recursion;
use dotenvy::dotenv;
use reqwest::header;
use reqwest::Client;
use sqlx::pool::PoolConnection;
use sqlx::{MySqlPool};
use types::Teams;
use std::env;
use std::error::Error;
use std::{thread, time::Duration};

mod types;

#[derive(Debug, Clone)]
pub struct SportMonks {
    client: Client,
}

impl SportMonks {
    pub async fn new(token: &str) -> Result<Self, reqwest::Error> {
        let mut headers = header::HeaderMap::new();
        headers.insert(
            header::AUTHORIZATION,
            format!("{}", token)
                .parse()
                .expect("Can't parse formatted token into a HeaderName"),
        );

        let client = Client::builder().default_headers(headers).build()?;

        Ok(SportMonks { client })
    }

    async fn get_response<T: types::SportMonks>(
        self,
        url: &str,
    ) -> Result<types::Response<Vec<T>>, reqwest::Error> {
        self.client
            .get(url)
            .send()
            .await?
            .json::<types::Response<Vec<T>>>()
            .await
    }

    pub async fn get<T: types::SportMonks>(self, url: &str) -> Result<Vec<T>, reqwest::Error> {
        Ok(self.get_response(url).await?.data)
    }

    #[async_recursion(?Send)]
    pub async fn all<T: types::SportMonks>(self, url: &str) -> Result<Vec<T>, reqwest::Error> {
        let mut items: Vec<T> = vec![];

        let response = self.clone().get_response(url).await?;
        items.extend(response.data);

        if response.pagination.has_more {
            if response.rate_limit.remaining <= 0 {
                println!(
                    "Rate limited, we will wait for {}",
                    response.rate_limit.remaining
                );
                thread::sleep(Duration::from_secs(
                    (response.rate_limit.remaining + 1).into(),
                ));
            }

            items.extend(
                self.all(&response.pagination.next_page.expect(
                    "Can't get next_page while there ismore page (has_more arg in pagination)",
                ))
                .await?,
            );
        }

        Ok(items)
    }
}

fn get_env(variable: &str) -> String {
    env::var(variable).expect(&format!(
        "Can't get {} env variable, please set it up in the .env file or as a variable",
        variable
    ))
}

async fn insert_all<T: types::SportMonks>(conn: &mut PoolConnection<sqlx::MySql>, token: &str) -> Result<(), Box<dyn Error>> {
    let sport = SportMonks::new(token).await?;
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

    insert_all::<Teams>(conn, &token).await?;

    Ok(())
}