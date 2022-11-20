//! Rapidash engine configuration.

use crate::error::{RapidashError, Result};
use datafusion::arrow::datatypes::DataType;
use std::collections::HashMap;
use std::result;

pub const RAPIDASH_JOB_NAME: &str = "RAPIDASH.job.name";
pub const RAPIDASH_DEFAULT_SHUFFLE_PARTITIONS: &str = "RAPIDASH.shuffle.partitions";
pub const RAPIDASH_DEFAULT_BATCH_SIZE: &str = "RAPIDASH.batch.size";
pub const RAPIDASH_REPARTITION_JOINS: &str = "RAPIDASH.repartition.joins";
pub const RAPIDASH_REPARTITION_AGGREGATIONS: &str = "RAPIDASH.repartition.aggregations";
pub const RAPIDASH_REPARTITION_WINDOWS: &str = "RAPIDASH.repartition.windows";
pub const RAPIDASH_PARQUET_PRUNING: &str = "RAPIDASH.parquet.pruning";
pub const RAPIDASH_WITH_INFORMATION_SCHEMA: &str = "RAPIDASH.with_information_schema";

pub type ParseResult<T> = result::Result<T, String>;

/// Configuration option meta-data
#[derive(Debug, Clone)]
pub struct ConfigEntry {
    name: String,
    _description: String,
    _data_type: DataType,
    default_value: Option<String>,
}

impl ConfigEntry {
    fn new(
        name: String,
        _description: String,
        _data_type: DataType,
        default_value: Option<String>,
    ) -> Self {
        Self {
            name,
            _description,
            _data_type,
            default_value,
        }
    }
}

/// Configuration builder
pub struct ConfigBuilder {
    settings: HashMap<String, String>,
}

impl Default for ConfigBuilder {
    /// Create a new config builder
    fn default() -> Self {
        Self {
            settings: HashMap::new(),
        }
    }
}

impl ConfigBuilder {
    /// Create a new config with an additional setting
    pub fn set(&self, k: &str, v: &str) -> Self {
        let mut settings = self.settings.clone();
        settings.insert(k.to_owned(), v.to_owned());
        Self { settings }
    }

    pub fn build(&self) -> Result<Config> {
        Config::with_settings(self.settings.clone())
    }
}

/// Rapidash configuration
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Config {
    /// Settings stored in map for easy serde
    settings: HashMap<String, String>,
}

impl Config {
    /// Create a default configuration
    pub fn new() -> Result<Self> {
        Self::with_settings(HashMap::new())
    }

    /// Create a configuration builder
    pub fn builder() -> ConfigBuilder {
        ConfigBuilder::default()
    }

    /// Create a new configuration based on key-value pairs
    pub fn with_settings(settings: HashMap<String, String>) -> Result<Self> {
        let supported_entries = Config::valid_entries();
        for (name, entry) in &supported_entries {
            if let Some(v) = settings.get(name) {
                // validate that we can parse the user-supplied value
                Self::parse_value(v.as_str(), entry._data_type.clone()).map_err(|e| RapidashError::General(format!("Failed to parse user-supplied value '{}' for configuration setting '{}': {}", name, v, e)))?;
            } else if let Some(v) = entry.default_value.clone() {
                Self::parse_value(v.as_str(), entry._data_type.clone()).map_err(|e| {
                    RapidashError::General(format!(
                        "Failed to parse default value '{}' for configuration setting '{}': {}",
                        name, v, e
                    ))
                })?;
            } else if entry.default_value.is_none() {
                // optional config
            } else {
                return Err(RapidashError::General(format!(
                    "No value specified for mandatory configuration setting '{}'",
                    name
                )));
            }
        }

        Ok(Self { settings })
    }

    pub fn settings(&self) -> &HashMap<String, String> {
        &self.settings
    }

    pub fn parse_value(val: &str, data_type: DataType) -> ParseResult<()> {
        match data_type {
            DataType::UInt16 => {
                val.to_string()
                    .parse::<usize>()
                    .map_err(|e| format!("{:?}", e))?;
            }
            DataType::Boolean => {
                val.to_string()
                    .parse::<bool>()
                    .map_err(|e| format!("{:?}", e))?;
            }
            DataType::Utf8 => {
                val.to_string();
            }
            _ => {
                return Err(format!("not support data type: {}", data_type));
            }
        }

        Ok(())
    }

