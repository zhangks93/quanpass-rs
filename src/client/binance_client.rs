use std::{
    collections::BTreeMap,
    time::{SystemTime, UNIX_EPOCH},
};

use anyhow::bail;
use hex::encode as hex_encode;
use hmac::{Hmac, Mac};
use reqwest::{
    blocking::Response,
    header::{HeaderMap, HeaderName, HeaderValue},
    StatusCode,
};
use serde::{de::DeserializeOwned, Deserialize, Serialize};
use sha2::Sha256;

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct ServerTime {
    server_time: i64,
}

#[derive(Clone)]
pub struct BinanceClient {
    api_key: String,
    secret_key: String,
    host: String,
    inner_client: reqwest::blocking::Client,
}

impl BinanceClient {
    pub fn new() -> Self {
        BinanceClient {
            api_key: String::from(
                "uzmfZmBb2jlmCNi5O9hp27o0CJxa5v42Lec3kVvFkXSPOUl9r8qa3CEFBhAkQThP",
            ),
            secret_key: String::from(
                "LJjVjovJ1oaGowqPE0dRQBFIH9NIxI14Fsq4RTi3NWjFWHQf3yZaEMnkqywW8FIB",
            ),
            host: String::from("https://api.binance.com"),
            inner_client: reqwest::blocking::Client::builder()
                .pool_idle_timeout(None)
                .build()
                .unwrap(),
        }
    }

    pub fn new_with_host(host: &str) -> Self {
        BinanceClient {
            api_key: String::from(
                "uzmfZmBb2jlmCNi5O9hp27o0CJxa5v42Lec3kVvFkXSPOUl9r8qa3CEFBhAkQThP",
            ),
            secret_key: String::from(
                "LJjVjovJ1oaGowqPE0dRQBFIH9NIxI14Fsq4RTi3NWjFWHQf3yZaEMnkqywW8FIB",
            ),
            inner_client: reqwest::blocking::Client::builder()
                .pool_idle_timeout(None)
                .build()
                .unwrap(),
            host: host.to_string(),
            // ... initialize other fields ...
        }
    }

    fn build_headers(&self, content_type: bool) -> Result<HeaderMap, anyhow::Error> {
        let mut custom_headers = HeaderMap::new();

        custom_headers.insert("user-agent", HeaderValue::from_static("binance-rs"));
        if content_type {
            custom_headers.insert(
                "content-type",
                HeaderValue::from_static("application/x-www-form-urlencoded"),
            );
        }
        custom_headers.insert(
            HeaderName::from_static("x-mbx-apikey"),
            HeaderValue::from_str(self.api_key.as_str())?,
        );

        Ok(custom_headers)
    }

    pub fn build_request(&self, parameters: BTreeMap<String, String>) -> String {
        let mut request = String::new();
        for (key, value) in parameters {
            let param = format!("{}={}&", key, value);
            request.push_str(param.as_ref());
        }
        request.pop();
        request
    }

    fn get_timestamp(&self, start: SystemTime) -> u64 {
        let since_epoch = start.duration_since(UNIX_EPOCH).unwrap();
        since_epoch.as_secs() * 1000 + u64::from(since_epoch.subsec_nanos()) / 1_000_000
    }

    pub fn build_signed_request(&self, mut parameters: BTreeMap<String, String>) -> String {
        let timestamp = self.get_timestamp(SystemTime::now());
        parameters.insert("timestamp".into(), timestamp.to_string());
        return self.build_request(parameters);
    }

    pub fn build_future_signed_request(
        &self,
        parameters: BTreeMap<String, String>,
        recv_window: u64,
    ) -> String {
        {
            let mut parameters = parameters;
            if recv_window > 0 {
                parameters.insert("recvWindow".into(), recv_window.to_string());
            }
            let server_time: ServerTime = self.get("/fapi/v1/time", None).unwrap();
            let timestamp = server_time.server_time as u64;

            parameters.insert("timestamp".into(), timestamp.to_string());
            self.build_request(parameters)
        }
    }

    fn sign_request(&self, endpoint: &str, request: Option<String>) -> String {
        if let Some(request) = request {
            let mut signed_key =
                Hmac::<Sha256>::new_from_slice(self.secret_key.as_bytes()).unwrap();
            signed_key.update(request.as_bytes());
            let signature = hex_encode(signed_key.finalize().into_bytes());
            let request_body: String = format!("{}&signature={}", request, signature);
            format!("{}{}?{}", self.host, String::from(endpoint), request_body)
        } else {
            let signed_key = Hmac::<Sha256>::new_from_slice(self.secret_key.as_bytes()).unwrap();
            let signature = hex_encode(signed_key.finalize().into_bytes());
            let request_body: String = format!("&signature={}", signature);
            format!("{}{}?{}", self.host, String::from(endpoint), request_body)
        }
    }

    fn handler<T: DeserializeOwned>(&self, response: Response) -> Result<T, anyhow::Error> {
        match response.status() {
            StatusCode::OK => Ok(response.json::<T>()?),
            StatusCode::INTERNAL_SERVER_ERROR => {
                bail!("Internal Server Error");
            }
            StatusCode::SERVICE_UNAVAILABLE => {
                bail!("Service Unavailable");
            }
            StatusCode::UNAUTHORIZED => {
                bail!("Unauthorized");
            }
            s => {
                bail!(format!(
                    "Code:{:?}, Received mesaage: {:?}",
                    s,
                    response.text()
                ));
            }
        }
    }

    pub fn get_signed<T: DeserializeOwned>(
        &self,
        endpoint: &str,
        request: Option<String>,
    ) -> Result<T, anyhow::Error> {
        let url = self.sign_request(endpoint, request);
        let client = &self.inner_client;
        let response = client
            .get(url.as_str())
            .headers(self.build_headers(true)?)
            .send()?;

        self.handler(response)
    }

    pub fn post_signed<T: DeserializeOwned>(
        &self,
        endpoint: &str,
        request: Option<String>,
    ) -> Result<T, anyhow::Error> {
        let url = self.sign_request(endpoint, request);
        let client = &self.inner_client;
        let response = client
            .post(url.as_str())
            .headers(self.build_headers(true)?)
            .send()?;

        self.handler(response)
    }

    pub fn delete_signed<T: DeserializeOwned>(
        &self,
        endpoint: &str,
        request: Option<String>,
    ) -> Result<T, anyhow::Error> {
        let url = self.sign_request(endpoint, request);
        let client = &self.inner_client;
        let response = client
            .delete(url.as_str())
            .headers(self.build_headers(true)?)
            .send()?;

        self.handler(response)
    }
    pub fn get<T: DeserializeOwned>(
        &self,
        endpoint: &str,
        request: Option<String>,
    ) -> Result<T, anyhow::Error> {
        let mut url: String = format!("{}{}", self.host, String::from(endpoint));
        if let Some(request) = request {
            if !request.is_empty() {
                url.push_str(format!("?{}", request).as_str());
            }
        }

        let client = &self.inner_client;
        let response = client.get(url.as_str()).send()?;

        self.handler(response)
    }

    pub fn post<T: DeserializeOwned>(&self, endpoint: &str) -> Result<T, anyhow::Error> {
        let url: String = format!("{}{}", self.host, String::from(endpoint));

        let client = &self.inner_client;
        let response = client
            .post(url.as_str())
            .headers(self.build_headers(false)?)
            .send()?;

        self.handler(response)
    }
}
