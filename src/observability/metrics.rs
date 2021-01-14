use actix_web::http::StatusCode;
use prometheus::{Histogram, HistogramVec, IntCounterVec};

const ENDPOINT: &str = "endpoint";
const STATUS_CODE: &str = "status_code";

lazy_static! {
    static ref HTTP_REQUSET_COUNTER: IntCounterVec =
        register_int_counter_vec!("http_request_count", "http request count", &[ENDPOINT]).unwrap();
    static ref HTTP_RESPONSE_COUNTER: IntCounterVec = register_int_counter_vec!(
        "http_response_count",
        "http response count",
        &[ENDPOINT, STATUS_CODE]
    )
    .unwrap();
    static ref HTTPS_RESPONSE_DURATION: HistogramVec = register_histogram_vec!(
        "http_response_duration",
        "http response duration",
        &[ENDPOINT, STATUS_CODE]
    )
    .unwrap();
}

pub fn http_request_counter(endpoint: &str) {
    HTTP_REQUSET_COUNTER.with_label_values(&[endpoint]).inc()
}

pub fn http_response_count(endpoint: &str, status_code: &StatusCode) {
    HTTP_RESPONSE_COUNTER
        .with_label_values(&[endpoint, status_code.as_str()])
        .inc()
}

pub fn http_response_duration(endpoint: &str, status_code: &StatusCode) -> Histogram {
    HTTPS_RESPONSE_DURATION.with_label_values(&[endpoint, status_code.as_str()])
}
