use std::collections::BTreeMap;

use anyhow::{bail, Result};
use hex::encode as hex_encode;
use hmac::{Hmac, Mac};
use reqwest::header::{HeaderMap, HeaderName, HeaderValue, CONTENT_TYPE, USER_AGENT};
use serde::{Deserialize, Serialize};
use sha2::Sha256;

#[derive(Serialize, Deserialize, Debug)]
struct ServerTime {
    server_time: i64,
}

#[derive(Clone)]
pub struct Binance {
    api_key: String,
    secret_key: String,
    host: String,
    inner_client: reqwest::blocking::Client,
}

impl Binance {
    pub fn new() -> Self {
        Binance {
            api_key: String::from(
                "uzmfZmBb2jlmCNi5O9hp27o0CJxa5v42Lec3kVvFkXSPOUl9r8qa3CEFBhAkQThP",
            ),
            secret_key: String::from(
                "LJjVjovJ1oaGowqPE0dRQBFIH9NIxI14Fsq4RTi3NWjFWHQf3yZaEMnkqywW8FIB",
            ),
            host: String::from("https://fapi.binance.com"),
            inner_client: reqwest::blocking::Client::builder()
                .pool_idle_timeout(None)
                .build()
                .unwrap(),
        }
    }

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
            HeaderValue::from_str(self.api_key.as_str()).unwrap(),
        );

        custom_headers
    }

    pub fn build_signed_request(
        &self,
        parameters: BTreeMap<String, String>,
        recv_window: u64,
    ) -> String {
        {
            let mut parameters = parameters;
            if recv_window > 0 {
                parameters.insert("recvWindow".into(), recv_window.to_string());
            }

            let s: String = self.get("/fapi/v1/time".to_owned(), None).unwrap();
            let server_time: ServerTime = serde_json::from_str(&s).unwrap();

            parameters.insert("timestamp".into(), server_time.server_time.to_string());
            self.build_request(parameters)
        }
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

    pub fn get(&self, endpoint: String, request: Option<String>) -> Result<String> {
        let mut url: String = format!("{}{}", self.host, String::from(endpoint));
        if let Some(request) = request {
            if !request.is_empty() {
                url.push_str(format!("?{}", request).as_str());
            }
        }

        let client = &self.inner_client;
        match client.get(url.as_str()).send() {
            Ok(resp) => {
                let content = resp.text().unwrap();
                Ok(content)
            }
            Err(err) => bail!("request failed, {}", err),
        }
    }

    pub fn get_signed(&self, endpoint: String, request: Option<String>) -> Result<String> {
        let url = self.sign_request(endpoint, request);

        let client = &self.inner_client;
        match client
            .get(url.as_str())
            .headers(self.build_headers(true))
            .send()
        {
            Ok(resp) => {
                let content = resp.text().unwrap();
                Ok(content)
            }
            Err(err) => bail!("get failed, {}", err),
        }
    }

    pub fn post_signed(&self, endpoint: String, request: String) -> Result<String> {
        let url = self.sign_request(endpoint, Some(request));

        let client = &self.inner_client;
        match client
            .post(url.as_str())
            .headers(self.build_headers(true))
            .send()
        {
            Ok(resp) => {
                let content = resp.text().unwrap();
                Ok(content)
            }
            Err(err) => bail!("post failed, {}", err),
        }
    }

    pub fn delete_signed(&self, endpoint: String, request: Option<String>) -> Result<String> {
        let url = self.sign_request(endpoint, request);

        let client = &self.inner_client;
        match client
            .delete(url.as_str())
            .headers(self.build_headers(true))
            .send()
        {
            Ok(resp) => {
                let content = resp.text().unwrap();
                Ok(content)
            }
            Err(err) => bail!("delete failed, {}", err),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Binance;
    use std::collections::BTreeMap;

    #[test]
    fn test_excute() {
        let binance = Binance::new();

        let request = binance.build_signed_request(BTreeMap::new(), 1000);
        let result = binance
            .get_signed("/fapi/v1/openOrders".to_owned(), Some(request))
            .unwrap();
        println!("{:?}", result);
    }
}
