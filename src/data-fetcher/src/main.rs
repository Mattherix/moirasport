use reqwest::{header, Result};
use reqwest::blocking::Client;
use url::Url;
use std::env;
use dotenv::dotenv;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
enum TeamsType {
    #[serde(rename = "domestic")] 
    Domestic
}

#[derive(Debug, Deserialize)]
struct Teams {
    id: u32,
    name: String,
    short_code: Option<String>,
    #[serde(rename = "type")] 
    teams_type: TeamsType,
    image_path: String,
    founded: u32,
}

#[derive(Deserialize, Debug)]
struct Response<T> {
    data: T
    /*
    pagination: String,
    subscription: String,
    rate_limit: String,
    timezone: String
    */
}

fn main() -> Result<()> {
    dotenv();
    let token = env::var("TOKEN").unwrap();

    let mut headers = header::HeaderMap::new();
    headers.insert(
        header::AUTHORIZATION,
        format!(
            "{}",
            token
        )
        .parse()
        .expect("Can't parse formatted token into a HeaderName"),
    );

    let client = Client::builder()
        .default_headers(headers)
        .build()?;
    
    let url = "https://api.sportmonks.com/v3/football/teams";
    
    let data: Response<Vec<Teams>> = client.get(url)
        .send()?
        .json()?;
    println!("{:?}", data);

    Ok(())
}
