use crate::health::models::Check;
use chrono::{DateTime, SecondsFormat, Utc};
use std::time::Instant;

pub(crate) fn uptime_checker(now: &DateTime<Utc>, application_start: &Instant) -> Check {
    Check {
        component_id: None,
        component_type: Some("system".to_owned()),
        observed_value: Some(application_start.elapsed().as_secs_f32().to_string()),
        observed_unit: Some("s".to_owned()),
        status: Some("pass".to_owned()),
        affected_endpoints: None,
        time: Some(now.to_rfc3339_opts(SecondsFormat::Secs, true)),
        output: None,
        links: None,
        additional_keys: None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::str::FromStr;

    #[actix_rt::test]
    async fn test_uptime_checker() {
        let application_start =
            DateTime::<Utc>::from_str("2018-01-17T03:36:48Z").expect("valid date time");
        let instant = Instant::now();

        let check = uptime_checker(&application_start, &instant);

        assert_eq!(check.component_id, None);
        assert_eq!(check.component_type, Some("system".to_owned()));
        assert!(
            check
                .observed_value
                .expect("value")
                .parse::<f32>()
                .expect("f32")
                > 0f32
        );
        assert_eq!(check.observed_unit, Some("s".to_owned()));
        assert_eq!(check.status, Some("pass".to_owned()));
        assert_eq!(check.affected_endpoints, None);
        assert_eq!(check.time, Some("2018-01-17T03:36:48Z".to_owned()));
        assert_eq!(check.output, None);
        assert_eq!(check.links, None);
        assert_eq!(check.additional_keys, None);
    }
}
