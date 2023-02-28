use anyhow::{bail, Result};
use array2d::Array2D;
use polars::frame::DataFrame;
use polars::prelude::AnyValue;
use polars::{df, prelude::NamedFrom};
use reqwest::header::HeaderMap;
use reqwest::{Client, StatusCode};
use serde::{Deserialize, Serialize};
use std::time::Duration;

#[derive(Serialize, Deserialize, Debug)]
pub struct KlineResponse {
    pub prices: Vec<Vec<f64>>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct MarketItem {
    id: String,
    symbol: String,
    name: String,
    current_price: f32,
    market_cap: f32,
    total_volume: f32,
    price_change_percentage_24h: f32,
}

impl MarketItem {
    pub fn to_array(&self) -> Vec<AnyValue> {
        vec![
            AnyValue::Utf8(&self.id),
            AnyValue::Utf8(&self.symbol),
            AnyValue::Utf8(&self.name),
            AnyValue::Float32(self.current_price),
            AnyValue::Float32(self.market_cap),
            AnyValue::Float32(self.total_volume),
            AnyValue::Float32(self.price_change_percentage_24h),
        ]
    }
}

#[derive(Debug)]
pub struct CryptoClient {
    http_client: Client,
    base_url: String,
}

impl CryptoClient {
    pub fn new() -> CryptoClient {
        let mut headers = HeaderMap::new();
        headers.insert("accept", "application/json".parse().unwrap());
        let client = Client::builder()
            .default_headers(headers)
            .timeout(Duration::from_secs(10))
            .build()
            .unwrap();

        CryptoClient {
            http_client: client,
            base_url: String::from("https://api.coingecko.com/api/v3"),
        }
    }

    pub async fn get_kline(&self, id: &str, days: i32) -> Result<DataFrame> {
        let endpoint = format!(
            "{}/coins/{}/market_chart?vs_currency=usd&days={}",
            self.base_url, id, days
        );
        match self.http_client.get(endpoint).send().await {
            Ok(resp) => {
                let status = resp.status();
                if status != StatusCode::OK {
                    bail!("request return error, http code: {}", status)
                }
                let content = resp.text().await.unwrap();
                let response: KlineResponse = serde_json::from_str(&content).unwrap();
                let array = Array2D::from_rows(&response.prices).unwrap();
                let cols = vec!["time", "price"];

                let df = df!(&cols[0] => &array.as_columns()[0],
                                        &cols[1] => &array.as_columns()[1])?;
                Ok(df)
            }
            Err(err) => bail!("request failed, {}", err),
        }
    }

    pub async fn get_market(&self, order_type: &str, sort: &str) -> Result<DataFrame> {
        let endpoint = format!(
            "{}/coins/markets?vs_currency=usd&order={}_{}&per_page=100&page=1&sparkline=false",
            self.base_url, order_type, sort
        );
        match self.http_client.get(endpoint).send().await {
            Ok(resp) => {
                let status = resp.status();
                if status != StatusCode::OK {
                    bail!("request return error, http code: {}", status)
                }
                let content = resp.text().await.unwrap();
                let response: Vec<MarketItem> = serde_json::from_str(&content).unwrap();
                let data: Vec<Vec<AnyValue>> = response.iter().map(|x| x.to_array()).collect();

                let array = Array2D::from_rows(&data).unwrap();
                let cols = vec![
                    "id",
                    "symbol",
                    "name",
                    "price",
                    "market_cap",
                    "volume",
                    "change_percentage",
                ];
                let df = df!(&cols[0] => &array.as_columns()[0],
                                    &cols[1] => &array.as_columns()[1],
                                    &cols[2] => &array.as_columns()[2],
                                    &cols[3] => &array.as_columns()[3],
                                    &cols[4] => &array.as_columns()[4],
                                    &cols[5] => &array.as_columns()[5],
                                    &cols[6] => &array.as_columns()[6])?;

                Ok(df)
            }
            Err(err) => bail!("request failed, {}", err),
        }
    }
}
