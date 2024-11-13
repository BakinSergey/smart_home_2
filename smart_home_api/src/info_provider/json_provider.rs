use crate::info_provider::provider::{DeviceInfoProvider, IterableProvider};
use crate::my_smart_home::home::Home;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::{HashMap, HashSet};

#[derive(Debug, Serialize, Deserialize)]
pub struct JsonDeviceInfoProvider {
    /// маппинг: комната-устройства
    pub schema: HashMap<String, HashSet<String>>,
}

impl JsonDeviceInfoProvider {
    pub fn from_json(json: Value) -> serde_json::Result<JsonDeviceInfoProvider> {
        serde_json::from_value(json)
    }
}

impl DeviceInfoProvider for JsonDeviceInfoProvider {}

impl IterableProvider for JsonDeviceInfoProvider {
    fn as_set(&self) -> HashSet<String> {
        let mut device_set = HashSet::new();

        for (room, devices) in &self.schema {
            device_set.extend(devices.iter().map(|dev| Home::device_path(room, dev)));
        }
        device_set
    }
}