    /// All available configuration options
    pub fn valid_entries() -> HashMap<String, ConfigEntry> {
        let entries = vec![
            ConfigEntry::new(RAPIDASH_JOB_NAME.to_string(),
                             "Sets the job name that will appear in the web user interface for any submitted jobs".to_string(),
                             DataType::Utf8, None),
            ConfigEntry::new(RAPIDASH_DEFAULT_SHUFFLE_PARTITIONS.to_string(),
                             "Sets the default number of partitions to create when repartitioning query stages".to_string(),
                             DataType::UInt16, Some("16".to_string())),
            ConfigEntry::new(RAPIDASH_DEFAULT_BATCH_SIZE.to_string(),
                             "Sets the default batch size".to_string(),
                             DataType::UInt16, Some("8192".to_string())),
            ConfigEntry::new(RAPIDASH_REPARTITION_JOINS.to_string(),
                             "Configuration for repartition joins".to_string(),
                             DataType::Boolean, Some("true".to_string())),
            ConfigEntry::new(RAPIDASH_REPARTITION_AGGREGATIONS.to_string(),
                             "Configuration for repartition aggregations".to_string(),
                             DataType::Boolean, Some("true".to_string())),
            ConfigEntry::new(RAPIDASH_REPARTITION_WINDOWS.to_string(),
                             "Configuration for repartition windows".to_string(),
                             DataType::Boolean, Some("true".to_string())),
            ConfigEntry::new(RAPIDASH_PARQUET_PRUNING.to_string(),
                             "Configuration for parquet prune".to_string(),
                             DataType::Boolean, Some("true".to_string())),
            ConfigEntry::new(RAPIDASH_WITH_INFORMATION_SCHEMA.to_string(),
                             "Sets whether enable information_schema".to_string(),
                             DataType::Boolean, Some("false".to_string())),
        ];
        entries
            .iter()
            .map(|e| (e.name.clone(), e.clone()))
            .collect::<HashMap<_, _>>()
    }

    pub fn default_shuffle_partitions(&self) -> usize {
        self.get_usize_setting(RAPIDASH_DEFAULT_SHUFFLE_PARTITIONS)
    }

    pub fn default_batch_size(&self) -> usize {
        self.get_usize_setting(RAPIDASH_DEFAULT_BATCH_SIZE)
    }

    pub fn repartition_joins(&self) -> bool {
        self.get_bool_setting(RAPIDASH_REPARTITION_JOINS)
    }

    pub fn repartition_aggregations(&self) -> bool {
        self.get_bool_setting(RAPIDASH_REPARTITION_AGGREGATIONS)
    }

    pub fn repartition_windows(&self) -> bool {
        self.get_bool_setting(RAPIDASH_REPARTITION_WINDOWS)
    }

    pub fn parquet_pruning(&self) -> bool {
        self.get_bool_setting(RAPIDASH_PARQUET_PRUNING)
    }

    pub fn default_with_information_schema(&self) -> bool {
        self.get_bool_setting(RAPIDASH_WITH_INFORMATION_SCHEMA)
    }

    fn get_usize_setting(&self, key: &str) -> usize {
        if let Some(v) = self.settings.get(key) {
            // infallible because we validate all configs in the constructor
            v.parse().unwrap()
        } else {
            let entries = Self::valid_entries();
            // infallible because we validate all configs in the constructor
            let v = entries.get(key).unwrap().default_value.as_ref().unwrap();
            v.parse().unwrap()
        }
    }

    fn get_bool_setting(&self, key: &str) -> bool {
        if let Some(v) = self.settings.get(key) {
            // infallible because we validate all configs in the constructor
            v.parse::<bool>().unwrap()
        } else {
            let entries = Self::valid_entries();
            // infallible because we validate all configs in the constructor
            let v = entries.get(key).unwrap().default_value.as_ref().unwrap();
            v.parse::<bool>().unwrap()
        }
    }
    fn get_string_setting(&self, key: &str) -> String {
        if let Some(v) = self.settings.get(key) {
            // infallible because we validate all configs in the constructor
            v.to_string()
        } else {
            let entries = Self::valid_entries();
            // infallible because we validate all configs in the constructor
            let v = entries.get(key).unwrap().default_value.as_ref().unwrap();
            v.to_string()
        }
    }
}
