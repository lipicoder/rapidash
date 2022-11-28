//! configuration.

use std::collections::HashMap;
use std::result;

use error::Result;

use crate::config::Config;

pub type ParseResult<T> = result::Result<T, String>;

/// Configuration builder
pub struct ConfigBuilder {
    settings: HashMap<String, String>,
}

impl Default for ConfigBuilder {
    /// Create a new config builder
    fn default() -> Self {
        Self {
            settings: HashMap::new(),
        }
    }
}

impl ConfigBuilder {
    /// Create a new config with an additional setting
    pub fn set(&self, k: &str, v: &str) -> Self {
        let mut settings = self.settings.clone();
        settings.insert(k.to_owned(), v.to_owned());
        Self { settings }
    }

    pub fn build(&self) -> Result<Config> {
        Config::with_settings(self.settings.clone())
    }
}
