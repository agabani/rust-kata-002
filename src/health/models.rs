use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Deserialize, Serialize)]
pub(crate) struct Health {
    #[serde(rename = "status")]
    pub(crate) status: String,
    #[serde(skip_serializing_if = "Option::is_none", rename = "version")]
    pub(crate) version: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "releaseId")]
    pub(crate) release_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "notes")]
    pub(crate) notes: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "output")]
    pub(crate) output: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "checks")]
    pub(crate) checks: Option<HashMap<String, Vec<Check>>>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "links")]
    pub(crate) links: Option<HashMap<String, String>>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "serviceId")]
    pub(crate) service_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "description")]
    pub(crate) description: Option<String>,
}

#[derive(Deserialize, Serialize)]
pub(crate) struct Check {
    #[serde(skip_serializing_if = "Option::is_none", rename = "componentId")]
    pub(crate) component_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "componentType")]
    pub(crate) component_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "observedValue")]
    pub(crate) observed_value: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "observedUnit")]
    pub(crate) observed_unit: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "status")]
    pub(crate) status: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "affectedEndpoints")]
    pub(crate) affected_endpoints: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "time")]
    pub(crate) time: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "output")]
    pub(crate) output: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "links")]
    pub(crate) links: Option<HashMap<String, String>>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "additionalKeys")]
    pub(crate) additional_keys: Option<HashMap<String, String>>,
}
