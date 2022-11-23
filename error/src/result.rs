//! Error result.
//!
use crate::error::RapidashError;
use std::result as std_result;

pub type Result<T> = std_result::Result<T, RapidashError>;
