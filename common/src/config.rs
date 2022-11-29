//! Configuration for the `config` crate.
use std::collections::HashMap;

use arrow_schema::DataType;
use error::{RapidashError, Result};

use crate::builder::ConfigBuilder;
use crate::builder::ParseResult;
use crate::entry::ConfigEntry;

pub const JOB_NAME: &str = "rapidash.job.name";
pub const DEFAULT_BATCH_SIZE: &str = "rapidash.batch.size";

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
            ConfigEntry::new(JOB_NAME.to_string(),
                             "Sets the job name that will appear in the web user interface for any submitted jobs".to_string(),
                             DataType::Utf8, None),
            ConfigEntry::new(DEFAULT_BATCH_SIZE.to_string(),
                             "Sets the default batch size".to_string(),
                             DataType::UInt16, Some("8192".to_string())),
        ];
        entries
            .iter()
            .map(|e| (e.name.clone(), e.clone()))
            .collect::<HashMap<_, _>>()
    }

    pub fn settings(&self) -> &HashMap<String, String> {
        &self.settings
    }

    pub fn default_batch_size(&self) -> usize {
        self.get_usize_setting(DEFAULT_BATCH_SIZE)
    }

    pub fn get_usize_setting(&self, key: &str) -> usize {
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

    pub fn get_bool_setting(&self, key: &str) -> bool {
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

    pub fn get_string_setting(&self, key: &str) -> String {
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


/// Configuration options struct. This can contain values for built-in and custom options
#[derive(Clone)]
pub struct ConfigOptions {
    options: HashMap<String, ScalarValue>,
}

/// Print the configurations in an ordered way so that we can directly compare the equality of two ConfigOptions by their debug strings
impl Debug for ConfigOptions {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ConfigOptions")
            .field(
                "options",
                &format!("{:?}", BTreeMap::from_iter(self.options.iter())),
            )
            .finish()
    }
}

impl Default for ConfigOptions {
    fn default() -> Self {
        Self::new()
    }
}

impl ConfigOptions {
    /// Create new ConfigOptions struct
    pub fn new() -> Self {
        let built_in = BuiltInConfigs::new();
        let mut options = HashMap::with_capacity(built_in.config_definitions.len());
        for config_def in &built_in.config_definitions {
            options.insert(config_def.key.clone(), config_def.default_value.clone());
        }
        Self { options }
    }

    /// Create a new [`ConfigOptions`] wrapped in an RwLock and Arc
    pub fn into_shareable(self) -> Arc<RwLock<Self>> {
        Arc::new(RwLock::new(self))
    }

    /// Create new ConfigOptions struct, taking values from
    /// environment variables where possible.
    ///
    /// For example, setting `DATAFUSION_EXECUTION_BATCH_SIZE` will
    /// control `datafusion.execution.batch_size`.
    pub fn from_env() -> Self {
        let built_in = BuiltInConfigs::new();
        let mut options = HashMap::with_capacity(built_in.config_definitions.len());
        for config_def in &built_in.config_definitions {
            let config_value = {
                let mut env_key = config_def.key.replace('.', "_");
                env_key.make_ascii_uppercase();
                match env::var(&env_key) {
                    Ok(value) => match ScalarValue::try_from_string(
                        value.clone(),
                        &config_def.data_type,
                    ) {
                        Ok(parsed) => parsed,
                        Err(_) => {
                            warn!("Warning: could not parse environment variable {}={} to type {}.", env_key, value, config_def.data_type);
                            config_def.default_value.clone()
                        }
                    },
                    Err(_) => config_def.default_value.clone(),
                }
            };
            options.insert(config_def.key.clone(), config_value);
        }
        Self { options }
    }

    /// set a configuration option
    pub fn set(&mut self, key: &str, value: ScalarValue) {
        self.options.insert(key.to_string(), value);
    }

    /// set a boolean configuration option
    pub fn set_bool(&mut self, key: &str, value: bool) {
        self.set(key, ScalarValue::Boolean(Some(value)))
    }

    /// set a `u64` configuration option
    pub fn set_u64(&mut self, key: &str, value: u64) {
        self.set(key, ScalarValue::UInt64(Some(value)))
    }

    /// set a `String` configuration option
    pub fn set_string(&mut self, key: &str, value: impl Into<String>) {
        self.set(key, ScalarValue::Utf8(Some(value.into())))
    }

    /// get a configuration option
    pub fn get(&self, key: &str) -> Option<ScalarValue> {
        self.options.get(key).cloned()
    }

    /// get a boolean configuration option
    pub fn get_bool(&self, key: &str) -> Option<bool> {
        get_conf_value!(self, Boolean, key, "bool")
    }

    /// get a u64 configuration option
    pub fn get_u64(&self, key: &str) -> Option<u64> {
        get_conf_value!(self, UInt64, key, "u64")
    }

    /// get a string configuration option
    pub fn get_string(&self, key: &str) -> Option<String> {
        get_conf_value!(self, Utf8, key, "string")
    }

    /// Access the underlying hashmap
    pub fn options(&self) -> &HashMap<String, ScalarValue> {
        &self.options
    }

    /// Tests if the key exists in the configuration
    pub fn exists(&self, key: &str) -> bool {
        self.options().contains_key(key)
    }
}
