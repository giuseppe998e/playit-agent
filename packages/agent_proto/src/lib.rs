pub mod agent;
pub mod control;
pub mod hmac;
pub mod socket;

// Sync bytes encoding & decoding
#[cfg(feature = "blocking")]
pub mod blocking;

// Async bytes encoding & decoding
#[cfg(feature = "tokio")]
mod tokio;
#[cfg(feature = "tokio")]
pub use crate::tokio::{dec, en};
