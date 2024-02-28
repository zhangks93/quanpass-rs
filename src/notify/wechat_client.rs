use std::time::Duration;

use futures::TryFutureExt;
use reqwest::Client;

pub struct WechatClient {
    url: String,
}

impl WechatClient {
    pub fn new() -> WechatClient {
        WechatClient {
            url: "https://xizhi.qqoq.net/XZ121f7102a882c19c840f754d6b728a2b.send".to_owned(),
        }
    }

    pub async fn send(&self, title: &str, content: &str) {
        let client = Client::builder()
            .timeout(Duration::from_secs(10))
            .build()
            .unwrap();
        let endpoint = self.url.as_str().to_owned() + "?title=" + title + "&content=" + content;
        println!("{}", endpoint);
        client.get(endpoint).send().await.unwrap();
    }
}

mod tests {
    use super::WechatClient;


    #[test]
    fn test_excute() {
        WechatClient::new().send("title", "content");
    }
}