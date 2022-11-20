//! Context create
use crate::config::Config;
use crate::plan::object::FeatureBasedObjectStoreProvider;
use datafusion::datasource::object_store::ObjectStoreRegistry;
use datafusion::execution::context::{SessionConfig, SessionContext, SessionState};
use datafusion::execution::runtime_env::{RuntimeConfig, RuntimeEnv};
use datafusion_proto::logical_plan::AsLogicalPlan;
use std::sync::Arc;

use crate::plan::RapidashQueryPlanner;

/// Create a client DataFusion context that uses the RapidashQueryPlanner to send logical plans
/// to a Rapidash scheduler
pub fn create_ctx<T: 'static + AsLogicalPlan>(
    scheduler_url: String,
    session_id: String,
    config: &Config,
) -> SessionContext {
    let planner: Arc<RapidashQueryPlanner<T>> =
        Arc::new(RapidashQueryPlanner::new(scheduler_url, config.clone()));

    let session_config = SessionConfig::new()
        .with_target_partitions(config.default_shuffle_partitions())
        .with_information_schema(true);
    let mut session_state = SessionState::with_config_rt(
        session_config,
        Arc::new(RuntimeEnv::new(store_provider(RuntimeConfig::default())).unwrap()),
    )
    .with_query_planner(planner);
    session_state.session_id = session_id;
    // the SessionContext created here is the client side context, but the session_id is from server side.
    SessionContext::with_state(session_state)
}

/// Get a RuntimeConfig with specific ObjectStoreDetector in the ObjectStoreRegistry
pub fn store_provider(config: RuntimeConfig) -> RuntimeConfig {
    config.with_object_store_registry(Arc::new(ObjectStoreRegistry::new_with_provider(Some(
        Arc::new(FeatureBasedObjectStoreProvider),
    ))))
}
