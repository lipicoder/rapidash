//! Rapidash error types
use datafusion::arrow::error::ArrowError;
use datafusion::error::DataFusionError;
use std::{
    error::Error,
    fmt::{Display, Formatter},
    io, result,
};

pub type Result<T> = result::Result<T, RapidashError>;

/// Rapidash error
/// copy from RapidashError
#[derive(Debug)]
pub enum RapidashError {
    NotImplemented(String),
    General(String),
    Internal(String),
    DataFusionError(DataFusionError),
    ArrowError(ArrowError),
    // SqlError(parser::ParserError),
    IoError(io::Error),
    // ReqwestError(reqwest::Error),
    // HttpError(http::Error),
    // KubeAPIError(kube::error::Error),
    // KubeAPIRequestError(k8s_openapi::RequestError),
    // KubeAPIResponseError(k8s_openapi::ResponseError),
    // TonicError(tonic::transport::Error),
    // GrpcError(tonic::Status),
    GrpcConnectionError(String),
    TokioError(tokio::task::JoinError),
    GrpcActionError(String),
    // (executor_id, map_stage_id, map_partition_id, message)
    FetchFailed(String, usize, usize, String),
    Cancelled,
}

pub fn ballista_error(message: &str) -> RapidashError {
    RapidashError::General(message.to_owned())
}

impl From<String> for RapidashError {
    fn from(e: String) -> Self {
        RapidashError::General(e)
    }
}

impl From<io::Error> for RapidashError {
    fn from(e: io::Error) -> Self {
        RapidashError::IoError(e)
    }
}

impl Display for RapidashError {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        match self {
            RapidashError::NotImplemented(ref desc) => {
                write!(f, "Not implemented: {}", desc)
            }
            RapidashError::General(ref desc) => write!(f, "General error: {}", desc),
            RapidashError::IoError(ref desc) => write!(f, "IO error: {}", desc),
            RapidashError::GrpcConnectionError(desc) => {
                write!(f, "Grpc connection error: {}", desc)
            }
            RapidashError::ArrowError(ref desc) => write!(f, "Arrow error: {}", desc),
            RapidashError::DataFusionError(ref desc) => {
                write!(f, "DataFusion error: {:?}", desc)
            }
            RapidashError::Internal(desc) => {
                write!(f, "Internal Ballista error: {}", desc)
            }
            RapidashError::TokioError(desc) => write!(f, "Tokio join error: {}", desc),
            RapidashError::GrpcActionError(desc) => {
                write!(f, "Grpc Execute Action error: {}", desc)
            }
            RapidashError::FetchFailed(executor_id, map_stage, map_partition, desc) => {
                write!(
                    f,
                    "Shuffle fetch partition error from Executor {}, map_stage {}, \
                map_partition {}, error desc: {}",
                    executor_id, map_stage, map_partition, desc
                )
            }
            RapidashError::Cancelled => write!(f, "Task cancelled"),
        }
    }
}

impl From<DataFusionError> for RapidashError {
    fn from(e: DataFusionError) -> Self {
        RapidashError::DataFusionError(e)
    }
}

impl From<ArrowError> for RapidashError {
    fn from(e: ArrowError) -> Self {
        match e {
            ArrowError::ExternalError(e) if e.downcast_ref::<RapidashError>().is_some() => {
                *e.downcast::<RapidashError>().unwrap()
            }
            ArrowError::ExternalError(e) if e.downcast_ref::<DataFusionError>().is_some() => {
                RapidashError::DataFusionError(*e.downcast::<DataFusionError>().unwrap())
            }
            other => RapidashError::ArrowError(other),
        }
    }
}

impl Error for RapidashError {}
