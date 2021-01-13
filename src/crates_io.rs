use crate::errors::{RustKataError, RustKataResult};
use actix_web::http::StatusCode;
use serde::{Deserialize, Serialize};

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

    pub async fn get_dependencies(
        &self,
        crate_name: &str,
        crate_version: &str,
    ) -> RustKataResult<CrateDependenciesResponse> {
        let string = format!(
            "{}/api/v1/crates/{}/{}/dependencies",
            self.base_url, crate_name, crate_version
        );

        let response = self.client.get(&string).send().await.unwrap();

        if response.status() != StatusCode::OK {
            return Err(RustKataError {});
        }

        let json = response
            .json::<CrateDependenciesResponse>()
            .await
            .map_err(|_| RustKataError {})?;

        Ok(json)
    }
}

#[derive(Debug, Deserialize, Serialize)]
struct CrateDependenciesResponse {
    #[serde(rename = "dependencies")]
    dependencies: Vec<CrateDependencyResponse>,
}

#[derive(Debug, Deserialize, Serialize)]
struct CrateDependencyResponse {
    #[serde(rename = "id")]
    id: i32,
    #[serde(rename = "version_id")]
    version_id: i32,
    #[serde(rename = "crate_id")]
    crate_id: String,
    #[serde(rename = "req")]
    req: String,
    #[serde(rename = "optional")]
    optional: bool,
    #[serde(rename = "default_features")]
    default_features: bool,
    #[serde(rename = "features")]
    features: Option<Vec<String>>,
    #[serde(rename = "target")]
    target: Option<String>,
    #[serde(rename = "kind")]
    kind: String,
    #[serde(rename = "downloads")]
    downloads: i32,
}

#[cfg(test)]
mod tests {
    use super::*;
    use mockito::mock;

    #[actix_rt::test]
    async fn get_dependencies() {
        let mock = mock("GET", "/api/v1/crates/rand/0.8.2/dependencies")
            .with_status(200)
            .with_body(
                r#"
{
    "dependencies": [
        {
            "id": 2012432,
            "version_id": 326822,
            "crate_id": "bincode",
            "req": "^1.2.1",
            "optional": false,
            "default_features": true,
            "features": [],
            "target": null,
            "kind": "dev",
            "downloads": 0
        },
        {
            "id": 2012430,
            "version_id": 326822,
            "crate_id": "rand_core",
            "req": "^0.6.0",
            "optional": false,
            "default_features": true,
            "features": [],
            "target": null,
            "kind": "normal",
            "downloads": 0
        },
        {
            "id": 2012433,
            "version_id": 326822,
            "crate_id": "rand_hc",
            "req": "^0.3.0",
            "optional": false,
            "default_features": true,
            "features": [],
            "target": null,
            "kind": "dev",
            "downloads": 0
        },
        {
            "id": 2012434,
            "version_id": 326822,
            "crate_id": "rand_pcg",
            "req": "^0.3.0",
            "optional": false,
            "default_features": true,
            "features": [],
            "target": null,
            "kind": "dev",
            "downloads": 0
        },
        {
            "id": 2012437,
            "version_id": 326822,
            "crate_id": "libc",
            "req": "^0.2.22",
            "optional": true,
            "default_features": false,
            "features": [],
            "target": "cfg(unix)",
            "kind": "normal",
            "downloads": 0
        },
        {
            "id": 2012428,
            "version_id": 326822,
            "crate_id": "log",
            "req": "^0.4.4",
            "optional": true,
            "default_features": true,
            "features": [],
            "target": null,
            "kind": "normal",
            "downloads": 0
        },
        {
            "id": 2012429,
            "version_id": 326822,
            "crate_id": "packed_simd_2",
            "req": "^0.3.4",
            "optional": true,
            "default_features": true,
            "features": [
                "into_bits"
            ],
            "target": null,
            "kind": "normal",
            "downloads": 0
        },
        {
            "id": 2012435,
            "version_id": 326822,
            "crate_id": "rand_chacha",
            "req": "^0.3.0",
            "optional": true,
            "default_features": false,
            "features": [],
            "target": "cfg(not(target_os = \"emscripten\"))",
            "kind": "normal",
            "downloads": 0
        },
        {
            "id": 2012436,
            "version_id": 326822,
            "crate_id": "rand_hc",
            "req": "^0.3.0",
            "optional": true,
            "default_features": true,
            "features": [],
            "target": "cfg(target_os = \"emscripten\")",
            "kind": "normal",
            "downloads": 0
        },
        {
            "id": 2012431,
            "version_id": 326822,
            "crate_id": "serde",
            "req": "^1.0.103",
            "optional": true,
            "default_features": true,
            "features": [
                "derive"
            ],
            "target": null,
            "kind": "normal",
            "downloads": 0
        }
    ]
}
            "#,
            )
            .with_header("content-type", "application/json; charset=utf-8")
            .match_header(
                "user-agent",
                "rust-kata-002 (https://github.com/agabani/rust-kata-002)",
            )
            .create();

        let client = CratesIoClient::new(&mockito::server_url()).unwrap();
        let response = client.get_dependencies(&"rand", &"0.8.2").await.unwrap();

        assert_eq!(response.dependencies.len(), 10usize);

        assert_eq!(response.dependencies[0].id, 2012432);
        assert_eq!(response.dependencies[0].version_id, 326822);
        assert_eq!(response.dependencies[0].crate_id, "bincode");
        assert_eq!(response.dependencies[0].req, "^1.2.1");
        assert_eq!(response.dependencies[0].optional, false);
        assert_eq!(response.dependencies[0].default_features, true);
        assert_eq!(
            response.dependencies[0].features,
            Some(Vec::<String>::new())
        );
        assert_eq!(response.dependencies[0].target, None);
        assert_eq!(response.dependencies[0].kind, "dev");
        assert_eq!(response.dependencies[0].downloads, 0);

        assert_eq!(response.dependencies[9].id, 2012431);
        assert_eq!(response.dependencies[9].version_id, 326822);
        assert_eq!(response.dependencies[9].crate_id, "serde");
        assert_eq!(response.dependencies[9].req, "^1.0.103");
        assert_eq!(response.dependencies[9].optional, true);
        assert_eq!(response.dependencies[9].default_features, true);
        assert_eq!(
            response.dependencies[9].features,
            Some(vec!["derive".to_owned()])
        );
        assert_eq!(response.dependencies[9].target, None);
        assert_eq!(response.dependencies[9].kind, "normal");
        assert_eq!(response.dependencies[9].downloads, 0);

        mock.assert();
    }
}
