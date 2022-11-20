//! Query planner
use crate::config::Config;
use async_trait::async_trait;
use datafusion::arrow::datatypes::Schema;
use datafusion::error::DataFusionError;
use datafusion::execution::context::{QueryPlanner, SessionState};
use datafusion::logical_expr::LogicalPlan;
use datafusion::physical_plan::empty::EmptyExec;
use datafusion::physical_plan::ExecutionPlan;
use datafusion_proto::logical_plan::{
    AsLogicalPlan, DefaultLogicalExtensionCodec, LogicalExtensionCodec,
};
use std::marker::PhantomData;
use std::sync::Arc;

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
