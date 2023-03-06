pub mod agent;
pub mod control;
pub mod hmac;
pub mod socket;

// Sync bytes encoding & decoding
#[cfg(feature = "blocking")]
mod blocking;
#[cfg(feature = "blocking")]
pub use crate::blocking::{dec::MessageDecode, en::MessageEncode};

// Async bytes encoding & decoding
#[cfg(feature = "tokio")]
mod tokio;
#[cfg(feature = "tokio")]
pub use crate::tokio::{dec::AsyncMessageDecode, en::AsyncMessageEncode};
