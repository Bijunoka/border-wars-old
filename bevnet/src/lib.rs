use serde::{de::DeserializeOwned, Serialize};
use std::{
    collections::hash_map::DefaultHasher,
    hash::{Hash, Hasher},
};

#[cfg(feature = "client")]
pub mod client;

#[cfg(feature = "server")]
pub mod server;

mod tcp;

/// A packet that can be sent over a [Connection].
pub trait Packet: DeserializeOwned + Serialize + Send + Sync {
    /// Returns a unique identifier for this packet.
    ///
    /// This function uses [std::any::type_name] to get a string
    /// representation of the type of the object and returns the
    /// hash of that string. This is not perfect... but I didn't
    /// find a better solution.
    fn packet_id() -> u64 {
        let mut hasher = DefaultHasher::new();
        std::any::type_name::<Self>().hash(&mut hasher);
        hasher.finish()
    }
}

impl<T: DeserializeOwned + Serialize + Send + Sync> Packet for T {}
