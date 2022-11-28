//! Rapidash error types
use crate::error::RapidashError;
use arrow::error::ArrowError;
use object_store;
use parquet::errors::ParquetError;
use sqlparser::parser::ParserError;
use std::result as std_result;
use std::{
    error::Error,
    fmt::{Display, Formatter},
    io,
};

pub type Result<T> = std_result::Result<T, RapidashError>;

/// Error type for generic operations that could result in RapidashError::External
pub type GenericError = Box<dyn Error + Send + Sync>;

/// Rapidash error
/// copy from RapidashError
#[derive(Debug)]
pub enum RapidashError {
    General(String),
    GrpcConnectionError(String),
    GrpcActionError(String),
    FetchFailed(String, usize, usize, String),
    Cancelled,
    /// Error returned by arrow.
    ArrowError(ArrowError),
    /// Wraps an error from the Parquet crate
    #[cfg(feature = "parquet")]
    ParquetError(ParquetError),
    /// Error associated to I/O operations and associated traits.
    IoError(io::Error),
    /// Error returned when SQL is syntactically incorrect.
    SQL(ParserError),
    /// Error returned on a branch that we know it is possible
    /// but to which we still have no implementation for.
    /// Often, these errors are tracked in our issue tracker.
    NotImplemented(String),
    /// Error returned as a consequence of an error in DataFusion.
    /// This error should not happen in normal usage of DataFusion.
    // DataFusions has internal invariants that we are unable to ask the compiler to check for us.
    // This error is raised when one of those invariants is not verified during execution.
    Internal(String),
    /// This error happens with schema-related errors, such as schema inference not possible
    /// and non-unique column names.
    SchemaError(SchemaError),
    /// Error returned during execution of the query.
    /// Examples include files not found, errors in parsing certain types.
    Execution(String),
    /// This error is thrown when a consumer cannot acquire memory from the Memory Manager
    /// we can just cancel the execution of the partition.
    ResourcesExhausted(String),
    /// Errors originating from outside DataFusion's core codebase.
    /// For example, a custom S3Error from the crate datafusion-objectstore-s3
    External(GenericError),
    /// Error with additional context
    Context(String, Box<RapidashError>),
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
            RapidashError::General(e) => write!(f, "General error: {}", e),
            RapidashError::IoError(e) => write!(f, "Io error: {}", e),
            RapidashError::GrpcConnectionError(e) => write!(f, "Grpc connection error: {}", e),
            RapidashError::GrpcActionError(e) => write!(f, "Grpc action error: {}", e),
            RapidashError::FetchFailed(e, i, j, s) => {
                write!(f, "Fetch failed: {}, {}, {}, {}", e, i, j, s)
            }
            RapidashError::Cancelled => write!(f, "Task cancelled"),
            RapidashError::ArrowError(ref desc) => write!(f, "Arrow error: {}", desc),
            #[cfg(feature = "parquet")]
            RapidashError::ParquetError(ref desc) => {
                write!(f, "Parquet error: {}", desc)
            }
            RapidashError::IoError(ref desc) => write!(f, "IO error: {}", desc),
            RapidashError::SQL(ref desc) => {
                write!(f, "SQL error: {:?}", desc)
            }
            RapidashError::NotImplemented(ref desc) => {
                write!(f, "This feature is not implemented: {}", desc)
            }
            RapidashError::Internal(ref desc) => {
                write!(
                    f,
                    "Internal error: {}. This was likely caused by a bug in DataFusion's \
                    code and we would welcome that you file an bug report in our issue tracker",
                    desc
                )
            }
            RapidashError::SchemaError(ref desc) => {
                write!(f, "Schema error: {}", desc)
            }
            RapidashError::Execution(ref desc) => {
                write!(f, "Execution error: {}", desc)
            }
            RapidashError::ResourcesExhausted(ref desc) => {
                write!(f, "Resources exhausted: {}", desc)
            }
            RapidashError::External(ref desc) => {
                write!(f, "External error: {}", desc)
            }
            RapidashError::Context(ref desc, ref err) => {
                write!(f, "{}\ncaused by\n{}", desc, *err)
            }
        }
    }
}

impl Error for RapidashError {}

impl From<io::Error> for RapidashError {
    fn from(e: io::Error) -> Self {
        RapidashError::IoError(e)
    }
}

impl From<ArrowError> for RapidashError {
    fn from(e: ArrowError) -> Self {
        RapidashError::ArrowError(e)
    }
}

impl From<RapidashError> for ArrowError {
    fn from(e: RapidashError) -> Self {
        match e {
            RapidashError::ArrowError(e) => e,
            RapidashError::External(e) => ArrowError::ExternalError(e),
            other => ArrowError::ExternalError(Box::new(other)),
        }
    }
}

#[cfg(feature = "parquet")]
impl From<ParquetError> for RapidashError {
    fn from(e: ParquetError) -> Self {
        RapidashError::ParquetError(e)
    }
}

/// Schema-related errors
#[derive(Debug)]
pub enum SchemaError {
    /// Schema contains a (possibly) qualified and unqualified field with same unqualified name
    AmbiguousReference {
        qualifier: Option<String>,
        name: String,
    },
    /// Schema contains duplicate qualified field name
    DuplicateQualifiedField { qualifier: String, name: String },
    /// Schema contains duplicate unqualified field name
    DuplicateUnqualifiedField { name: String },
    /// No field with this name
    FieldNotFound {
        qualifier: Option<String>,
        name: String,
        valid_fields: Option<Vec<String>>,
    },
}

/// Create a "field not found" DataFusion::SchemaError
pub fn field_not_found(
    qualifier: Option<String>,
    name: &str,
    schema: &DFSchema,
) -> DataFusionError {
    DataFusionError::SchemaError(SchemaError::FieldNotFound {
        qualifier,
        name: name.to_string(),
        valid_fields: Some(schema.field_names()),
    })
}

impl Display for SchemaError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::FieldNotFound {
                qualifier,
                name,
                valid_fields,
            } => {
                write!(f, "No field named ")?;
                if let Some(q) = qualifier {
                    write!(f, "'{}.{}'", q, name)?;
                } else {
                    write!(f, "'{}'", name)?;
                }
                if let Some(field_names) = valid_fields {
                    write!(
                        f,
                        ". Valid fields are {}",
                        field_names
                            .iter()
                            .map(|name| format!("'{}'", name))
                            .collect::<Vec<String>>()
                            .join(", ")
                    )?;
                }
                write!(f, ".")
            }
            Self::DuplicateQualifiedField { qualifier, name } => {
                write!(
                    f,
                    "Schema contains duplicate qualified field name '{}.{}'",
                    qualifier, name
                )
            }
            Self::DuplicateUnqualifiedField { name } => {
                write!(
                    f,
                    "Schema contains duplicate unqualified field name '{}'",
                    name
                )
            }
            Self::AmbiguousReference { qualifier, name } => {
                if let Some(q) = qualifier {
                    write!(f, "Schema contains qualified field name '{}.{}' and unqualified field name '{}' which would be ambiguous", q, name, name)
                } else {
                    write!(f, "Ambiguous reference to unqualified field '{}'", name)
                }
            }
        }
    }
}
