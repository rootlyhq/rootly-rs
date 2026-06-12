use crate::generated::Client as GeneratedClient;
use reqwest::header::{HeaderMap, HeaderValue, ACCEPT, AUTHORIZATION, CONTENT_TYPE};

pub struct RootlyClient {
    inner: GeneratedClient,
}

pub struct RootlyClientConfig {
    pub token: String,
    pub base_url: String,
}

impl Default for RootlyClientConfig {
    fn default() -> Self {
        Self {
            token: String::new(),
            base_url: "https://api.rootly.com".to_string(),
        }
    }
}

impl RootlyClient {
    pub fn new(config: RootlyClientConfig) -> Self {
        let mut headers = HeaderMap::new();
        headers.insert(
            AUTHORIZATION,
            HeaderValue::from_str(&format!("Bearer {}", config.token))
                .expect("invalid token for header"),
        );
        headers.insert(
            CONTENT_TYPE,
            HeaderValue::from_static("application/vnd.api+json"),
        );
        headers.insert(ACCEPT, HeaderValue::from_static("application/vnd.api+json"));

        let http_client = reqwest::Client::builder()
            .default_headers(headers)
            .timeout(std::time::Duration::from_secs(30))
            .build()
            .expect("failed to build reqwest client");

        let inner = GeneratedClient::new_with_client(&config.base_url, http_client);

        Self { inner }
    }

    pub fn from_token(token: impl Into<String>) -> Self {
        Self::new(RootlyClientConfig {
            token: token.into(),
            ..Default::default()
        })
    }

    pub fn client(&self) -> &GeneratedClient {
        &self.inner
    }
}
