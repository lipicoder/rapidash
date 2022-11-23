//! Rapidash error types
use std::{
    error::Error,
    fmt::{Display, Formatter},
    io,
};

/// Rapidash error
/// copy from RapidashError
#[derive(Debug)]
pub enum RapidashError {
    NotImplemented(String),
    General(String),
    Internal(String),
    IoError(io::Error),
    GrpcConnectionError(String),
    GrpcActionError(String),
    FetchFailed(String, usize, usize, String),
    Cancelled,
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
            RapidashError::Internal(desc) => {
                write!(f, "Internal Rapidash error: {}", desc)
            }
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

impl Error for RapidashError {}
