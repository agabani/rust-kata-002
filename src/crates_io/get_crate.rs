use crate::crates_io::CratesIoClient;
use crate::errors::{RustKataError, RustKataResult};
use crate::observability::metrics;
use reqwest::StatusCode;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::time::Instant;

impl CratesIoClient {
    pub async fn get_crate(&self, crate_name: &str) -> RustKataResult<Response> {
        let url = format!("{}/api/v1/crates/{}", self.base_url, crate_name);

        let instant = Instant::now();

        let response = self.client.get(&url).send().await.unwrap();

        let duration = instant.elapsed();

        metrics::api_request_duration_seconds(&self.base_url, "get_crates", &response.status())
            .observe(duration.as_secs_f64());

        if response.status() != StatusCode::OK {
            return Err(RustKataError {});
        }

        let response = response.json().await.unwrap();

        Ok(response)
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Response {
    #[serde(rename = "crate")]
    crate_: CrateResponse,
    #[serde(rename = "versions")]
    versions: Vec<VersionResponse>,
    #[serde(rename = "keywords")]
    keywords: Vec<KeywordResponse>,
    #[serde(rename = "categories")]
    categories: Vec<CategoryResponse>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct CrateResponse {
    #[serde(rename = "id")]
    id: String,
    #[serde(rename = "name")]
    name: String,
    #[serde(rename = "updated_at")]
    updated_at: String,
    #[serde(rename = "versions")]
    versions: Vec<i64>,
    #[serde(rename = "keywords")]
    keywords: Vec<String>,
    #[serde(rename = "categories")]
    categories: Vec<String>,
    #[serde(rename = "badges")]
    badges: Vec<CrateBadgeResponse>,
    #[serde(rename = "created_at")]
    created_at: String,
    #[serde(rename = "downloads")]
    downloads: i64,
    #[serde(rename = "recent_downloads")]
    recent_downloads: i64,
    #[serde(rename = "max_version")]
    max_version: String,
    #[serde(rename = "newest_version")]
    newest_version: String,
    #[serde(rename = "description")]
    description: String,
    #[serde(rename = "homepage")]
    homepage: Option<String>,
    #[serde(rename = "documentation")]
    documentation: Option<String>,
    #[serde(rename = "repository")]
    repository: String,
    #[serde(rename = "links")]
    links: CrateLinksResponse,
    #[serde(rename = "exact_match")]
    exact_match: bool,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct CrateBadgeResponse {
    #[serde(rename = "badge_type")]
    badge_type: String,
    #[serde(rename = "attributes")]
    attributes: CrateBadgeAttributesResponse,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct CrateBadgeAttributesResponse {
    #[serde(rename = "service")]
    service: Option<String>,
    #[serde(rename = "repository")]
    repository: String,
    #[serde(rename = "branch")]
    branch: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct CrateLinksResponse {
    #[serde(rename = "version_downloads")]
    version_downloads: String,
    #[serde(rename = "versions")]
    versions: Option<String>,
    #[serde(rename = "owners")]
    owners: String,
    #[serde(rename = "owner_team")]
    owner_team: String,
    #[serde(rename = "owner_user")]
    owner_user: String,
    #[serde(rename = "reverse_dependencies")]
    reverse_dependencies: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct VersionResponse {
    #[serde(rename = "id")]
    id: i64,
    #[serde(rename = "crate")]
    crate_: String,
    #[serde(rename = "num")]
    num: String,
    #[serde(rename = "dl_path")]
    dl_path: String,
    #[serde(rename = "readme_path")]
    readme_path: String,
    #[serde(rename = "updated_at")]
    updated_at: String,
    #[serde(rename = "created_at")]
    created_at: String,
    #[serde(rename = "downloads")]
    downloads: i64,
    #[serde(rename = "features")]
    features: HashMap<String, Vec<String>>,
    #[serde(rename = "yanked")]
    yanked: bool,
    #[serde(rename = "license")]
    license: String,
    #[serde(rename = "links")]
    links: VersionLinksResponse,
    #[serde(rename = "crate_size")]
    crate_size: Option<i64>,
    #[serde(rename = "published_by")]
    published_by: Option<UserResponse>,
    #[serde(rename = "audit_actions")]
    audit_actions: Vec<VersionAuditActionResponse>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct VersionLinksResponse {
    #[serde(rename = "dependencies")]
    dependencies: String,
    #[serde(rename = "version_downloads")]
    version_downloads: String,
    #[serde(rename = "authors")]
    authors: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct VersionAuditActionResponse {
    #[serde(rename = "action")]
    action: String,
    #[serde(rename = "user")]
    user: UserResponse,
    #[serde(rename = "time")]
    time: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct UserResponse {
    #[serde(rename = "id")]
    id: i64,
    #[serde(rename = "login")]
    login: String,
    #[serde(rename = "name")]
    name: Option<String>,
    #[serde(rename = "avatar")]
    avatar: String,
    #[serde(rename = "url")]
    url: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct KeywordResponse {
    #[serde(rename = "id")]
    id: String,
    #[serde(rename = "keyword")]
    keyword: String,
    #[serde(rename = "created_at")]
    created_at: String,
    #[serde(rename = "crates_cnt")]
    crates_cnt: i64,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct CategoryResponse {
    #[serde(rename = "id")]
    id: String,
    #[serde(rename = "category")]
    category: String,
    #[serde(rename = "slug")]
    slug: String,
    #[serde(rename = "description")]
    description: String,
    #[serde(rename = "created_at")]
    created_at: String,
    #[serde(rename = "crates_cnt")]
    crates_cnt: i64,
}

#[cfg(test)]
mod tests {
    use super::*;
    use mockito::mock;

    #[actix_rt::test]
    async fn test() {
        let response = theory("rand", "get_crates_rand.json").await;

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
        theory("actix-web", "get_crates_actix_web.json").await;
    }

    #[actix_rt::test]
    async fn test_funny() {
        theory("funny", "get_crates_funny.json").await;
    }

    #[actix_rt::test]
    async fn test_serde() {
        theory("serde", "get_crates_serde.json").await;
    }

    #[actix_rt::test]
    async fn test_syn() {
        theory("syn", "get_crates_syn.json").await;
    }

    #[actix_rt::test]
    async fn test_tokio() {
        theory("tokio", "get_crates_tokio.json").await;
    }

    async fn theory(crate_name: &str, test_fixture_file: &str) -> Response {
        let path = format!("/api/v1/crates/{}", crate_name);

        let mock = mock("GET", path.as_str())
            .with_status(200)
            .with_header("content-type", "application/json; charset=utf-8")
            .with_body(
                std::fs::read_to_string(format!("./tests/fixtures/{}", test_fixture_file)).unwrap(),
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
