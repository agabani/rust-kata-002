use crate::errors::{RustKataError, RustKataResult};
use crate::observability::metrics;
use crate::traits::CrateRegistry;
use async_trait::async_trait;
use reqwest::StatusCode;
use std::time::Instant;

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

    async fn get<T: for<'de> serde::Deserialize<'de>>(
        &self,
        path: &str,
        endpoint: &str,
    ) -> RustKataResult<T> {
        let url = format!("{}{}", self.base_url, path);

        let instant = Instant::now();

        let response = self.client.get(&url).send().await.unwrap();

        let duration = instant.elapsed();

        metrics::api_request_duration_seconds(&self.base_url, endpoint, &response.status())
            .observe(duration.as_secs_f64());

        if response.status() != StatusCode::OK {
            return Err(RustKataError {});
        }

        let response = response.json().await.unwrap();

        Ok(response)
    }
}

#[async_trait]
impl CrateRegistry for CratesIoClient {
    async fn get_crate(
        &self,
        crate_name: &str,
    ) -> RustKataResult<crate::traits::get_crate::Response> {
        let path = format!("/api/v1/crates/{}", crate_name);

        self.get(&path, "get_crate").await
    }

    async fn get_crate_dependencies(
        &self,
        crate_name: &str,
        crate_version: &str,
    ) -> RustKataResult<crate::traits::get_crate_dependencies::Response> {
        let path = format!(
            "/api/v1/crates/{}/{}/dependencies",
            crate_name, crate_version
        );

        self.get(&path, "get_crate_dependencies").await
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use mockito::mock;

    mod get_crate {
        use super::*;
        use crate::traits::get_crate;

        #[actix_rt::test]
        async fn test() {
            let response = theory_get_crates("rand", "get_crates_rand.json").await;

            assert_eq!(response.crate_.id, "rand");
            assert_eq!(response.crate_.name, "rand");
            assert_eq!(
                response.crate_.updated_at,
                "2021-01-13T09:55:20.922296+00:00"
            );
            assert_eq!(response.crate_.versions.len(), 65);
            assert_eq!(response.crate_.versions[0], 326822);
            assert_eq!(response.crate_.versions[64], 4362);
            assert_eq!(response.crate_.keywords, vec!["random", "rng"]);
            assert_eq!(
                response.crate_.links.version_downloads,
                "/api/v1/crates/rand/downloads"
            );
            assert_eq!(response.crate_.links.versions, None);
            assert_eq!(response.crate_.exact_match, false);

            assert_eq!(response.versions.len(), 65);
            assert_eq!(response.versions[0].id, 326822);
            assert_eq!(response.versions[0].crate_, "rand");
            assert_eq!(response.versions[64].id, 4362);
            assert_eq!(response.versions[64].yanked, false);

            assert_eq!(response.keywords.len(), 2);
            assert_eq!(response.keywords[0].id, "random");
            assert_eq!(response.keywords[0].keyword, "random");
            assert_eq!(
                response.keywords[0].created_at,
                "2014-11-21T00:22:50.038243+00:00"
            );
            assert_eq!(response.keywords[0].crates_cnt, 171);
            assert_eq!(response.keywords[1].id, "rng");
            assert_eq!(response.keywords[1].keyword, "rng");
            assert_eq!(
                response.keywords[1].created_at,
                "2015-02-02T03:37:04.452064+00:00"
            );
            assert_eq!(response.keywords[1].crates_cnt, 58);

            assert_eq!(response.categories.len(), 2);
            assert_eq!(response.categories[0].id, "no-std");
            assert_eq!(response.categories[0].category, "No standard library");
            assert_eq!(response.categories[0].slug, "no-std");
            assert_eq!(
                response.categories[0].description,
                "Crates that are able to function without the Rust standard library.\n"
            );
            assert_eq!(
                response.categories[0].created_at,
                "2017-02-10T01:52:09.447906+00:00"
            );
            assert_eq!(response.categories[0].crates_cnt, 2300);

            assert_eq!(response.categories[1].id, "algorithms");
            assert_eq!(response.categories[1].category, "Algorithms");
            assert_eq!(response.categories[1].slug, "algorithms");
            assert_eq!(response.categories[1].description,  "Rust implementations of core algorithms such as hashing, sorting, searching, and more.");
            assert_eq!(
                response.categories[1].created_at,
                "2017-01-17T19:13:05.112025+00:00"
            );
            assert_eq!(response.categories[1].crates_cnt, 999);
        }

        #[actix_rt::test]
        async fn test_actix_web() {
            theory_get_crates("actix-web", "get_crates_actix_web.json").await;
        }

        #[actix_rt::test]
        async fn test_funny() {
            theory_get_crates("funny", "get_crates_funny.json").await;
        }

        #[actix_rt::test]
        async fn test_serde() {
            theory_get_crates("serde", "get_crates_serde.json").await;
        }

        #[actix_rt::test]
        async fn test_syn() {
            theory_get_crates("syn", "get_crates_syn.json").await;
        }

        #[actix_rt::test]
        async fn test_tokio() {
            theory_get_crates("tokio", "get_crates_tokio.json").await;
        }

        async fn theory_get_crates(
            crate_name: &str,
            test_fixture_file: &str,
        ) -> get_crate::Response {
            let path = format!("/api/v1/crates/{}", crate_name);

            let mock = mock("GET", path.as_str())
                .with_status(200)
                .with_header("content-type", "application/json; charset=utf-8")
                .with_body(
                    std::fs::read_to_string(format!("./tests/fixtures/{}", test_fixture_file))
                        .unwrap(),
                )
                .match_header(
                    "user-agent",
                    "rust-kata-002 (https://github.com/agabani/rust-kata-002)",
                )
                .create();

            let client = CratesIoClient::new(&mockito::server_url()).unwrap();

            let response = client.get_crate(crate_name).await.unwrap();

            mock.assert();

            response
        }
    }

    mod get_crate_dependencies {
        use super::*;
        use crate::traits::get_crate_dependencies;

        #[actix_rt::test]
        async fn test() {
            let response = theory("rand", "0.8.2", "get_crate_dependencies_rand_0.8.2.json").await;

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
        }

        async fn theory(
            crate_name: &str,
            crate_version: &str,
            test_fixture_file: &str,
        ) -> get_crate_dependencies::Response {
            let path = format!(
                "/api/v1/crates/{}/{}/dependencies",
                crate_name, crate_version
            );

            let mock = mock("GET", path.as_str())
                .with_status(200)
                .with_header("content-type", "application/json; charset=utf-8")
                .with_body(
                    std::fs::read_to_string(format!("./tests/fixtures/{}", test_fixture_file))
                        .unwrap(),
                )
                .match_header(
                    "user-agent",
                    "rust-kata-002 (https://github.com/agabani/rust-kata-002)",
                )
                .create();

            let client = CratesIoClient::new(&mockito::server_url()).unwrap();
            let response = client
                .get_crate_dependencies(&"rand", &"0.8.2")
                .await
                .unwrap();

            mock.assert();

            response
        }
    }
}
