use rootly::{RootlyClient, RootlyClientConfig};

#[test]
fn test_client_default_config() {
    let config = RootlyClientConfig::default();
    assert_eq!(config.base_url, "https://api.rootly.com");
    assert_eq!(config.token, "");
}

#[test]
fn test_client_from_token() {
    let _client = RootlyClient::from_token("test-token");
}

#[test]
fn test_client_custom_config() {
    let _client = RootlyClient::new(RootlyClientConfig {
        token: "custom-token".into(),
        base_url: "https://custom.rootly.com".into(),
    });
}

#[test]
fn test_rate_limit_config_defaults() {
    let config = rootly::RateLimitConfig::default();
    assert_eq!(config.max_retries, 3);
    assert_eq!(config.initial_backoff_ms, 1000);
}
