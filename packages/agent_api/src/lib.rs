mod client;
mod error;

pub use client::{ApiClient, ApiClientBuilder};
pub use error::{Error, ErrorKind, Result};

pub const DEFAULT_API_BASE_URL: &str = "https://api.playit.cloud/";
pub const DEFAULT_CLIENT_USER_AGENT: &str = concat!("playit-agent/", env!("CARGO_PKG_VERSION"));
