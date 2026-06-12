#[allow(clippy::all, unused_imports, dead_code)]
pub mod generated;

mod client;
mod error;
pub mod retry;

pub use client::{RootlyClient, RootlyClientConfig};
pub use error::RootlyError;
pub use generated::types;
pub use generated::Client;
pub use progenitor_client::{Error as ClientError, ResponseValue};
pub use retry::RateLimitConfig;
