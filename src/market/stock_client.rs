use anyhow::{bail, Result};
use array2d::Array2D;
use polars::frame::DataFrame;
use polars::{df, prelude::NamedFrom};
use reqwest::header::HeaderMap;
use reqwest::{Client, StatusCode};
use serde::{Deserialize, Serialize};
use std::time::Duration;

#[derive(Serialize, Deserialize, Debug)]
pub struct Kline {
    pub symbol: String,
    pub column: Vec<String>,
    pub item: Vec<Vec<Option<f64>>>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Response {
    pub data: Kline,
    pub error_code: i8,
    pub error_description: String,
}

pub async fn request_data(endpoint: &str) -> Result<DataFrame> {
    let mut headers = HeaderMap::new();
    headers.insert("Content-Type", "application/json".parse().unwrap());
    headers.insert("user-agent","	Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/109.0.0.0 Safari/537.36".parse().unwrap());
    headers.insert("Cookie", "device_id=6e17e4a3e8450862becc9b2548f3cb54; xq_a_token=06c970814873215375f1cd02e4c8e64b740f6704; xqat=06c970814873215375f1cd02e4c8e64b740f6704; xq_r_token=9546eea976a2e2f78e2667bb2221518d5306c5b6; xq_id_token=eyJ0eXAiOiJKV1QiLCJhbGciOiJSUzI1NiJ9.eyJ1aWQiOi0xLCJpc3MiOiJ1YyIsImV4cCI6MTY3NzM3MDg0NiwiY3RtIjoxNjc1MjM4MjU3NjM2LCJjaWQiOiJkOWQwbjRBWnVwIn0.CqJSA1jYdF74ccP3TifAneRkpEHrLq5nA5-GRUPXKqwQdaWNfiPKGxJ3ZHW49WVhitVL7kABsc8aL85HjjeRdjFUbJR-ApHdXpv76uTdwBOdbFDDfJFEgesHyyxOYrkjeq53ntkRnmdG8TTAOs-kxsCHKPMrQB_rDsjS2b7mGHwPjWkC44plOsKgZ8TaaICNxI4Ey9kfRX26GzuZaGAwutPFPQNxbqfrwyklCYDPs5YOcWmNtDwK63n9Udwsz1QKoAu-Drz9psL3Fu8Or-75ylkG4foObVG4V8L7cdeC511eUSz5YCOjkF_dg2BTnVAkmq8VZ58C2KOVTc96kac1Rw; u=151675238286631; Hm_lvt_1db88642e346389874251b5a1eded6e3=1673250633,1675238290; Hm_lpvt_1db88642e346389874251b5a1eded6e3=1675238297".parse().unwrap());

    let client = Client::builder()
        .default_headers(headers)
        .timeout(Duration::from_secs(10))
        .build()
        .unwrap();
    match client.get(endpoint).send().await {
        Ok(resp) => {
            let status = resp.status();
            if status != StatusCode::OK {
                bail!("request return error, http code: {}", status)
            }
            let content = resp.text().await.unwrap();
            let response: Response = serde_json::from_str(&content).unwrap();
            // let data = Array::from_shape_vec((284,12), response.data.item.concat()).unwrap().t();
            let array = Array2D::from_rows(&response.data.item).unwrap();
            let cols = response.data.column;
            let df = df!(&cols[0] => &array.as_columns()[0],
                                    &cols[1] => &array.as_columns()[1],
                                    &cols[2] => &array.as_columns()[2],
                                    &cols[3] => &array.as_columns()[3],
                                    &cols[4] => &array.as_columns()[4],
                                    &cols[5] => &array.as_columns()[5],
                                    &cols[7] => &array.as_columns()[7],
                                    &cols[8] => &array.as_columns()[8])?;
            Ok(df)
        }
        Err(err) => bail!("request failed, {}", err),
    }
}
