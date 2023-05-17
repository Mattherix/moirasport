use reqwest::{header, Result};
use reqwest::blocking::Client;
use std::env;
use dotenvy::dotenv;

use crate::types::Teams;
mod types;

pub struct SportMonks {
    client: Client
}

impl SportMonks {
    pub fn new(token: &str) -> Result<Self>{
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

        Ok(SportMonks {
            client
        })
    }

    pub fn get<T: types::SportMonks>(self, url: &str) -> Result<Vec<T>> {   
        let response = self.client.get(url)
            .send()?
            .json::<types::Response<Vec<T>>>()?;
        
        Ok(response.data)
    }
}

fn main() -> Result<()> {
    dotenv();
    let token = env::var("TOKEN").expect("Can't get TOKEN env variable, please set it up in the .env file or as a variable");

    let sport = SportMonks::new(token.as_str())?;
    let url = "https://api.sportmonks.com/v3/football/teams";
    let data: Vec<types::Teams> = sport.get(url)?;

    println!("{:#?}", data);

    Ok(())
}
