use anyhow::{bail, Result};
use reqwest::header::HeaderMap;
use reqwest::{Client, StatusCode};
use scraper::{Html, Selector};
use std::time::Duration;

pub struct RssClient {
    rss_client: Client,
}

impl RssClient {
    pub fn new() -> RssClient {
        let mut headers = HeaderMap::new();
        headers.insert("Accept", "text/html,application/xhtml+xml,application/xml;q=0.9,image/avif,image/webp,image/apng,*/*;q=0.8,application/signed-exchange;v=b3;q=0.7".parse().unwrap());
        headers.insert("Accept-language", "zh-CN,zh;q=0.9".parse().unwrap());
        headers.insert("User-Agent", "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/102.0.0.0 Safari/537.36".parse().unwrap());

        let client = Client::builder()
            .default_headers(headers)
            .timeout(Duration::from_secs(10))
            .build()
            .unwrap();

        RssClient { rss_client: client }
    }

    pub async fn collect(&self, endpoint: &str) -> Result<String> {
        match self.rss_client.get(endpoint).send().await {
            Ok(resp) => {
                let status = resp.status();
                if status != StatusCode::OK {
                    bail!("request return error, http code: {}", status)
                }
                let content = resp.text().await.unwrap();
                let fragment = scraper::Html::parse_fragment(&content);
                let selector = Selector::parse("item").unwrap();
                let items: Vec<String> = fragment
                    .select(&selector)
                    .into_iter()
                    .map(|x| x.inner_html())
                    .collect();

                Ok(items[1].clone())
            }
            Err(err) => bail!("request failed, {}", err),
        }
    }
}
