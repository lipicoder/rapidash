//! Query plan
use crate::config::Config;
use crate::error::{RapidashError, Result};
// use crate::execution_plans::{DistributedQueryExec, ShuffleWriterExec, UnresolvedShuffleExec};
use async_trait::async_trait;
use datafusion::arrow::datatypes::Schema;
use datafusion::datasource::object_store::{ObjectStoreProvider, ObjectStoreRegistry};
use datafusion::error::DataFusionError;
use datafusion::execution::context::{QueryPlanner, SessionConfig, SessionContext, SessionState};
use datafusion::execution::runtime_env::{RuntimeConfig, RuntimeEnv};
use datafusion::logical_expr::LogicalPlan;
use datafusion::physical_plan::empty::EmptyExec;
use datafusion::physical_plan::{metrics, ExecutionPlan, RecordBatchStream};
use datafusion_proto::logical_plan::{
    AsLogicalPlan, DefaultLogicalExtensionCodec, LogicalExtensionCodec,
};
use object_store::ObjectStore;
use std::marker::PhantomData;
use std::sync::Arc;
use url::Url;

pub struct RapidashQueryPlanner<T: AsLogicalPlan> {
    scheduler_url: String,
    config: Config,
    extension_codec: Arc<dyn LogicalExtensionCodec>,
    plan_repr: PhantomData<T>,
}

impl<T: 'static + AsLogicalPlan> RapidashQueryPlanner<T> {
    pub fn new(scheduler_url: String, config: Config) -> Self {
        Self {
            scheduler_url,
            config,
            extension_codec: Arc::new(DefaultLogicalExtensionCodec {}),
            plan_repr: PhantomData,
        }
    }

    pub fn with_extension(
        scheduler_url: String,
        config: Config,
        extension_codec: Arc<dyn LogicalExtensionCodec>,
    ) -> Self {
        Self {
            scheduler_url,
            config,
            extension_codec,
            plan_repr: PhantomData,
        }
    }

    pub fn with_repr(
        scheduler_url: String,
        config: Config,
        extension_codec: Arc<dyn LogicalExtensionCodec>,
        plan_repr: PhantomData<T>,
    ) -> Self {
        Self {
            scheduler_url,
            config,
            extension_codec,
            plan_repr,
        }
    }
}

#[async_trait]
impl<T: 'static + AsLogicalPlan> QueryPlanner for RapidashQueryPlanner<T> {
    async fn create_physical_plan(
        &self,
        logical_plan: &LogicalPlan,
        session_state: &SessionState,
    ) -> std::result::Result<Arc<dyn ExecutionPlan>, DataFusionError> {
        match logical_plan {
            LogicalPlan::CreateExternalTable(_) => {
                // table state is managed locally in the BallistaContext, not in the scheduler
                Ok(Arc::new(EmptyExec::new(false, Arc::new(Schema::empty()))))
            }
            _ => Ok(Arc::new(DistributedQueryExec::with_repr(
                self.scheduler_url.clone(),
                self.config.clone(),
                logical_plan.clone(),
                self.extension_codec.clone(),
                self.plan_repr,
                session_state.session_id.clone(),
            ))),
        }
    }
}

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

/// An object store detector based on which features are enable for different kinds of object stores
pub struct FeatureBasedObjectStoreProvider;

impl ObjectStoreProvider for FeatureBasedObjectStoreProvider {
    /// Detector a suitable object store based on its url if possible
    /// Return the key and object store
    #[allow(unused_variables)]
    fn get_by_url(&self, url: &Url) -> datafusion::error::Result<Arc<dyn ObjectStore>> {
        #[cfg(any(feature = "hdfs", feature = "hdfs3"))]
        {
            let store = HadoopFileSystem::new(url.as_str());
            if let Some(store) = store {
                return Ok(Arc::new(store));
            }
        }

        #[cfg(feature = "s3")]
        {
            if url.to_string().starts_with("s3://") {
                if let Some(bucket_name) = url.host_str() {
                    let store = AmazonS3Builder::from_env()
                        .with_bucket_name(bucket_name)
                        .build()?;
                    return Ok(Arc::new(store));
                }
            }
        }

        Err(DataFusionError::Execution(format!(
            "No object store available for {}",
            url
        )))
    }
}
