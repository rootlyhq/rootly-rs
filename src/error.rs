#[derive(Debug, thiserror::Error)]
pub enum RootlyError {
    #[error("API error ({status}): {message}")]
    Api { status: u16, message: String },

    #[error("Rate limited after {retries} retries")]
    RateLimited { retries: u32 },

    #[error("Request error: {0}")]
    Reqwest(#[from] reqwest::Error),
}
