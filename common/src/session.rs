//! Client session
use chrono::{DateTime, Utc};
use std::fmt::Debug;
use std::sync::{Arc, RwLock};

/// SessionContext is the main interface for executing queries with Rapidash.
/// It stands for the connection between user and cluster.

#[derive(Clone)]
pub struct SessionContext {
    /// Uuid for the session
    session_id: String,
    /// Session start time
    pub session_start_time: DateTime<Utc>,
    /// Shared session state for the session
    pub state: Arc<RwLock<SessionState>>,
}

impl Default for SessionContext {
    fn default() -> Self {
        Self::new()
    }
}

/// Execution context for registering data sources and executing queries
#[derive(Clone)]
pub struct SessionState {
    /// Uuid for the session
    pub session_id: String,
    /// Session configuration
    pub config: SessionConfig,
}

impl Debug for SessionState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("SessionState")
            .field("session_id", &self.session_id)
            // TODO should we print out more?
            .finish()
    }
}

/// Configuration options for session context
#[derive(Clone)]
pub struct SessionConfig {
    /// Configuration options
    pub config_options: Arc<RwLock<ConfigOptions>>,
}
