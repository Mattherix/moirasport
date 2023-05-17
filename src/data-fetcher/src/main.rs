use dotenvy::dotenv;
use reqwest::blocking::Client;
use reqwest::{header};
use sqlx::{MySqlPool, Row};
use std::env;
use std::error::Error;
use std::{thread, time::Duration};

use crate::types::Teams;
mod types;

#[derive(Debug, Clone)]
pub struct SportMonks {
    client: Client,
}

impl SportMonks {
    pub fn new(token: &str) -> Result<Self, reqwest::Error> {
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

    fn get_response<T: types::SportMonks>(self, url: &str) -> Result<types::Response<Vec<T>>, reqwest::Error> {
        self.client
            .get(url)
            .send()?
            .json::<types::Response<Vec<T>>>()
    }

    pub fn get<T: types::SportMonks>(self, url: &str) -> Result<Vec<T>, reqwest::Error> {
        Ok(self.get_response(url)?.data)
    }

    pub fn all<T: types::SportMonks>(self, url: &str) -> Result<Vec<T>, reqwest::Error> {
        let mut items: Vec<T> = vec![];

        let response = self.clone().get_response(url)?;
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

            items.extend(self.all(&response.pagination.next_page.expect(
                "Can't get next_page while there ismore page (has_more arg in pagination)",
            ))?);
        }

        Ok(items)
    }
}

fn get_env(variable: &str) -> String {
    env::var(variable)
        .expect(&format!("Can't get {} env variable, please set it up in the .env file or as a variable", variable))
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    dotenv();
    let token = get_env("TOKEN");
    let pool = MySqlPool::connect(
        &format!(
            "mysql://{}:{}@{}/{}",
            get_env("MYSQL_USER"),
            get_env("MYSQL_PASSWORD"),
            get_env("MYSQL_HOST"),
            get_env("MYSQL_DATABASE")
        )
    ).await?;

    let mut res = sqlx::query("SELECT 1 + 1 as sum")
        .fetch_one(&pool)
        .await?;

    let sum: i32 = res.get("sum");
    println!("{:#?}", sum);
    return Ok(());

    let sport = SportMonks::new(token.as_str())?;
    let url = "https://api.sportmonks.com/v3/football/teams";
    let data: Vec<types::Teams> = sport.all(url)?;

    println!("{:#?}", data);

    Ok(())
}
