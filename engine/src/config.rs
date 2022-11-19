//! Rapidash engine configuration.
//!

use std::collections::HashMap;

/// Rapidash configuration
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Config {
    /// Settings stored in map for easy serde
    settings: HashMap<String, String>,
}

impl Config {}
