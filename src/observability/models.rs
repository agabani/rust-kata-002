use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Deserialize, Serialize)]
pub(crate) struct Health {
    #[serde(rename = "status")]
    status: String,
    #[serde(skip_serializing_if = "Option::is_none", rename = "version")]
    version: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "releaseId")]
    release_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "notes")]
    notes: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "output")]
    output: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "checks")]
    checks: Option<HashMap<String, Vec<Check>>>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "links")]
    links: Option<HashMap<String, String>>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "serviceId")]
    service_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "description")]
    description: Option<String>,
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

impl Health {
    pub fn envelope(checks: HashMap<String, Vec<Check>>) -> Self {
        Health {
            status: Health::status(&checks).to_owned(),
            version: Some(env!("CARGO_PKG_VERSION_MAJOR").to_owned()),
            release_id: Some(env!("CARGO_PKG_VERSION").to_owned()),
            notes: None,
            output: None,
            checks: Some(checks),
            links: None,
            service_id: None,
            description: Some("health of rust-kata-002 service".to_owned()),
        }
    }

    fn status(checks: &HashMap<String, Vec<Check>>) -> &str {
        let has_any_fail = checks.iter().any(|(_, checks)| {
            checks
                .iter()
                .any(|check| check.status == Some("fail".to_owned()))
        });
        if has_any_fail {
            return "fail";
        }

        let has_any_warn = checks.iter().any(|(_, checks)| {
            checks
                .iter()
                .any(|check| check.status == Some("warn".to_owned()))
        });
        if has_any_warn {
            return "warn";
        }

        "pass"
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[actix_rt::test]
    async fn test_envelope() {
        let mut checks = HashMap::new();
        checks
            .entry("key".to_owned())
            .or_insert(vec![])
            .push(Check {
                component_id: None,
                component_type: None,
                observed_value: None,
                observed_unit: None,
                status: None,
                affected_endpoints: None,
                time: None,
                output: None,
                links: None,
                additional_keys: None,
            });

        let health = Health::envelope(checks);

        assert_eq!(health.status, "pass");
        assert_eq!(health.version.unwrap(), "0");
        assert_eq!(health.release_id.unwrap(), "0.1.0");
        assert_eq!(health.notes, None);
        assert_eq!(health.output, None);
        assert_eq!(health.checks.unwrap().len(), 1);
        assert_eq!(health.links, None);
        assert_eq!(health.service_id, None);
        assert_eq!(
            health.description.unwrap(),
            "health of rust-kata-002 service"
        );
    }

    #[actix_rt::test]
    async fn test_envelope_pass() {
        let mut checks = HashMap::new();
        given_a_pass(&mut checks);

        let health = Health::envelope(checks);

        assert_eq!(health.status, "pass");
    }

    #[actix_rt::test]
    async fn test_envelope_pass_none() {
        let checks = HashMap::new();

        let health = Health::envelope(checks);

        assert_eq!(health.status, "pass");
    }

    #[actix_rt::test]
    async fn test_envelope_warn() {
        let mut checks = HashMap::new();
        given_a_warn(&mut checks);

        let health = Health::envelope(checks);

        assert_eq!(health.status, "warn");
    }

    #[actix_rt::test]
    async fn test_envelope_warn_multiple() {
        let mut checks = HashMap::new();
        given_a_pass(&mut checks);
        given_a_warn(&mut checks);
        given_a_pass(&mut checks);

        let health = Health::envelope(checks);

        assert_eq!(health.status, "warn");
    }

    #[actix_rt::test]
    async fn test_envelope_fail() {
        let mut checks = HashMap::new();
        given_a_fail(&mut checks);

        let health = Health::envelope(checks);

        assert_eq!(health.status, "fail");
    }

    #[actix_rt::test]
    async fn test_envelope_fail_multiple() {
        let mut checks = HashMap::new();
        given_a_pass(&mut checks);
        given_a_warn(&mut checks);
        given_a_fail(&mut checks);
        given_a_warn(&mut checks);
        given_a_pass(&mut checks);

        let health = Health::envelope(checks);

        assert_eq!(health.status, "fail");
    }

    fn given_a_pass(checks: &mut HashMap<String, Vec<Check>>) {
        checks.entry("p".to_owned()).or_insert(vec![]).push(Check {
            component_id: None,
            component_type: None,
            observed_value: None,
            observed_unit: None,
            status: Some("pass".to_owned()),
            affected_endpoints: None,
            time: None,
            output: None,
            links: None,
            additional_keys: None,
        });
    }

    fn given_a_warn(checks: &mut HashMap<String, Vec<Check>>) {
        checks.entry("w".to_owned()).or_insert(vec![]).push(Check {
            component_id: None,
            component_type: None,
            observed_value: None,
            observed_unit: None,
            status: Some("warn".to_owned()),
            affected_endpoints: None,
            time: None,
            output: None,
            links: None,
            additional_keys: None,
        });
    }

    fn given_a_fail(checks: &mut HashMap<String, Vec<Check>>) {
        checks.entry("f".to_owned()).or_insert(vec![]).push(Check {
            component_id: None,
            component_type: None,
            observed_value: None,
            observed_unit: None,
            status: Some("fail".to_owned()),
            affected_endpoints: None,
            time: None,
            output: None,
            links: None,
            additional_keys: None,
        });
    }
}
