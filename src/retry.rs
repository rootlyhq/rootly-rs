use std::future::Future;
use std::time::Duration;
use tokio::time::sleep;

pub struct RateLimitConfig {
    pub max_retries: u32,
    pub initial_backoff_ms: u64,
}

impl Default for RateLimitConfig {
    fn default() -> Self {
        Self {
            max_retries: 3,
            initial_backoff_ms: 1000,
        }
    }
}

pub async fn with_backoff<F, Fut, T, E>(config: RateLimitConfig, mut f: F) -> Result<T, E>
where
    F: FnMut() -> Fut,
    Fut: Future<Output = Result<T, E>>,
{
    let mut retries = 0;
    loop {
        match f().await {
            Ok(val) => return Ok(val),
            Err(e) => {
                if retries >= config.max_retries {
                    return Err(e);
                }
                let backoff_ms = config.initial_backoff_ms * 2u64.pow(retries);
                let jitter_ms = rand::random::<u64>() % (backoff_ms / 2 + 1);
                sleep(Duration::from_millis(backoff_ms + jitter_ms)).await;
                retries += 1;
            }
        }
    }
}
