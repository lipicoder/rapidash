//! Configuration option meta-data

use arrow_schema::DataType;
#[derive(Debug, Clone)]
pub struct ConfigEntry {
    pub(crate) name: String,
    pub(crate) _description: String,
    pub(crate) _data_type: DataType,
    pub(crate) default_value: Option<String>,
}

impl ConfigEntry {
    pub fn new(
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
