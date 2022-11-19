//! Distributed execution context.

use engine::config::Config;

pub struct ContextState {
    /// Rapidash configuration
    config: Config,
}

impl ContextState {
    pub fn new(config: &Config) -> Self {
        Self {
            config: config.clone(),
        }
    }

    pub fn config(&self) -> &Config {
        &self.config
    }
}
