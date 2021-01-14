mod get_dependencies;

use crate::errors::{RustKataError, RustKataResult};

struct CratesIoClient {
    base_url: String,
    client: reqwest::Client,
}

impl CratesIoClient {
    pub fn new(base_url: &str) -> RustKataResult<Self> {
        let mut headers = reqwest::header::HeaderMap::new();
        headers.insert(
            reqwest::header::USER_AGENT,
            reqwest::header::HeaderValue::from_static(
                "rust-kata-002 (https://github.com/agabani/rust-kata-002)",
            ),
        );

        let client = reqwest::Client::builder()
            .default_headers(headers)
            .build()
            .map_err(|_| RustKataError {})?;

        Ok(CratesIoClient {
            base_url: base_url.to_owned(),
            client,
        })
    }
}
