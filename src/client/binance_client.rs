use anyhow::{bail, Error, Result};
use std::collections::BTreeMap;
use std::time::{SystemTime, UNIX_EPOCH};

use hex::encode as hex_encode;
use hmac::{Hmac, Mac};
use reqwest::blocking::Response;
use reqwest::header::{HeaderMap, HeaderName, HeaderValue, CONTENT_TYPE, USER_AGENT};
use reqwest::StatusCode;
use serde::de::DeserializeOwned;
use sha2::Sha256;

#[derive(Clone)]
pub struct Client {
    api_key: String,
    secret_key: String,
    host: String,
    inner_client: reqwest::blocking::Client,
}

impl Client {
    pub fn new(api_key: Option<String>, secret_key: Option<String>, host: String) -> Self {
        Client {
            api_key: api_key.unwrap_or_default(),
            secret_key: secret_key.unwrap_or_default(),
            host,
            inner_client: reqwest::blocking::Client::builder()
                .pool_idle_timeout(None)
                .build()
                .unwrap(),
        }
    }

    pub fn get_signed<T: DeserializeOwned>(
        &self,
        endpoint: String,
        request: Option<String>,
    ) -> Result<T> {
        let url = self.sign_request(endpoint, request);
        let client = &self.inner_client;
        let response = client
            .get(url.as_str())
            .headers(self.build_headers(true)?)
            .send()?;

        self.handler(response).unwrap()
    }

    pub fn post_signed<T: DeserializeOwned>(&self, endpoint: String, request: String) -> Result<T> {
        let url = self.sign_request(endpoint, Some(request));
        let client = &self.inner_client;
        let response = client
            .post(url.as_str())
            .headers(self.build_headers(true)?)
            .send()?;

        self.handler(response).unwrap()
    }

    pub fn get<T: DeserializeOwned>(&self, endpoint: String, request: Option<String>) -> Result<T> {
        let mut url: String = format!("{}{}", self.host, String::from(endpoint));
        if let Some(request) = request {
            if !request.is_empty() {
                url.push_str(format!("?{}", request).as_str());
            }
        }

        let client = &self.inner_client;
        let response = client.get(url.as_str()).send()?;

        self.handler(response).unwrap()
    }

    // Request must be signed
    fn sign_request(&self, endpoint: String, request: Option<String>) -> String {
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

    fn build_headers(&self, content_type: bool) -> HeaderMap {
        let mut custom_headers = HeaderMap::new();

        custom_headers.insert(USER_AGENT, HeaderValue::from_static("binance-rs"));
        if content_type {
            custom_headers.insert(
                CONTENT_TYPE,
                HeaderValue::from_static("application/x-www-form-urlencoded"),
            );
        }
        custom_headers.insert(
            HeaderName::from_static("x-mbx-apikey"),
            HeaderValue::from_str(self.api_key.as_str())?,
        );

        custom_headers
    }

    fn handler<T: DeserializeOwned>(&self, response: Response) -> Result<T> {
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
            StatusCode::BAD_REQUEST => {
                bail!("Bad request");
            }
        }
    }
}

pub fn build_signed_request(parameters: BTreeMap<String, String>, recv_window: u64) -> String {
    build_signed_request_custom(parameters, recv_window, SystemTime::now())
}

pub fn build_signed_request_custom(
    mut parameters: BTreeMap<String, String>,
    recv_window: u64,
    start: SystemTime,
) -> String {
    if recv_window > 0 {
        parameters.insert("recvWindow".into(), recv_window.to_string());
    }
    let timestamp = get_timestamp(start);
    parameters.insert("timestamp".into(), timestamp.to_string());
    build_request(parameters)
}

pub fn build_request(parameters: BTreeMap<String, String>) -> String {
    let mut request = String::new();
    for (key, value) in parameters {
        let param = format!("{}={}&", key, value);
        request.push_str(param.as_ref());
    }
    request.pop();
    request
}

fn get_timestamp(start: SystemTime) -> u64 {
    let since_epoch = start.duration_since(UNIX_EPOCH)?;
    since_epoch.as_secs() * 1000 + u64::from(since_epoch.subsec_nanos()) / 1_000_000
}

#[cfg(test)]
mod tests {
    use super::Client;

    #[test]
    fn test_excute() {
        let api_key =
            Some("uzmfZmBb2jlmCNi5O9hp27o0CJxa5v42Lec3kVvFkXSPOUl9r8qa3CEFBhAkQThP".into());
        let secret_key =
            Some("LJjVjovJ1oaGowqPE0dRQBFIH9NIxI14Fsq4RTi3NWjFWHQf3yZaEMnkqywW8FIB".into());
        let client = Client::new(
            api_key,
            secret_key,
            String::from("https://fapi.binance.com"),
        );
        client.get_signed(String::from("/fapi/v1/ticker/24hr"), None);
    }
}
