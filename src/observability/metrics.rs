use actix_web::http::StatusCode;
use prometheus::{Histogram, HistogramVec, IntCounterVec};

const BASE_URL: &str = "base_url";
const ENDPOINT: &str = "endpoint";
const STATUS_CODE: &str = "status_code";

pub fn api_request_duration_seconds(
    base_url: &str,
    endpoint: &str,
    status_code: &StatusCode,
) -> Histogram {
    lazy_static! {
        static ref METRIC: HistogramVec = register_histogram_vec!(
            "api_request_duration_seconds",
            "api request duration seconds",
            &[BASE_URL, ENDPOINT, STATUS_CODE]
        )
        .unwrap();
    }

    METRIC.with_label_values(&[base_url, endpoint, status_code.as_str()])
}

pub fn http_request_count(endpoint: &str) {
    lazy_static! {
        static ref METRIC: IntCounterVec =
            register_int_counter_vec!("http_request_count", "http request count", &[ENDPOINT])
                .unwrap();
    }

    METRIC.with_label_values(&[endpoint]).inc()
}

pub fn http_response_count(endpoint: &str, status_code: &StatusCode) {
    lazy_static! {
        static ref METRIC: IntCounterVec = register_int_counter_vec!(
            "http_response_count",
            "http response count",
            &[ENDPOINT, STATUS_CODE]
        )
        .unwrap();
    }

    METRIC
        .with_label_values(&[endpoint, status_code.as_str()])
        .inc()
}

pub fn http_response_duration_seconds(endpoint: &str, status_code: &StatusCode) -> Histogram {
    lazy_static! {
        static ref METRIC: HistogramVec = register_histogram_vec!(
            "http_response_duration_seconds",
            "http response duration seconds",
            &[ENDPOINT, STATUS_CODE]
        )
        .unwrap();
    }

    METRIC.with_label_values(&[endpoint, status_code.as_str()])
}
