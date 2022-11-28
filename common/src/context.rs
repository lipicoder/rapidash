//! Distributed execution context.

use crate::config::Config;

struct ContextState {
    /// Rapidash configuration
    config: Config,
    /// Scheduler host
    scheduler_host: String,
    /// Scheduler port
    scheduler_port: u16,
}

impl ContextState {
    pub fn new(scheduler_host: String, scheduler_port: u16, config: &Config) -> Self {
        Self {
            config: config.clone(),
            scheduler_host,
            scheduler_port,
        }
    }

    pub fn config(&self) -> &Config {
        &self.config
    }
}

pub struct Context {
    state: Arc<Mutex<ContextState>>,
    context: Arc<SessionContext>,
}
