//! Distributed execution context.

use datafusion::catalog::TableReference;
use datafusion::dataframe::DataFrame;
use datafusion::datasource::{source_as_provider, TableProvider};
use datafusion::error::{DataFusionError, Result};
use datafusion::logical_expr::{CreateExternalTable, LogicalPlan, TableScan};
use datafusion::prelude::{
    AvroReadOptions, CsvReadOptions, ParquetReadOptions, SessionConfig, SessionContext,
};
use datafusion::sql::parser::{DFParser, Statement as DFStatement};
use engine::config::Config;
use log::info;
use std::sync::{Arc, Mutex};

use engine::rpc::create_client_conn;

pub struct Context {
    state: Arc<Mutex<ContextState>>,
    context: Arc<SessionContext>,
}

impl Context {
    /// Create a new context to connect to a remote engine cluster.
    pub async fn remote(host: &str, port: u16, config: &Config) -> engine::error::Result<Self> {
        let state = ContextState::new(host.to_owned(), port, config);

        let url = format!("http://{}:{}", &state.host, state.port);

        info!("Connecting to Ballista scheduler at {}", url.clone());

        let connection = create_client_conn(url.clone())
            .await
            .map_err(|e| DataFusionError::Execution(format!("{:?}", e)))?;

        // TODO: use a real client
        let ctx = SessionContext::new();

        Ok(Self {
            state: Arc::new(Mutex::new(state)),
            context: Arc::new(ctx),
        })
    }
}

pub struct ContextState {
    /// Rapidash configuration
    config: Config,

    /// Engine Host
    host: String,

    /// Engine port
    port: u16,
}

impl ContextState {
    pub fn new(host: String, port: u16, config: &Config) -> Self {
        Self {
            host,
            port,
            config: config.clone(),
        }
    }

    pub fn config(&self) -> &Config {
        &self.config
    }
}
