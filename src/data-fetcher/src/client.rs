use std::{thread, time::Duration};

use async_recursion::async_recursion;
use reqwest::{header, Client};

use crate::types::{self, SportMonks};

#[derive(Debug, Clone)]
pub struct ApiClient {
    client: Client,
}

impl ApiClient {
    pub async fn new(token: &str) -> Result<Self, reqwest::Error> {
        let mut headers = header::HeaderMap::new();
        headers.insert(
            header::AUTHORIZATION,
            format!("{}", token)
                .parse()
                .expect("Can't parse formatted token into a HeaderName"),
        );

        let client = Client::builder().default_headers(headers).build()?;

        Ok(ApiClient { client })
    }

    async fn get_response<T: SportMonks>(
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

    pub async fn get<T: SportMonks>(self, url: &str) -> Result<Vec<T>, reqwest::Error> {
        Ok(self.get_response(url).await?.data)
    }

    #[async_recursion(?Send)]
    pub async fn all<T: SportMonks>(self, url: &str) -> Result<Vec<T>, reqwest::Error> {
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
