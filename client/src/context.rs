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

use engine::generated::rapidash::scheduler_grpc_client::SchedulerGrpcClient;
use engine::generated::rapidash::{ExecuteQueryParams, KeyValuePair};
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

        // create tonic channel
        let connection = create_client_conn(url.clone())
            .await
            .map_err(|e| DataFusionError::Execution(format!("{:?}", e)))?;

        // create scheduler by connection
        let mut scheduler = SchedulerGrpcClient::new(connection);

        // get remote session id
        let remote_session_id = scheduler
            .execute_query(ExecuteQueryParams {
                query: None,
                settings: config
                    .settings()
                    .iter()
                    .map(|(k, v)| KeyValuePair {
                        key: k.to_owned(),
                        value: v.to_owned(),
                    })
                    .collect::<Vec<_>>(),
                optional_session_id: None,
            })
            .await
            .map_err(|e| DataFusionError::Execution(format!("{:?}", e)))?
            .into_inner()
            .session_id;

        info!(
            "Server side SessionContext created with session id: {}",
            remote_session_id
        );

        // create context
        let ctx = {
            create_df_ctx_with_ballista_query_planner::<LogicalPlanNode>(
                scheduler_url,
                remote_session_id,
                state.config(),
            )
        };

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
