// Sync bytes encoding & decoding
#[cfg(feature = "blocking")]
mod blocking;
#[cfg(feature = "blocking")]
pub use self::blocking::{dec::MessageDecode, en::MessageEncode};

// Async bytes encoding & decoding
#[cfg(feature = "tokio")]
mod tokio;
#[cfg(feature = "tokio")]
pub use self::tokio::{dec::AsyncMessageDecode, en::AsyncMessageEncode};
