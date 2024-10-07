use candid::CandidType;
use serde::Deserialize;
use std::{collections::HashMap, fmt::Display};

/// A wrapper around a `HashMap` that maintains insertion order.
#[derive(CandidType, Deserialize, Default)]
pub(crate) struct OrderedMap {
    map: HashMap<Id, String>,
    keys: Vec<Id>,
}

impl OrderedMap {
    /// Create a new `OrderedMap`.
    pub(crate) fn new() -> Self {
        Self {
            map: HashMap::new(),
            keys: Vec::new(),
        }
    }

    /// Insert a key-value pair into the map.
    pub(crate) fn insert(&mut self, key: Id, value: String) {
        self.map.insert(key.clone(), value);
        self.keys.push(key);
    }

    /// Get the value associated with a key.
    pub(crate) fn get(&self, key: &Id) -> Option<&String> {
        self.map.get(key)
    }

    /// Get an iterator over the key-value pairs in the map.
    pub(crate) fn iter(&self) -> impl Iterator<Item = (&Id, &String)> {
        self.keys
            .iter()
            .map(move |key| (key, self.map.get(key).unwrap()))
    }

    /// Remove a key from the map.
    pub(crate) fn remove(&mut self, key: &Id) {
        self.map.remove(key);
        self.keys.retain(|k| k != key);
    }
}

#[derive(serde::Deserialize, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Debug, CandidType)]
pub(crate) struct Id(String);

impl Display for Id {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl Id {
    /// Create a new `Id`.
    pub(crate) fn new() -> Self {
        Self(Self::new_uuid())
    }

    /// Generate a new UUID without any dependency
    /// on [Uuid](https://crates.io/crates/uuid) or
    /// [Ulid](https://crates.io/crates/ulid).
    ///
    /// Since UUID & ULID have a wasm-bindgen dependency, we can't use them in the IC.
    fn new_uuid() -> String {
        #[cfg(test)]
        let seed = {
            // `ic_cdk::api::time()` is available only to canisters,
            // so we can't use it in tests.

            // Get the current time in nanoseconds as a u128
            let nanos = std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .expect("Failed to get system time")
                .as_nanos();

            // Convert u128 to u64 by combining the higher and lower 64 bits
            ((nanos >> 64) as u64) ^ (nanos as u64)
        };

        #[cfg(not(test))]
        // We use this because SystemTime APIs are not available in the wasm build.
        // Get the current time in nanoseconds as a u64
        let seed = ic_cdk::api::time();

        const CHARSET: &str = "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789";
        const LENGTH: usize = CHARSET.len();

        (0..LENGTH)
            .map(|i| {
                // Limit the shift amount to be within the range 0-63 to avoid overflow
                let shift_amount = (i * 2) % 64;
                let byte = (seed >> shift_amount) as u8;
                let idx = (byte % LENGTH as u8) as usize;
                CHARSET.as_bytes()[idx] as char
            })
            .collect::<String>()
    }
}
