mod get_crate;
mod get_crate_dependencies;

use crate::errors::RustKataResult;

pub struct CratesIoClient {
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
            .unwrap();

        Ok(CratesIoClient {
            base_url: base_url.to_owned(),
            client,
        })
    }
}
