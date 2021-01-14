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
    badges: Vec<String>,
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
    homepage: String,
    #[serde(rename = "documentation")]
    documentation: String,
    #[serde(rename = "repository")]
    repository: String,
    #[serde(rename = "links")]
    links: CrateLinksResponse,
    #[serde(rename = "exact_match")]
    exact_match: bool,
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
    name: String,
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
        let mock = mock("GET", "/api/v1/crates/rand")
            .with_status(200)
            .with_header("content-type", "application/json; charset=utf-8")
            .with_body(RESPONSE)
            .match_header(
                "user-agent",
                "rust-kata-002 (https://github.com/agabani/rust-kata-002)",
            )
            .create();

        let client = CratesIoClient::new(&mockito::server_url()).unwrap();

        let response = client.get_crate("rand").await.unwrap();

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

        mock.assert();
    }

    const RESPONSE: &str = r#"
{
    "crate": {
        "id": "rand",
        "name": "rand",
        "updated_at": "2021-01-13T09:55:20.922296+00:00",
        "versions": [
            326822,
            322376,
            316445,
            202916,
            176558,
            176021,
            158945,
            158711,
            155960,
            154971,
            130460,
            127212,
            125555,
            125521,
            119212,
            117808,
            115893,
            113343,
            130165,
            102932,
            99538,
            98022,
            97193,
            95831,
            93664,
            92908,
            89893,
            86584,
            130166,
            127355,
            127151,
            104133,
            76564,
            74803,
            73853,
            130167,
            79903,
            78200,
            76563,
            75606,
            70485,
            67607,
            60891,
            38564,
            22300,
            20439,
            17884,
            15268,
            14687,
            13851,
            9238,
            7843,
            7771,
            7710,
            7648,
            7248,
            7239,
            7237,
            6964,
            6704,
            6029,
            5937,
            5252,
            4371,
            4362
        ],
        "keywords": [
            "random",
            "rng"
        ],
        "categories": [
            "no-std",
            "algorithms"
        ],
        "badges": [],
        "created_at": "2015-02-03T06:17:14.147783+00:00",
        "downloads": 51501017,
        "recent_downloads": 6821683,
        "max_version": "0.8.2",
        "newest_version": "0.8.2",
        "description": "Random number generators and other randomness functionality.\n",
        "homepage": "https://rust-random.github.io/book",
        "documentation": "https://docs.rs/rand",
        "repository": "https://github.com/rust-random/rand",
        "links": {
            "version_downloads": "/api/v1/crates/rand/downloads",
            "versions": null,
            "owners": "/api/v1/crates/rand/owners",
            "owner_team": "/api/v1/crates/rand/owner_team",
            "owner_user": "/api/v1/crates/rand/owner_user",
            "reverse_dependencies": "/api/v1/crates/rand/reverse_dependencies"
        },
        "exact_match": false
    },
    "versions": [
        {
            "id": 326822,
            "crate": "rand",
            "num": "0.8.2",
            "dl_path": "/api/v1/crates/rand/0.8.2/download",
            "readme_path": "/api/v1/crates/rand/0.8.2/readme",
            "updated_at": "2021-01-13T09:55:20.922296+00:00",
            "created_at": "2021-01-13T09:55:20.922296+00:00",
            "downloads": 26567,
            "features": {
                "alloc": [
                    "rand_core/alloc"
                ],
                "default": [
                    "std",
                    "std_rng"
                ],
                "getrandom": [
                    "rand_core/getrandom"
                ],
                "nightly": [],
                "serde1": [
                    "serde"
                ],
                "simd_support": [
                    "packed_simd"
                ],
                "small_rng": [],
                "std": [
                    "rand_core/std",
                    "rand_chacha/std",
                    "alloc",
                    "getrandom",
                    "libc"
                ],
                "std_rng": [
                    "rand_chacha",
                    "rand_hc"
                ]
            },
            "yanked": false,
            "license": "MIT OR Apache-2.0",
            "links": {
                "dependencies": "/api/v1/crates/rand/0.8.2/dependencies",
                "version_downloads": "/api/v1/crates/rand/0.8.2/downloads",
                "authors": "/api/v1/crates/rand/0.8.2/authors"
            },
            "crate_size": 84288,
            "published_by": {
                "id": 1234,
                "login": "dhardy",
                "name": "Diggory Hardy",
                "avatar": "https://avatars1.githubusercontent.com/u/134893?v=4",
                "url": "https://github.com/dhardy"
            },
            "audit_actions": [
                {
                    "action": "publish",
                    "user": {
                        "id": 1234,
                        "login": "dhardy",
                        "name": "Diggory Hardy",
                        "avatar": "https://avatars1.githubusercontent.com/u/134893?v=4",
                        "url": "https://github.com/dhardy"
                    },
                    "time": "2021-01-13T09:55:20.922296+00:00"
                }
            ]
        },
        {
            "id": 322376,
            "crate": "rand",
            "num": "0.8.1",
            "dl_path": "/api/v1/crates/rand/0.8.1/download",
            "readme_path": "/api/v1/crates/rand/0.8.1/readme",
            "updated_at": "2021-01-04T15:40:22.098135+00:00",
            "created_at": "2021-01-04T15:40:22.098135+00:00",
            "downloads": 95347,
            "features": {
                "alloc": [
                    "rand_core/alloc"
                ],
                "default": [
                    "std",
                    "std_rng"
                ],
                "getrandom": [
                    "rand_core/getrandom"
                ],
                "nightly": [],
                "serde1": [
                    "serde"
                ],
                "simd_support": [
                    "packed_simd"
                ],
                "small_rng": [],
                "std": [
                    "rand_core/std",
                    "rand_chacha/std",
                    "alloc",
                    "getrandom",
                    "libc"
                ],
                "std_rng": [
                    "rand_chacha",
                    "rand_hc"
                ]
            },
            "yanked": false,
            "license": "MIT OR Apache-2.0",
            "links": {
                "dependencies": "/api/v1/crates/rand/0.8.1/dependencies",
                "version_downloads": "/api/v1/crates/rand/0.8.1/downloads",
                "authors": "/api/v1/crates/rand/0.8.1/authors"
            },
            "crate_size": 84130,
            "published_by": {
                "id": 1234,
                "login": "dhardy",
                "name": "Diggory Hardy",
                "avatar": "https://avatars1.githubusercontent.com/u/134893?v=4",
                "url": "https://github.com/dhardy"
            },
            "audit_actions": [
                {
                    "action": "publish",
                    "user": {
                        "id": 1234,
                        "login": "dhardy",
                        "name": "Diggory Hardy",
                        "avatar": "https://avatars1.githubusercontent.com/u/134893?v=4",
                        "url": "https://github.com/dhardy"
                    },
                    "time": "2021-01-04T15:40:22.098135+00:00"
                }
            ]
        },
        {
            "id": 316445,
            "crate": "rand",
            "num": "0.8.0",
            "dl_path": "/api/v1/crates/rand/0.8.0/download",
            "readme_path": "/api/v1/crates/rand/0.8.0/readme",
            "updated_at": "2020-12-18T23:27:50.864347+00:00",
            "created_at": "2020-12-18T23:27:50.864347+00:00",
            "downloads": 86236,
            "features": {
                "alloc": [
                    "rand_core/alloc"
                ],
                "default": [
                    "std",
                    "std_rng"
                ],
                "getrandom": [
                    "rand_core/getrandom"
                ],
                "nightly": [],
                "serde1": [
                    "serde"
                ],
                "simd_support": [
                    "packed_simd"
                ],
                "small_rng": [],
                "std": [
                    "rand_core/std",
                    "rand_chacha/std",
                    "alloc",
                    "getrandom",
                    "libc"
                ],
                "std_rng": [
                    "rand_chacha",
                    "rand_hc"
                ]
            },
            "yanked": false,
            "license": "MIT OR Apache-2.0",
            "links": {
                "dependencies": "/api/v1/crates/rand/0.8.0/dependencies",
                "version_downloads": "/api/v1/crates/rand/0.8.0/downloads",
                "authors": "/api/v1/crates/rand/0.8.0/authors"
            },
            "crate_size": 84076,
            "published_by": {
                "id": 1234,
                "login": "dhardy",
                "name": "Diggory Hardy",
                "avatar": "https://avatars1.githubusercontent.com/u/134893?v=4",
                "url": "https://github.com/dhardy"
            },
            "audit_actions": [
                {
                    "action": "publish",
                    "user": {
                        "id": 1234,
                        "login": "dhardy",
                        "name": "Diggory Hardy",
                        "avatar": "https://avatars1.githubusercontent.com/u/134893?v=4",
                        "url": "https://github.com/dhardy"
                    },
                    "time": "2020-12-18T23:27:50.864347+00:00"
                }
            ]
        },
        {
            "id": 202916,
            "crate": "rand",
            "num": "0.7.3",
            "dl_path": "/api/v1/crates/rand/0.7.3/download",
            "readme_path": "/api/v1/crates/rand/0.7.3/readme",
            "updated_at": "2020-01-10T21:46:21.337656+00:00",
            "created_at": "2020-01-10T21:46:21.337656+00:00",
            "downloads": 9589433,
            "features": {
                "alloc": [
                    "rand_core/alloc"
                ],
                "default": [
                    "std"
                ],
                "getrandom": [
                    "getrandom_package",
                    "rand_core/getrandom"
                ],
                "nightly": [
                    "simd_support"
                ],
                "serde1": [],
                "simd_support": [
                    "packed_simd"
                ],
                "small_rng": [
                    "rand_pcg"
                ],
                "std": [
                    "rand_core/std",
                    "rand_chacha/std",
                    "alloc",
                    "getrandom",
                    "libc"
                ],
                "stdweb": [
                    "getrandom_package/stdweb"
                ],
                "wasm-bindgen": [
                    "getrandom_package/wasm-bindgen"
                ]
            },
            "yanked": false,
            "license": "MIT OR Apache-2.0",
            "links": {
                "dependencies": "/api/v1/crates/rand/0.7.3/dependencies",
                "version_downloads": "/api/v1/crates/rand/0.7.3/downloads",
                "authors": "/api/v1/crates/rand/0.7.3/authors"
            },
            "crate_size": 112246,
            "published_by": {
                "id": 1234,
                "login": "dhardy",
                "name": "Diggory Hardy",
                "avatar": "https://avatars1.githubusercontent.com/u/134893?v=4",
                "url": "https://github.com/dhardy"
            },
            "audit_actions": [
                {
                    "action": "publish",
                    "user": {
                        "id": 1234,
                        "login": "dhardy",
                        "name": "Diggory Hardy",
                        "avatar": "https://avatars1.githubusercontent.com/u/134893?v=4",
                        "url": "https://github.com/dhardy"
                    },
                    "time": "2020-01-10T21:46:21.337656+00:00"
                }
            ]
        },
        {
            "id": 176558,
            "crate": "rand",
            "num": "0.7.2",
            "dl_path": "/api/v1/crates/rand/0.7.2/download",
            "readme_path": "/api/v1/crates/rand/0.7.2/readme",
            "updated_at": "2019-09-16T20:09:24.860231+00:00",
            "created_at": "2019-09-16T20:09:24.860231+00:00",
            "downloads": 1969814,
            "features": {
                "alloc": [
                    "rand_core/alloc"
                ],
                "default": [
                    "std"
                ],
                "getrandom": [
                    "getrandom_package",
                    "rand_core/getrandom"
                ],
                "nightly": [
                    "simd_support"
                ],
                "serde1": [],
                "simd_support": [
                    "packed_simd"
                ],
                "small_rng": [
                    "rand_pcg"
                ],
                "std": [
                    "rand_core/std",
                    "rand_chacha/std",
                    "alloc",
                    "getrandom"
                ],
                "stdweb": [
                    "getrandom_package/stdweb"
                ],
                "wasm-bindgen": [
                    "getrandom_package/wasm-bindgen"
                ]
            },
            "yanked": false,
            "license": "MIT OR Apache-2.0",
            "links": {
                "dependencies": "/api/v1/crates/rand/0.7.2/dependencies",
                "version_downloads": "/api/v1/crates/rand/0.7.2/downloads",
                "authors": "/api/v1/crates/rand/0.7.2/authors"
            },
            "crate_size": 111438,
            "published_by": {
                "id": 1234,
                "login": "dhardy",
                "name": "Diggory Hardy",
                "avatar": "https://avatars1.githubusercontent.com/u/134893?v=4",
                "url": "https://github.com/dhardy"
            },
            "audit_actions": []
        },
        {
            "id": 176021,
            "crate": "rand",
            "num": "0.7.1",
            "dl_path": "/api/v1/crates/rand/0.7.1/download",
            "readme_path": "/api/v1/crates/rand/0.7.1/readme",
            "updated_at": "2019-09-16T20:09:38.509094+00:00",
            "created_at": "2019-09-13T15:10:33.345335+00:00",
            "downloads": 49277,
            "features": {
                "alloc": [
                    "rand_core/alloc"
                ],
                "default": [
                    "std"
                ],
                "getrandom": [
                    "getrandom_package",
                    "rand_core/getrandom"
                ],
                "nightly": [
                    "simd_support"
                ],
                "serde1": [],
                "simd_support": [
                    "packed_simd"
                ],
                "small_rng": [
                    "rand_pcg"
                ],
                "std": [
                    "rand_core/std",
                    "rand_chacha/std",
                    "alloc",
                    "getrandom"
                ],
                "stdweb": [
                    "getrandom_package/stdweb"
                ],
                "wasm-bindgen": [
                    "getrandom_package/wasm-bindgen"
                ]
            },
            "yanked": true,
            "license": "MIT OR Apache-2.0",
            "links": {
                "dependencies": "/api/v1/crates/rand/0.7.1/dependencies",
                "version_downloads": "/api/v1/crates/rand/0.7.1/downloads",
                "authors": "/api/v1/crates/rand/0.7.1/authors"
            },
            "crate_size": 108412,
            "published_by": {
                "id": 1234,
                "login": "dhardy",
                "name": "Diggory Hardy",
                "avatar": "https://avatars1.githubusercontent.com/u/134893?v=4",
                "url": "https://github.com/dhardy"
            },
            "audit_actions": []
        },
        {
            "id": 158945,
            "crate": "rand",
            "num": "0.7.0",
            "dl_path": "/api/v1/crates/rand/0.7.0/download",
            "readme_path": "/api/v1/crates/rand/0.7.0/readme",
            "updated_at": "2019-06-28T08:45:50.459959+00:00",
            "created_at": "2019-06-28T08:45:50.459959+00:00",
            "downloads": 1280192,
            "features": {
                "alloc": [
                    "rand_core/alloc"
                ],
                "default": [
                    "std"
                ],
                "getrandom": [
                    "getrandom_package",
                    "rand_core/getrandom"
                ],
                "nightly": [
                    "simd_support"
                ],
                "serde1": [],
                "simd_support": [
                    "packed_simd"
                ],
                "small_rng": [
                    "rand_pcg"
                ],
                "std": [
                    "rand_core/std",
                    "alloc",
                    "getrandom"
                ],
                "stdweb": [
                    "getrandom_package/stdweb"
                ],
                "wasm-bindgen": [
                    "getrandom_package/wasm-bindgen"
                ]
            },
            "yanked": false,
            "license": "MIT/Apache-2.0",
            "links": {
                "dependencies": "/api/v1/crates/rand/0.7.0/dependencies",
                "version_downloads": "/api/v1/crates/rand/0.7.0/downloads",
                "authors": "/api/v1/crates/rand/0.7.0/authors"
            },
            "crate_size": 104208,
            "published_by": {
                "id": 1234,
                "login": "dhardy",
                "name": "Diggory Hardy",
                "avatar": "https://avatars1.githubusercontent.com/u/134893?v=4",
                "url": "https://github.com/dhardy"
            },
            "audit_actions": []
        },
        {
            "id": 158711,
            "crate": "rand",
            "num": "0.7.0-pre.2",
            "dl_path": "/api/v1/crates/rand/0.7.0-pre.2/download",
            "readme_path": "/api/v1/crates/rand/0.7.0-pre.2/readme",
            "updated_at": "2019-06-27T09:37:26.903661+00:00",
            "created_at": "2019-06-27T09:37:26.903661+00:00",
            "downloads": 1429,
            "features": {
                "alloc": [
                    "rand_core/alloc"
                ],
                "default": [
                    "std"
                ],
                "getrandom": [
                    "getrandom_package",
                    "rand_core/getrandom"
                ],
                "nightly": [
                    "simd_support"
                ],
                "serde1": [],
                "simd_support": [
                    "packed_simd"
                ],
                "small_rng": [
                    "rand_pcg"
                ],
                "std": [
                    "rand_core/std",
                    "alloc",
                    "getrandom"
                ],
                "stdweb": [
                    "getrandom_package/stdweb"
                ],
                "wasm-bindgen": [
                    "getrandom_package/wasm-bindgen"
                ]
            },
            "yanked": false,
            "license": "MIT/Apache-2.0",
            "links": {
                "dependencies": "/api/v1/crates/rand/0.7.0-pre.2/dependencies",
                "version_downloads": "/api/v1/crates/rand/0.7.0-pre.2/downloads",
                "authors": "/api/v1/crates/rand/0.7.0-pre.2/authors"
            },
            "crate_size": 104192,
            "published_by": {
                "id": 1234,
                "login": "dhardy",
                "name": "Diggory Hardy",
                "avatar": "https://avatars1.githubusercontent.com/u/134893?v=4",
                "url": "https://github.com/dhardy"
            },
            "audit_actions": []
        },
        {
            "id": 155960,
            "crate": "rand",
            "num": "0.7.0-pre.1",
            "dl_path": "/api/v1/crates/rand/0.7.0-pre.1/download",
            "readme_path": "/api/v1/crates/rand/0.7.0-pre.1/readme",
            "updated_at": "2019-06-12T09:19:21.581541+00:00",
            "created_at": "2019-06-12T09:19:21.581541+00:00",
            "downloads": 2289,
            "features": {
                "alloc": [
                    "rand_core/alloc"
                ],
                "default": [
                    "std"
                ],
                "getrandom": [
                    "getrandom_package",
                    "rand_core/getrandom"
                ],
                "nightly": [
                    "simd_support"
                ],
                "serde1": [
                    "rand_core/serde1",
                    "rand_isaac/serde1",
                    "rand_xorshift/serde1"
                ],
                "simd_support": [
                    "packed_simd"
                ],
                "small_rng": [
                    "rand_pcg"
                ],
                "std": [
                    "rand_core/std",
                    "alloc",
                    "getrandom"
                ],
                "stdweb": [
                    "getrandom_package/stdweb"
                ],
                "wasm-bindgen": [
                    "getrandom_package/wasm-bindgen"
                ]
            },
            "yanked": false,
            "license": "MIT/Apache-2.0",
            "links": {
                "dependencies": "/api/v1/crates/rand/0.7.0-pre.1/dependencies",
                "version_downloads": "/api/v1/crates/rand/0.7.0-pre.1/downloads",
                "authors": "/api/v1/crates/rand/0.7.0-pre.1/authors"
            },
            "crate_size": 103377,
            "published_by": {
                "id": 1234,
                "login": "dhardy",
                "name": "Diggory Hardy",
                "avatar": "https://avatars1.githubusercontent.com/u/134893?v=4",
                "url": "https://github.com/dhardy"
            },
            "audit_actions": []
        },
        {
            "id": 154971,
            "crate": "rand",
            "num": "0.7.0-pre.0",
            "dl_path": "/api/v1/crates/rand/0.7.0-pre.0/download",
            "readme_path": "/api/v1/crates/rand/0.7.0-pre.0/readme",
            "updated_at": "2019-06-11T10:18:05.590937+00:00",
            "created_at": "2019-06-06T17:07:16.500459+00:00",
            "downloads": 1335,
            "features": {
                "alloc": [
                    "rand_core/alloc"
                ],
                "default": [
                    "std"
                ],
                "getrandom": [
                    "getrandom_package",
                    "rand_core/getrandom"
                ],
                "nightly": [
                    "simd_support"
                ],
                "serde1": [
                    "rand_core/serde1",
                    "rand_isaac/serde1",
                    "rand_xorshift/serde1"
                ],
                "simd_support": [
                    "packed_simd"
                ],
                "small_rng": [
                    "rand_pcg"
                ],
                "std": [
                    "rand_core/std",
                    "alloc",
                    "getrandom"
                ],
                "stdweb": [
                    "getrandom_package/stdweb"
                ],
                "wasm-bindgen": [
                    "getrandom_package/wasm-bindgen"
                ]
            },
            "yanked": false,
            "license": "MIT/Apache-2.0",
            "links": {
                "dependencies": "/api/v1/crates/rand/0.7.0-pre.0/dependencies",
                "version_downloads": "/api/v1/crates/rand/0.7.0-pre.0/downloads",
                "authors": "/api/v1/crates/rand/0.7.0-pre.0/authors"
            },
            "crate_size": 105174,
            "published_by": {
                "id": 1234,
                "login": "dhardy",
                "name": "Diggory Hardy",
                "avatar": "https://avatars1.githubusercontent.com/u/134893?v=4",
                "url": "https://github.com/dhardy"
            },
            "audit_actions": []
        },
        {
            "id": 130460,
            "crate": "rand",
            "num": "0.6.5",
            "dl_path": "/api/v1/crates/rand/0.6.5/download",
            "readme_path": "/api/v1/crates/rand/0.6.5/readme",
            "updated_at": "2019-01-28T09:56:57.788327+00:00",
            "created_at": "2019-01-28T09:56:57.788327+00:00",
            "downloads": 9316495,
            "features": {
                "alloc": [
                    "rand_core/alloc"
                ],
                "default": [
                    "std"
                ],
                "i128_support": [],
                "nightly": [
                    "simd_support"
                ],
                "serde1": [
                    "rand_core/serde1",
                    "rand_isaac/serde1",
                    "rand_xorshift/serde1"
                ],
                "simd_support": [
                    "packed_simd"
                ],
                "std": [
                    "rand_core/std",
                    "alloc",
                    "rand_os",
                    "rand_jitter/std"
                ],
                "stdweb": [
                    "rand_os/stdweb"
                ],
                "wasm-bindgen": [
                    "rand_os/wasm-bindgen"
                ]
            },
            "yanked": false,
            "license": "MIT/Apache-2.0",
            "links": {
                "dependencies": "/api/v1/crates/rand/0.6.5/dependencies",
                "version_downloads": "/api/v1/crates/rand/0.6.5/downloads",
                "authors": "/api/v1/crates/rand/0.6.5/authors"
            },
            "crate_size": 104814,
            "published_by": null,
            "audit_actions": []
        },
        {
            "id": 127212,
            "crate": "rand",
            "num": "0.6.4",
            "dl_path": "/api/v1/crates/rand/0.6.4/download",
            "readme_path": "/api/v1/crates/rand/0.6.4/readme",
            "updated_at": "2019-01-08T17:43:03.911462+00:00",
            "created_at": "2019-01-08T17:43:03.911462+00:00",
            "downloads": 299716,
            "features": {
                "alloc": [
                    "rand_core/alloc"
                ],
                "default": [
                    "std",
                    "rand_os"
                ],
                "i128_support": [],
                "nightly": [
                    "simd_support"
                ],
                "serde1": [
                    "rand_core/serde1",
                    "rand_isaac/serde1",
                    "rand_xorshift/serde1"
                ],
                "simd_support": [
                    "packed_simd"
                ],
                "std": [
                    "rand_core/std",
                    "alloc",
                    "rand_os"
                ],
                "stdweb": [
                    "rand_os/stdweb"
                ],
                "wasm-bindgen": [
                    "rand_os/wasm-bindgen"
                ]
            },
            "yanked": false,
            "license": "MIT/Apache-2.0",
            "links": {
                "dependencies": "/api/v1/crates/rand/0.6.4/dependencies",
                "version_downloads": "/api/v1/crates/rand/0.6.4/downloads",
                "authors": "/api/v1/crates/rand/0.6.4/authors"
            },
            "crate_size": 116260,
            "published_by": null,
            "audit_actions": []
        },
        {
            "id": 125555,
            "crate": "rand",
            "num": "0.6.3",
            "dl_path": "/api/v1/crates/rand/0.6.3/download",
            "readme_path": "/api/v1/crates/rand/0.6.3/readme",
            "updated_at": "2019-01-04T17:25:53.393514+00:00",
            "created_at": "2019-01-04T17:25:53.393514+00:00",
            "downloads": 74107,
            "features": {
                "alloc": [
                    "rand_core/alloc"
                ],
                "default": [
                    "std",
                    "rand_os"
                ],
                "i128_support": [],
                "nightly": [
                    "simd_support"
                ],
                "serde1": [
                    "rand_core/serde1",
                    "rand_isaac/serde1",
                    "rand_xorshift/serde1"
                ],
                "simd_support": [
                    "packed_simd"
                ],
                "std": [
                    "rand_core/std",
                    "alloc",
                    "rand_os"
                ],
                "stdweb": [
                    "rand_os/stdweb"
                ],
                "wasm-bindgen": [
                    "rand_os/wasm-bindgen"
                ]
            },
            "yanked": false,
            "license": "MIT/Apache-2.0",
            "links": {
                "dependencies": "/api/v1/crates/rand/0.6.3/dependencies",
                "version_downloads": "/api/v1/crates/rand/0.6.3/downloads",
                "authors": "/api/v1/crates/rand/0.6.3/authors"
            },
            "crate_size": 117566,
            "published_by": null,
            "audit_actions": []
        },
        {
            "id": 125521,
            "crate": "rand",
            "num": "0.6.2",
            "dl_path": "/api/v1/crates/rand/0.6.2/download",
            "readme_path": "/api/v1/crates/rand/0.6.2/readme",
            "updated_at": "2019-01-04T12:35:16.732753+00:00",
            "created_at": "2019-01-04T12:35:16.732753+00:00",
            "downloads": 4699,
            "features": {
                "alloc": [
                    "rand_core/alloc"
                ],
                "default": [
                    "std",
                    "rand_os"
                ],
                "i128_support": [],
                "nightly": [
                    "simd_support"
                ],
                "serde1": [
                    "rand_core/serde1",
                    "rand_isaac/serde1",
                    "rand_xorshift/serde1"
                ],
                "simd_support": [
                    "packed_simd"
                ],
                "std": [
                    "rand_core/std",
                    "alloc"
                ]
            },
            "yanked": false,
            "license": "MIT/Apache-2.0",
            "links": {
                "dependencies": "/api/v1/crates/rand/0.6.2/dependencies",
                "version_downloads": "/api/v1/crates/rand/0.6.2/downloads",
                "authors": "/api/v1/crates/rand/0.6.2/authors"
            },
            "crate_size": 117467,
            "published_by": null,
            "audit_actions": []
        },
        {
            "id": 119212,
            "crate": "rand",
            "num": "0.6.1",
            "dl_path": "/api/v1/crates/rand/0.6.1/download",
            "readme_path": "/api/v1/crates/rand/0.6.1/readme",
            "updated_at": "2018-11-23T10:43:32.927231+00:00",
            "created_at": "2018-11-23T10:43:32.927231+00:00",
            "downloads": 1001144,
            "features": {
                "alloc": [
                    "rand_core/alloc"
                ],
                "default": [
                    "std"
                ],
                "i128_support": [],
                "nightly": [
                    "simd_support"
                ],
                "serde1": [
                    "rand_core/serde1",
                    "rand_isaac/serde1",
                    "rand_xorshift/serde1"
                ],
                "simd_support": [
                    "packed_simd"
                ],
                "std": [
                    "rand_core/std",
                    "alloc",
                    "libc",
                    "winapi",
                    "cloudabi",
                    "fuchsia-zircon"
                ]
            },
            "yanked": false,
            "license": "MIT/Apache-2.0",
            "links": {
                "dependencies": "/api/v1/crates/rand/0.6.1/dependencies",
                "version_downloads": "/api/v1/crates/rand/0.6.1/downloads",
                "authors": "/api/v1/crates/rand/0.6.1/authors"
            },
            "crate_size": 126613,
            "published_by": null,
            "audit_actions": []
        },
        {
            "id": 117808,
            "crate": "rand",
            "num": "0.6.0",
            "dl_path": "/api/v1/crates/rand/0.6.0/download",
            "readme_path": "/api/v1/crates/rand/0.6.0/readme",
            "updated_at": "2018-11-14T11:36:08.017486+00:00",
            "created_at": "2018-11-14T11:36:08.017486+00:00",
            "downloads": 25789,
            "features": {
                "alloc": [
                    "rand_core/alloc"
                ],
                "default": [
                    "std"
                ],
                "i128_support": [],
                "nightly": [
                    "simd_support"
                ],
                "serde1": [
                    "rand_core/serde1",
                    "rand_isaac/serde1",
                    "rand_xorshift/serde1"
                ],
                "simd_support": [
                    "packed_simd"
                ],
                "std": [
                    "rand_core/std",
                    "alloc",
                    "libc",
                    "winapi",
                    "cloudabi",
                    "fuchsia-zircon"
                ]
            },
            "yanked": false,
            "license": "MIT/Apache-2.0",
            "links": {
                "dependencies": "/api/v1/crates/rand/0.6.0/dependencies",
                "version_downloads": "/api/v1/crates/rand/0.6.0/downloads",
                "authors": "/api/v1/crates/rand/0.6.0/authors"
            },
            "crate_size": 126632,
            "published_by": null,
            "audit_actions": []
        },
        {
            "id": 115893,
            "crate": "rand",
            "num": "0.6.0-pre.1",
            "dl_path": "/api/v1/crates/rand/0.6.0-pre.1/download",
            "readme_path": "/api/v1/crates/rand/0.6.0-pre.1/readme",
            "updated_at": "2018-11-02T14:32:22.000182+00:00",
            "created_at": "2018-11-02T14:32:22.000182+00:00",
            "downloads": 2511,
            "features": {
                "alloc": [
                    "rand_core/alloc"
                ],
                "default": [
                    "std"
                ],
                "i128_support": [],
                "nightly": [
                    "simd_support"
                ],
                "serde1": [
                    "rand_core/serde1",
                    "rand_isaac/serde1",
                    "rand_xorshift/serde1"
                ],
                "simd_support": [
                    "packed_simd"
                ],
                "std": [
                    "rand_core/std",
                    "alloc",
                    "libc",
                    "winapi",
                    "cloudabi",
                    "fuchsia-zircon"
                ]
            },
            "yanked": false,
            "license": "MIT/Apache-2.0",
            "links": {
                "dependencies": "/api/v1/crates/rand/0.6.0-pre.1/dependencies",
                "version_downloads": "/api/v1/crates/rand/0.6.0-pre.1/downloads",
                "authors": "/api/v1/crates/rand/0.6.0-pre.1/authors"
            },
            "crate_size": 142150,
            "published_by": null,
            "audit_actions": []
        },
        {
            "id": 113343,
            "crate": "rand",
            "num": "0.6.0-pre.0",
            "dl_path": "/api/v1/crates/rand/0.6.0-pre.0/download",
            "readme_path": "/api/v1/crates/rand/0.6.0-pre.0/readme",
            "updated_at": "2018-10-17T10:11:49.105381+00:00",
            "created_at": "2018-10-17T10:11:49.105381+00:00",
            "downloads": 1837,
            "features": {
                "alloc": [
                    "rand_core/alloc"
                ],
                "default": [
                    "std"
                ],
                "i128_support": [],
                "nightly": [
                    "simd_support"
                ],
                "serde1": [
                    "rand_core/serde1",
                    "rand_isaac/serde1",
                    "rand_xorshift/serde1"
                ],
                "simd_support": [
                    "packed_simd"
                ],
                "std": [
                    "rand_core/std",
                    "alloc",
                    "libc",
                    "winapi",
                    "cloudabi",
                    "fuchsia-zircon"
                ]
            },
            "yanked": false,
            "license": "MIT/Apache-2.0",
            "links": {
                "dependencies": "/api/v1/crates/rand/0.6.0-pre.0/dependencies",
                "version_downloads": "/api/v1/crates/rand/0.6.0-pre.0/downloads",
                "authors": "/api/v1/crates/rand/0.6.0-pre.0/authors"
            },
            "crate_size": 147575,
            "published_by": null,
            "audit_actions": []
        },
        {
            "id": 130165,
            "crate": "rand",
            "num": "0.5.6",
            "dl_path": "/api/v1/crates/rand/0.5.6/download",
            "readme_path": "/api/v1/crates/rand/0.5.6/readme",
            "updated_at": "2019-01-26T10:20:14.558184+00:00",
            "created_at": "2019-01-26T10:20:14.558184+00:00",
            "downloads": 3400778,
            "features": {
                "alloc": [
                    "rand_core/alloc"
                ],
                "default": [
                    "std"
                ],
                "i128_support": [],
                "nightly": [
                    "i128_support"
                ],
                "serde1": [
                    "serde",
                    "serde_derive",
                    "rand_core/serde1"
                ],
                "std": [
                    "rand_core/std",
                    "alloc",
                    "libc",
                    "winapi",
                    "cloudabi",
                    "fuchsia-cprng"
                ]
            },
            "yanked": false,
            "license": "MIT/Apache-2.0",
            "links": {
                "dependencies": "/api/v1/crates/rand/0.5.6/dependencies",
                "version_downloads": "/api/v1/crates/rand/0.5.6/downloads",
                "authors": "/api/v1/crates/rand/0.5.6/authors"
            },
            "crate_size": 137236,
            "published_by": null,
            "audit_actions": []
        },
        {
            "id": 102932,
            "crate": "rand",
            "num": "0.5.5",
            "dl_path": "/api/v1/crates/rand/0.5.5/download",
            "readme_path": "/api/v1/crates/rand/0.5.5/readme",
            "updated_at": "2018-08-07T15:59:14.899961+00:00",
            "created_at": "2018-08-07T15:59:14.899961+00:00",
            "downloads": 1831573,
            "features": {
                "alloc": [
                    "rand_core/alloc"
                ],
                "default": [
                    "std"
                ],
                "i128_support": [],
                "nightly": [
                    "i128_support"
                ],
                "serde1": [
                    "serde",
                    "serde_derive",
                    "rand_core/serde1"
                ],
                "std": [
                    "rand_core/std",
                    "alloc",
                    "libc",
                    "winapi",
                    "cloudabi",
                    "fuchsia-zircon"
                ]
            },
            "yanked": false,
            "license": "MIT/Apache-2.0",
            "links": {
                "dependencies": "/api/v1/crates/rand/0.5.5/dependencies",
                "version_downloads": "/api/v1/crates/rand/0.5.5/downloads",
                "authors": "/api/v1/crates/rand/0.5.5/authors"
            },
            "crate_size": null,
            "published_by": null,
            "audit_actions": []
        },
        {
            "id": 99538,
            "crate": "rand",
            "num": "0.5.4",
            "dl_path": "/api/v1/crates/rand/0.5.4/download",
            "readme_path": "/api/v1/crates/rand/0.5.4/readme",
            "updated_at": "2018-07-12T10:03:12.973175+00:00",
            "created_at": "2018-07-12T10:03:12.973175+00:00",
            "downloads": 247530,
            "features": {
                "alloc": [
                    "rand_core/alloc"
                ],
                "default": [
                    "std"
                ],
                "i128_support": [],
                "nightly": [
                    "i128_support"
                ],
                "serde1": [
                    "serde",
                    "serde_derive",
                    "rand_core/serde1"
                ],
                "std": [
                    "rand_core/std",
                    "alloc",
                    "libc",
                    "winapi",
                    "cloudabi",
                    "fuchsia-zircon"
                ]
            },
            "yanked": false,
            "license": "MIT/Apache-2.0",
            "links": {
                "dependencies": "/api/v1/crates/rand/0.5.4/dependencies",
                "version_downloads": "/api/v1/crates/rand/0.5.4/downloads",
                "authors": "/api/v1/crates/rand/0.5.4/authors"
            },
            "crate_size": null,
            "published_by": null,
            "audit_actions": []
        },
        {
            "id": 98022,
            "crate": "rand",
            "num": "0.5.3",
            "dl_path": "/api/v1/crates/rand/0.5.3/download",
            "readme_path": "/api/v1/crates/rand/0.5.3/readme",
            "updated_at": "2018-06-27T09:31:10.660151+00:00",
            "created_at": "2018-06-27T09:31:10.660151+00:00",
            "downloads": 35556,
            "features": {
                "alloc": [
                    "rand_core/alloc"
                ],
                "default": [
                    "std"
                ],
                "i128_support": [],
                "nightly": [
                    "i128_support"
                ],
                "serde1": [
                    "serde",
                    "serde_derive",
                    "rand_core/serde1"
                ],
                "std": [
                    "rand_core/std",
                    "alloc",
                    "libc",
                    "winapi",
                    "cloudabi",
                    "fuchsia-zircon"
                ]
            },
            "yanked": false,
            "license": "MIT/Apache-2.0",
            "links": {
                "dependencies": "/api/v1/crates/rand/0.5.3/dependencies",
                "version_downloads": "/api/v1/crates/rand/0.5.3/downloads",
                "authors": "/api/v1/crates/rand/0.5.3/authors"
            },
            "crate_size": null,
            "published_by": null,
            "audit_actions": []
        },
        {
            "id": 97193,
            "crate": "rand",
            "num": "0.5.2",
            "dl_path": "/api/v1/crates/rand/0.5.2/download",
            "readme_path": "/api/v1/crates/rand/0.5.2/readme",
            "updated_at": "2018-06-20T08:18:32.801565+00:00",
            "created_at": "2018-06-20T08:18:32.801565+00:00",
            "downloads": 14799,
            "features": {
                "alloc": [
                    "rand_core/alloc"
                ],
                "default": [
                    "std"
                ],
                "i128_support": [],
                "nightly": [
                    "i128_support"
                ],
                "serde1": [
                    "serde",
                    "serde_derive",
                    "rand_core/serde1"
                ],
                "std": [
                    "rand_core/std",
                    "alloc",
                    "libc",
                    "winapi",
                    "cloudabi",
                    "fuchsia-zircon"
                ]
            },
            "yanked": false,
            "license": "MIT/Apache-2.0",
            "links": {
                "dependencies": "/api/v1/crates/rand/0.5.2/dependencies",
                "version_downloads": "/api/v1/crates/rand/0.5.2/downloads",
                "authors": "/api/v1/crates/rand/0.5.2/authors"
            },
            "crate_size": null,
            "published_by": null,
            "audit_actions": []
        },
        {
            "id": 95831,
            "crate": "rand",
            "num": "0.5.1",
            "dl_path": "/api/v1/crates/rand/0.5.1/download",
            "readme_path": "/api/v1/crates/rand/0.5.1/readme",
            "updated_at": "2018-06-08T07:47:42.218944+00:00",
            "created_at": "2018-06-08T07:47:42.218944+00:00",
            "downloads": 24956,
            "features": {
                "alloc": [
                    "rand_core/alloc"
                ],
                "default": [
                    "std"
                ],
                "i128_support": [],
                "nightly": [
                    "i128_support"
                ],
                "serde1": [
                    "serde",
                    "serde_derive",
                    "rand_core/serde1"
                ],
                "std": [
                    "rand_core/std",
                    "alloc",
                    "libc",
                    "winapi",
                    "cloudabi",
                    "fuchsia-zircon"
                ]
            },
            "yanked": false,
            "license": "MIT/Apache-2.0",
            "links": {
                "dependencies": "/api/v1/crates/rand/0.5.1/dependencies",
                "version_downloads": "/api/v1/crates/rand/0.5.1/downloads",
                "authors": "/api/v1/crates/rand/0.5.1/authors"
            },
            "crate_size": null,
            "published_by": null,
            "audit_actions": []
        },
        {
            "id": 93664,
            "crate": "rand",
            "num": "0.5.0",
            "dl_path": "/api/v1/crates/rand/0.5.0/download",
            "readme_path": "/api/v1/crates/rand/0.5.0/readme",
            "updated_at": "2018-05-21T15:33:10.166567+00:00",
            "created_at": "2018-05-21T15:33:10.166567+00:00",
            "downloads": 59193,
            "features": {
                "alloc": [
                    "rand_core/alloc"
                ],
                "default": [
                    "std"
                ],
                "i128_support": [],
                "nightly": [
                    "i128_support"
                ],
                "serde1": [
                    "serde",
                    "serde_derive",
                    "rand_core/serde1"
                ],
                "std": [
                    "rand_core/std",
                    "alloc",
                    "libc",
                    "winapi",
                    "cloudabi",
                    "fuchsia-zircon"
                ]
            },
            "yanked": false,
            "license": "MIT/Apache-2.0",
            "links": {
                "dependencies": "/api/v1/crates/rand/0.5.0/dependencies",
                "version_downloads": "/api/v1/crates/rand/0.5.0/downloads",
                "authors": "/api/v1/crates/rand/0.5.0/authors"
            },
            "crate_size": null,
            "published_by": null,
            "audit_actions": []
        },
        {
            "id": 92908,
            "crate": "rand",
            "num": "0.5.0-pre.2",
            "dl_path": "/api/v1/crates/rand/0.5.0-pre.2/download",
            "readme_path": "/api/v1/crates/rand/0.5.0-pre.2/readme",
            "updated_at": "2018-05-15T11:24:19.867458+00:00",
            "created_at": "2018-05-15T11:24:19.867458+00:00",
            "downloads": 14170,
            "features": {
                "alloc": [
                    "rand_core/alloc"
                ],
                "default": [
                    "std"
                ],
                "i128_support": [],
                "nightly": [
                    "i128_support"
                ],
                "serde1": [
                    "serde",
                    "serde_derive",
                    "rand_core/serde1"
                ],
                "std": [
                    "rand_core/std",
                    "alloc",
                    "libc",
                    "winapi",
                    "cloudabi",
                    "fuchsia-zircon"
                ]
            },
            "yanked": false,
            "license": "MIT/Apache-2.0",
            "links": {
                "dependencies": "/api/v1/crates/rand/0.5.0-pre.2/dependencies",
                "version_downloads": "/api/v1/crates/rand/0.5.0-pre.2/downloads",
                "authors": "/api/v1/crates/rand/0.5.0-pre.2/authors"
            },
            "crate_size": null,
            "published_by": null,
            "audit_actions": []
        },
        {
            "id": 89893,
            "crate": "rand",
            "num": "0.5.0-pre.1",
            "dl_path": "/api/v1/crates/rand/0.5.0-pre.1/download",
            "readme_path": "/api/v1/crates/rand/0.5.0-pre.1/readme",
            "updated_at": "2018-04-22T09:08:33.322846+00:00",
            "created_at": "2018-04-22T09:08:33.322846+00:00",
            "downloads": 3910,
            "features": {
                "alloc": [
                    "rand_core/alloc"
                ],
                "default": [
                    "std"
                ],
                "i128_support": [],
                "nightly": [
                    "i128_support"
                ],
                "serde1": [
                    "serde",
                    "serde_derive",
                    "rand_core/serde1"
                ],
                "std": [
                    "rand_core/std",
                    "alloc",
                    "libc",
                    "winapi",
                    "cloudabi",
                    "fuchsia-zircon"
                ]
            },
            "yanked": false,
            "license": "MIT/Apache-2.0",
            "links": {
                "dependencies": "/api/v1/crates/rand/0.5.0-pre.1/dependencies",
                "version_downloads": "/api/v1/crates/rand/0.5.0-pre.1/downloads",
                "authors": "/api/v1/crates/rand/0.5.0-pre.1/authors"
            },
            "crate_size": null,
            "published_by": null,
            "audit_actions": []
        },
        {
            "id": 86584,
            "crate": "rand",
            "num": "0.5.0-pre.0",
            "dl_path": "/api/v1/crates/rand/0.5.0-pre.0/download",
            "readme_path": "/api/v1/crates/rand/0.5.0-pre.0/readme",
            "updated_at": "2018-03-27T11:41:46.243023+00:00",
            "created_at": "2018-03-27T11:41:46.243023+00:00",
            "downloads": 3203,
            "features": {
                "alloc": [
                    "rand_core/alloc"
                ],
                "default": [
                    "std"
                ],
                "i128_support": [],
                "nightly": [
                    "i128_support"
                ],
                "serde-1": [
                    "serde",
                    "serde_derive"
                ],
                "std": [
                    "rand_core/std",
                    "winapi",
                    "libc",
                    "alloc"
                ]
            },
            "yanked": false,
            "license": "MIT/Apache-2.0",
            "links": {
                "dependencies": "/api/v1/crates/rand/0.5.0-pre.0/dependencies",
                "version_downloads": "/api/v1/crates/rand/0.5.0-pre.0/downloads",
                "authors": "/api/v1/crates/rand/0.5.0-pre.0/authors"
            },
            "crate_size": null,
            "published_by": null,
            "audit_actions": []
        },
        {
            "id": 130166,
            "crate": "rand",
            "num": "0.4.6",
            "dl_path": "/api/v1/crates/rand/0.4.6/download",
            "readme_path": "/api/v1/crates/rand/0.4.6/readme",
            "updated_at": "2019-01-26T10:21:14.395663+00:00",
            "created_at": "2019-01-26T10:21:14.395663+00:00",
            "downloads": 7539069,
            "features": {
                "alloc": [],
                "default": [
                    "std"
                ],
                "i128_support": [],
                "nightly": [
                    "i128_support"
                ],
                "std": [
                    "libc"
                ]
            },
            "yanked": false,
            "license": "MIT/Apache-2.0",
            "links": {
                "dependencies": "/api/v1/crates/rand/0.4.6/dependencies",
                "version_downloads": "/api/v1/crates/rand/0.4.6/downloads",
                "authors": "/api/v1/crates/rand/0.4.6/authors"
            },
            "crate_size": 76401,
            "published_by": null,
            "audit_actions": []
        },
        {
            "id": 127355,
            "crate": "rand",
            "num": "0.4.5",
            "dl_path": "/api/v1/crates/rand/0.4.5/download",
            "readme_path": "/api/v1/crates/rand/0.4.5/readme",
            "updated_at": "2019-01-09T16:25:16.882709+00:00",
            "created_at": "2019-01-09T16:25:16.882709+00:00",
            "downloads": 165271,
            "features": {
                "alloc": [],
                "default": [
                    "std"
                ],
                "i128_support": [],
                "nightly": [
                    "i128_support"
                ],
                "std": [
                    "libc"
                ]
            },
            "yanked": false,
            "license": "MIT/Apache-2.0",
            "links": {
                "dependencies": "/api/v1/crates/rand/0.4.5/dependencies",
                "version_downloads": "/api/v1/crates/rand/0.4.5/downloads",
                "authors": "/api/v1/crates/rand/0.4.5/authors"
            },
            "crate_size": 76465,
            "published_by": null,
            "audit_actions": []
        },
        {
            "id": 127151,
            "crate": "rand",
            "num": "0.4.4",
            "dl_path": "/api/v1/crates/rand/0.4.4/download",
            "readme_path": "/api/v1/crates/rand/0.4.4/readme",
            "updated_at": "2019-01-09T16:25:41.940799+00:00",
            "created_at": "2019-01-08T12:39:19.894769+00:00",
            "downloads": 11894,
            "features": {
                "alloc": [],
                "default": [
                    "std"
                ],
                "i128_support": [],
                "nightly": [
                    "i128_support"
                ],
                "std": [
                    "libc"
                ]
            },
            "yanked": true,
            "license": "MIT/Apache-2.0",
            "links": {
                "dependencies": "/api/v1/crates/rand/0.4.4/dependencies",
                "version_downloads": "/api/v1/crates/rand/0.4.4/downloads",
                "authors": "/api/v1/crates/rand/0.4.4/authors"
            },
            "crate_size": 76460,
            "published_by": null,
            "audit_actions": []
        },
        {
            "id": 104133,
            "crate": "rand",
            "num": "0.4.3",
            "dl_path": "/api/v1/crates/rand/0.4.3/download",
            "readme_path": "/api/v1/crates/rand/0.4.3/readme",
            "updated_at": "2018-08-16T11:35:56.572441+00:00",
            "created_at": "2018-08-16T11:35:56.572441+00:00",
            "downloads": 1521068,
            "features": {
                "alloc": [],
                "default": [
                    "std"
                ],
                "i128_support": [],
                "nightly": [
                    "i128_support"
                ],
                "std": [
                    "libc"
                ]
            },
            "yanked": false,
            "license": "MIT/Apache-2.0",
            "links": {
                "dependencies": "/api/v1/crates/rand/0.4.3/dependencies",
                "version_downloads": "/api/v1/crates/rand/0.4.3/downloads",
                "authors": "/api/v1/crates/rand/0.4.3/authors"
            },
            "crate_size": 76094,
            "published_by": null,
            "audit_actions": []
        },
        {
            "id": 76564,
            "crate": "rand",
            "num": "0.4.2",
            "dl_path": "/api/v1/crates/rand/0.4.2/download",
            "readme_path": "/api/v1/crates/rand/0.4.2/readme",
            "updated_at": "2018-01-06T11:26:47.039110+00:00",
            "created_at": "2018-01-06T11:26:47.039110+00:00",
            "downloads": 1907185,
            "features": {
                "alloc": [],
                "default": [
                    "std"
                ],
                "i128_support": [],
                "nightly": [
                    "i128_support"
                ],
                "std": [
                    "libc"
                ]
            },
            "yanked": false,
            "license": "MIT/Apache-2.0",
            "links": {
                "dependencies": "/api/v1/crates/rand/0.4.2/dependencies",
                "version_downloads": "/api/v1/crates/rand/0.4.2/downloads",
                "authors": "/api/v1/crates/rand/0.4.2/authors"
            },
            "crate_size": null,
            "published_by": null,
            "audit_actions": []
        },
        {
            "id": 74803,
            "crate": "rand",
            "num": "0.4.1",
            "dl_path": "/api/v1/crates/rand/0.4.1/download",
            "readme_path": "/api/v1/crates/rand/0.4.1/readme",
            "updated_at": "2017-12-18T11:28:02.757880+00:00",
            "created_at": "2017-12-18T11:28:02.757880+00:00",
            "downloads": 45259,
            "features": {
                "alloc": [],
                "default": [
                    "std"
                ],
                "i128_support": [],
                "nightly": [
                    "i128_support"
                ],
                "std": [
                    "libc"
                ]
            },
            "yanked": false,
            "license": "MIT/Apache-2.0",
            "links": {
                "dependencies": "/api/v1/crates/rand/0.4.1/dependencies",
                "version_downloads": "/api/v1/crates/rand/0.4.1/downloads",
                "authors": "/api/v1/crates/rand/0.4.1/authors"
            },
            "crate_size": null,
            "published_by": null,
            "audit_actions": []
        },
        {
            "id": 73853,
            "crate": "rand",
            "num": "0.4.0-pre.0",
            "dl_path": "/api/v1/crates/rand/0.4.0-pre.0/download",
            "readme_path": "/api/v1/crates/rand/0.4.0-pre.0/readme",
            "updated_at": "2017-12-11T17:04:54.749661+00:00",
            "created_at": "2017-12-11T17:04:54.749661+00:00",
            "downloads": 1588,
            "features": {
                "i128_support": [],
                "nightly": [
                    "i128_support"
                ]
            },
            "yanked": false,
            "license": "MIT/Apache-2.0",
            "links": {
                "dependencies": "/api/v1/crates/rand/0.4.0-pre.0/dependencies",
                "version_downloads": "/api/v1/crates/rand/0.4.0-pre.0/downloads",
                "authors": "/api/v1/crates/rand/0.4.0-pre.0/authors"
            },
            "crate_size": null,
            "published_by": null,
            "audit_actions": []
        },
        {
            "id": 130167,
            "crate": "rand",
            "num": "0.3.23",
            "dl_path": "/api/v1/crates/rand/0.3.23/download",
            "readme_path": "/api/v1/crates/rand/0.3.23/readme",
            "updated_at": "2019-01-26T10:22:04.993065+00:00",
            "created_at": "2019-01-26T10:22:04.993065+00:00",
            "downloads": 3748328,
            "features": {
                "i128_support": [],
                "nightly": [
                    "i128_support"
                ]
            },
            "yanked": false,
            "license": "MIT/Apache-2.0",
            "links": {
                "dependencies": "/api/v1/crates/rand/0.3.23/dependencies",
                "version_downloads": "/api/v1/crates/rand/0.3.23/downloads",
                "authors": "/api/v1/crates/rand/0.3.23/authors"
            },
            "crate_size": 11318,
            "published_by": null,
            "audit_actions": []
        },
        {
            "id": 79903,
            "crate": "rand",
            "num": "0.3.22",
            "dl_path": "/api/v1/crates/rand/0.3.22/download",
            "readme_path": "/api/v1/crates/rand/0.3.22/readme",
            "updated_at": "2018-02-05T09:56:34.099992+00:00",
            "created_at": "2018-02-05T09:56:34.099992+00:00",
            "downloads": 1842769,
            "features": {
                "i128_support": [],
                "nightly": [
                    "i128_support"
                ]
            },
            "yanked": false,
            "license": "MIT/Apache-2.0",
            "links": {
                "dependencies": "/api/v1/crates/rand/0.3.22/dependencies",
                "version_downloads": "/api/v1/crates/rand/0.3.22/downloads",
                "authors": "/api/v1/crates/rand/0.3.22/authors"
            },
            "crate_size": null,
            "published_by": null,
            "audit_actions": []
        },
        {
            "id": 78200,
            "crate": "rand",
            "num": "0.3.21-pre.0",
            "dl_path": "/api/v1/crates/rand/0.3.21-pre.0/download",
            "readme_path": "/api/v1/crates/rand/0.3.21-pre.0/readme",
            "updated_at": "2018-01-21T15:38:12.762298+00:00",
            "created_at": "2018-01-21T15:38:12.762298+00:00",
            "downloads": 1446,
            "features": {
                "i128_support": [],
                "nightly": [
                    "i128_support"
                ]
            },
            "yanked": false,
            "license": "MIT/Apache-2.0",
            "links": {
                "dependencies": "/api/v1/crates/rand/0.3.21-pre.0/dependencies",
                "version_downloads": "/api/v1/crates/rand/0.3.21-pre.0/downloads",
                "authors": "/api/v1/crates/rand/0.3.21-pre.0/authors"
            },
            "crate_size": null,
            "published_by": null,
            "audit_actions": []
        },
        {
            "id": 76563,
            "crate": "rand",
            "num": "0.3.20",
            "dl_path": "/api/v1/crates/rand/0.3.20/download",
            "readme_path": "/api/v1/crates/rand/0.3.20/readme",
            "updated_at": "2018-01-06T11:23:51.744471+00:00",
            "created_at": "2018-01-06T11:23:51.744471+00:00",
            "downloads": 240177,
            "features": {
                "i128_support": [],
                "nightly": [
                    "i128_support"
                ]
            },
            "yanked": false,
            "license": "MIT/Apache-2.0",
            "links": {
                "dependencies": "/api/v1/crates/rand/0.3.20/dependencies",
                "version_downloads": "/api/v1/crates/rand/0.3.20/downloads",
                "authors": "/api/v1/crates/rand/0.3.20/authors"
            },
            "crate_size": null,
            "published_by": null,
            "audit_actions": []
        },
        {
            "id": 75606,
            "crate": "rand",
            "num": "0.3.19",
            "dl_path": "/api/v1/crates/rand/0.3.19/download",
            "readme_path": "/api/v1/crates/rand/0.3.19/readme",
            "updated_at": "2017-12-27T15:08:08.777274+00:00",
            "created_at": "2017-12-27T15:08:08.777274+00:00",
            "downloads": 87560,
            "features": {
                "i128_support": [],
                "nightly": [
                    "i128_support"
                ]
            },
            "yanked": false,
            "license": "MIT/Apache-2.0",
            "links": {
                "dependencies": "/api/v1/crates/rand/0.3.19/dependencies",
                "version_downloads": "/api/v1/crates/rand/0.3.19/downloads",
                "authors": "/api/v1/crates/rand/0.3.19/authors"
            },
            "crate_size": null,
            "published_by": null,
            "audit_actions": []
        },
        {
            "id": 70485,
            "crate": "rand",
            "num": "0.3.18",
            "dl_path": "/api/v1/crates/rand/0.3.18/download",
            "readme_path": "/api/v1/crates/rand/0.3.18/readme",
            "updated_at": "2017-11-30T03:01:39.779366+00:00",
            "created_at": "2017-11-06T16:14:49.577429+00:00",
            "downloads": 446895,
            "features": {
                "i128_support": [],
                "nightly": [
                    "i128_support"
                ]
            },
            "yanked": false,
            "license": "MIT/Apache-2.0",
            "links": {
                "dependencies": "/api/v1/crates/rand/0.3.18/dependencies",
                "version_downloads": "/api/v1/crates/rand/0.3.18/downloads",
                "authors": "/api/v1/crates/rand/0.3.18/authors"
            },
            "crate_size": null,
            "published_by": null,
            "audit_actions": []
        },
        {
            "id": 67607,
            "crate": "rand",
            "num": "0.3.17",
            "dl_path": "/api/v1/crates/rand/0.3.17/download",
            "readme_path": "/api/v1/crates/rand/0.3.17/readme",
            "updated_at": "2017-11-30T03:23:09.777592+00:00",
            "created_at": "2017-10-06T23:14:49.189763+00:00",
            "downloads": 213845,
            "features": {
                "i128_support": [],
                "nightly": [
                    "i128_support"
                ]
            },
            "yanked": false,
            "license": "MIT/Apache-2.0",
            "links": {
                "dependencies": "/api/v1/crates/rand/0.3.17/dependencies",
                "version_downloads": "/api/v1/crates/rand/0.3.17/downloads",
                "authors": "/api/v1/crates/rand/0.3.17/authors"
            },
            "crate_size": null,
            "published_by": null,
            "audit_actions": []
        },
        {
            "id": 60891,
            "crate": "rand",
            "num": "0.3.16",
            "dl_path": "/api/v1/crates/rand/0.3.16/download",
            "readme_path": "/api/v1/crates/rand/0.3.16/readme",
            "updated_at": "2017-11-30T02:38:03.828610+00:00",
            "created_at": "2017-07-27T21:36:53.538621+00:00",
            "downloads": 429545,
            "features": {
                "i128_support": [],
                "nightly": [
                    "i128_support"
                ]
            },
            "yanked": false,
            "license": "MIT/Apache-2.0",
            "links": {
                "dependencies": "/api/v1/crates/rand/0.3.16/dependencies",
                "version_downloads": "/api/v1/crates/rand/0.3.16/downloads",
                "authors": "/api/v1/crates/rand/0.3.16/authors"
            },
            "crate_size": null,
            "published_by": null,
            "audit_actions": []
        },
        {
            "id": 38564,
            "crate": "rand",
            "num": "0.3.15",
            "dl_path": "/api/v1/crates/rand/0.3.15/download",
            "readme_path": "/api/v1/crates/rand/0.3.15/readme",
            "updated_at": "2017-11-30T03:52:13.958691+00:00",
            "created_at": "2016-11-26T22:34:32.458356+00:00",
            "downloads": 1660677,
            "features": {},
            "yanked": false,
            "license": "MIT/Apache-2.0",
            "links": {
                "dependencies": "/api/v1/crates/rand/0.3.15/dependencies",
                "version_downloads": "/api/v1/crates/rand/0.3.15/downloads",
                "authors": "/api/v1/crates/rand/0.3.15/authors"
            },
            "crate_size": null,
            "published_by": null,
            "audit_actions": []
        },
        {
            "id": 22300,
            "crate": "rand",
            "num": "0.3.14",
            "dl_path": "/api/v1/crates/rand/0.3.14/download",
            "readme_path": "/api/v1/crates/rand/0.3.14/readme",
            "updated_at": "2017-11-30T02:50:07.026891+00:00",
            "created_at": "2016-02-13T08:28:26.855136+00:00",
            "downloads": 1414274,
            "features": {},
            "yanked": false,
            "license": "MIT/Apache-2.0",
            "links": {
                "dependencies": "/api/v1/crates/rand/0.3.14/dependencies",
                "version_downloads": "/api/v1/crates/rand/0.3.14/downloads",
                "authors": "/api/v1/crates/rand/0.3.14/authors"
            },
            "crate_size": null,
            "published_by": null,
            "audit_actions": []
        },
        {
            "id": 20439,
            "crate": "rand",
            "num": "0.3.13",
            "dl_path": "/api/v1/crates/rand/0.3.13/download",
            "readme_path": "/api/v1/crates/rand/0.3.13/readme",
            "updated_at": "2017-11-30T04:00:53.555990+00:00",
            "created_at": "2016-01-09T17:59:17.530313+00:00",
            "downloads": 125551,
            "features": {},
            "yanked": false,
            "license": "MIT/Apache-2.0",
            "links": {
                "dependencies": "/api/v1/crates/rand/0.3.13/dependencies",
                "version_downloads": "/api/v1/crates/rand/0.3.13/downloads",
                "authors": "/api/v1/crates/rand/0.3.13/authors"
            },
            "crate_size": null,
            "published_by": null,
            "audit_actions": []
        },
        {
            "id": 17884,
            "crate": "rand",
            "num": "0.3.12",
            "dl_path": "/api/v1/crates/rand/0.3.12/download",
            "readme_path": "/api/v1/crates/rand/0.3.12/readme",
            "updated_at": "2017-11-30T02:29:39.167151+00:00",
            "created_at": "2015-11-09T15:57:54.952164+00:00",
            "downloads": 183028,
            "features": {},
            "yanked": false,
            "license": "MIT/Apache-2.0",
            "links": {
                "dependencies": "/api/v1/crates/rand/0.3.12/dependencies",
                "version_downloads": "/api/v1/crates/rand/0.3.12/downloads",
                "authors": "/api/v1/crates/rand/0.3.12/authors"
            },
            "crate_size": null,
            "published_by": null,
            "audit_actions": []
        },
        {
            "id": 15268,
            "crate": "rand",
            "num": "0.3.11",
            "dl_path": "/api/v1/crates/rand/0.3.11/download",
            "readme_path": "/api/v1/crates/rand/0.3.11/readme",
            "updated_at": "2017-11-30T03:28:50.400876+00:00",
            "created_at": "2015-08-31T06:12:15.702270+00:00",
            "downloads": 155338,
            "features": {},
            "yanked": false,
            "license": "MIT/Apache-2.0",
            "links": {
                "dependencies": "/api/v1/crates/rand/0.3.11/dependencies",
                "version_downloads": "/api/v1/crates/rand/0.3.11/downloads",
                "authors": "/api/v1/crates/rand/0.3.11/authors"
            },
            "crate_size": null,
            "published_by": null,
            "audit_actions": []
        },
        {
            "id": 14687,
            "crate": "rand",
            "num": "0.3.10",
            "dl_path": "/api/v1/crates/rand/0.3.10/download",
            "readme_path": "/api/v1/crates/rand/0.3.10/readme",
            "updated_at": "2017-11-30T02:49:18.256536+00:00",
            "created_at": "2015-08-17T05:06:53.870047+00:00",
            "downloads": 34066,
            "features": {},
            "yanked": false,
            "license": "MIT/Apache-2.0",
            "links": {
                "dependencies": "/api/v1/crates/rand/0.3.10/dependencies",
                "version_downloads": "/api/v1/crates/rand/0.3.10/downloads",
                "authors": "/api/v1/crates/rand/0.3.10/authors"
            },
            "crate_size": null,
            "published_by": null,
            "audit_actions": []
        },
        {
            "id": 13851,
            "crate": "rand",
            "num": "0.3.9",
            "dl_path": "/api/v1/crates/rand/0.3.9/download",
            "readme_path": "/api/v1/crates/rand/0.3.9/readme",
            "updated_at": "2017-11-30T03:23:24.915468+00:00",
            "created_at": "2015-07-30T00:18:14.527848+00:00",
            "downloads": 45242,
            "features": {},
            "yanked": false,
            "license": "MIT/Apache-2.0",
            "links": {
                "dependencies": "/api/v1/crates/rand/0.3.9/dependencies",
                "version_downloads": "/api/v1/crates/rand/0.3.9/downloads",
                "authors": "/api/v1/crates/rand/0.3.9/authors"
            },
            "crate_size": null,
            "published_by": null,
            "audit_actions": []
        },
        {
            "id": 9238,
            "crate": "rand",
            "num": "0.3.8",
            "dl_path": "/api/v1/crates/rand/0.3.8/download",
            "readme_path": "/api/v1/crates/rand/0.3.8/readme",
            "updated_at": "2017-11-30T03:10:46.675998+00:00",
            "created_at": "2015-04-23T15:45:27.537203+00:00",
            "downloads": 131341,
            "features": {},
            "yanked": false,
            "license": "MIT/Apache-2.0",
            "links": {
                "dependencies": "/api/v1/crates/rand/0.3.8/dependencies",
                "version_downloads": "/api/v1/crates/rand/0.3.8/downloads",
                "authors": "/api/v1/crates/rand/0.3.8/authors"
            },
            "crate_size": null,
            "published_by": null,
            "audit_actions": []
        },
        {
            "id": 7843,
            "crate": "rand",
            "num": "0.3.7",
            "dl_path": "/api/v1/crates/rand/0.3.7/download",
            "readme_path": "/api/v1/crates/rand/0.3.7/readme",
            "updated_at": "2017-11-30T03:29:52.524963+00:00",
            "created_at": "2015-04-03T01:05:31.952768+00:00",
            "downloads": 20439,
            "features": {},
            "yanked": false,
            "license": "MIT/Apache-2.0",
            "links": {
                "dependencies": "/api/v1/crates/rand/0.3.7/dependencies",
                "version_downloads": "/api/v1/crates/rand/0.3.7/downloads",
                "authors": "/api/v1/crates/rand/0.3.7/authors"
            },
            "crate_size": null,
            "published_by": null,
            "audit_actions": []
        },
        {
            "id": 7771,
            "crate": "rand",
            "num": "0.3.6",
            "dl_path": "/api/v1/crates/rand/0.3.6/download",
            "readme_path": "/api/v1/crates/rand/0.3.6/readme",
            "updated_at": "2017-11-30T03:29:50.862185+00:00",
            "created_at": "2015-04-02T16:19:41.260649+00:00",
            "downloads": 2078,
            "features": {},
            "yanked": false,
            "license": "MIT/Apache-2.0",
            "links": {
                "dependencies": "/api/v1/crates/rand/0.3.6/dependencies",
                "version_downloads": "/api/v1/crates/rand/0.3.6/downloads",
                "authors": "/api/v1/crates/rand/0.3.6/authors"
            },
            "crate_size": null,
            "published_by": null,
            "audit_actions": []
        },
        {
            "id": 7710,
            "crate": "rand",
            "num": "0.3.5",
            "dl_path": "/api/v1/crates/rand/0.3.5/download",
            "readme_path": "/api/v1/crates/rand/0.3.5/readme",
            "updated_at": "2017-11-30T03:29:50.860086+00:00",
            "created_at": "2015-04-01T16:31:09.324585+00:00",
            "downloads": 2497,
            "features": {},
            "yanked": false,
            "license": "MIT/Apache-2.0",
            "links": {
                "dependencies": "/api/v1/crates/rand/0.3.5/dependencies",
                "version_downloads": "/api/v1/crates/rand/0.3.5/downloads",
                "authors": "/api/v1/crates/rand/0.3.5/authors"
            },
            "crate_size": null,
            "published_by": null,
            "audit_actions": []
        },
        {
            "id": 7648,
            "crate": "rand",
            "num": "0.3.4",
            "dl_path": "/api/v1/crates/rand/0.3.4/download",
            "readme_path": "/api/v1/crates/rand/0.3.4/readme",
            "updated_at": "2017-11-30T02:22:43.660971+00:00",
            "created_at": "2015-03-31T16:27:07.045712+00:00",
            "downloads": 2426,
            "features": {},
            "yanked": false,
            "license": "MIT/Apache-2.0",
            "links": {
                "dependencies": "/api/v1/crates/rand/0.3.4/dependencies",
                "version_downloads": "/api/v1/crates/rand/0.3.4/downloads",
                "authors": "/api/v1/crates/rand/0.3.4/authors"
            },
            "crate_size": null,
            "published_by": null,
            "audit_actions": []
        },
        {
            "id": 7248,
            "crate": "rand",
            "num": "0.3.3",
            "dl_path": "/api/v1/crates/rand/0.3.3/download",
            "readme_path": "/api/v1/crates/rand/0.3.3/readme",
            "updated_at": "2017-11-30T03:20:02.518029+00:00",
            "created_at": "2015-03-26T16:51:30.584466+00:00",
            "downloads": 5866,
            "features": {},
            "yanked": false,
            "license": "MIT/Apache-2.0",
            "links": {
                "dependencies": "/api/v1/crates/rand/0.3.3/dependencies",
                "version_downloads": "/api/v1/crates/rand/0.3.3/downloads",
                "authors": "/api/v1/crates/rand/0.3.3/authors"
            },
            "crate_size": null,
            "published_by": null,
            "audit_actions": []
        },
        {
            "id": 7239,
            "crate": "rand",
            "num": "0.3.2",
            "dl_path": "/api/v1/crates/rand/0.3.2/download",
            "readme_path": "/api/v1/crates/rand/0.3.2/readme",
            "updated_at": "2017-11-30T02:52:48.647396+00:00",
            "created_at": "2015-03-26T16:27:26.614515+00:00",
            "downloads": 1624,
            "features": {},
            "yanked": false,
            "license": "MIT/Apache-2.0",
            "links": {
                "dependencies": "/api/v1/crates/rand/0.3.2/dependencies",
                "version_downloads": "/api/v1/crates/rand/0.3.2/downloads",
                "authors": "/api/v1/crates/rand/0.3.2/authors"
            },
            "crate_size": null,
            "published_by": null,
            "audit_actions": []
        },
        {
            "id": 7237,
            "crate": "rand",
            "num": "0.3.1",
            "dl_path": "/api/v1/crates/rand/0.3.1/download",
            "readme_path": "/api/v1/crates/rand/0.3.1/readme",
            "updated_at": "2017-11-30T03:23:33.647509+00:00",
            "created_at": "2015-03-26T16:22:02.670733+00:00",
            "downloads": 1629,
            "features": {},
            "yanked": false,
            "license": "MIT/Apache-2.0",
            "links": {
                "dependencies": "/api/v1/crates/rand/0.3.1/dependencies",
                "version_downloads": "/api/v1/crates/rand/0.3.1/downloads",
                "authors": "/api/v1/crates/rand/0.3.1/authors"
            },
            "crate_size": null,
            "published_by": null,
            "audit_actions": []
        },
        {
            "id": 6964,
            "crate": "rand",
            "num": "0.3.0",
            "dl_path": "/api/v1/crates/rand/0.3.0/download",
            "readme_path": "/api/v1/crates/rand/0.3.0/readme",
            "updated_at": "2017-11-30T03:45:43.693804+00:00",
            "created_at": "2015-03-25T03:45:18.933975+00:00",
            "downloads": 3002,
            "features": {},
            "yanked": false,
            "license": "MIT/Apache-2.0",
            "links": {
                "dependencies": "/api/v1/crates/rand/0.3.0/dependencies",
                "version_downloads": "/api/v1/crates/rand/0.3.0/downloads",
                "authors": "/api/v1/crates/rand/0.3.0/authors"
            },
            "crate_size": null,
            "published_by": null,
            "audit_actions": []
        },
        {
            "id": 6704,
            "crate": "rand",
            "num": "0.2.1",
            "dl_path": "/api/v1/crates/rand/0.2.1/download",
            "readme_path": "/api/v1/crates/rand/0.2.1/readme",
            "updated_at": "2017-11-30T02:43:00.497770+00:00",
            "created_at": "2015-03-22T17:34:50.855059+00:00",
            "downloads": 7543,
            "features": {},
            "yanked": false,
            "license": "MIT/Apache-2.0",
            "links": {
                "dependencies": "/api/v1/crates/rand/0.2.1/dependencies",
                "version_downloads": "/api/v1/crates/rand/0.2.1/downloads",
                "authors": "/api/v1/crates/rand/0.2.1/authors"
            },
            "crate_size": null,
            "published_by": null,
            "audit_actions": []
        },
        {
            "id": 6029,
            "crate": "rand",
            "num": "0.2.0",
            "dl_path": "/api/v1/crates/rand/0.2.0/download",
            "readme_path": "/api/v1/crates/rand/0.2.0/readme",
            "updated_at": "2017-11-30T03:53:38.652065+00:00",
            "created_at": "2015-03-06T19:24:21.728280+00:00",
            "downloads": 7532,
            "features": {},
            "yanked": false,
            "license": "MIT/Apache-2.0",
            "links": {
                "dependencies": "/api/v1/crates/rand/0.2.0/dependencies",
                "version_downloads": "/api/v1/crates/rand/0.2.0/downloads",
                "authors": "/api/v1/crates/rand/0.2.0/authors"
            },
            "crate_size": null,
            "published_by": null,
            "audit_actions": []
        },
        {
            "id": 5937,
            "crate": "rand",
            "num": "0.1.4",
            "dl_path": "/api/v1/crates/rand/0.1.4/download",
            "readme_path": "/api/v1/crates/rand/0.1.4/readme",
            "updated_at": "2017-11-30T03:37:01.444909+00:00",
            "created_at": "2015-03-04T17:43:01.186628+00:00",
            "downloads": 14658,
            "features": {},
            "yanked": false,
            "license": "MIT/Apache-2.0",
            "links": {
                "dependencies": "/api/v1/crates/rand/0.1.4/dependencies",
                "version_downloads": "/api/v1/crates/rand/0.1.4/downloads",
                "authors": "/api/v1/crates/rand/0.1.4/authors"
            },
            "crate_size": null,
            "published_by": null,
            "audit_actions": []
        },
        {
            "id": 5252,
            "crate": "rand",
            "num": "0.1.3",
            "dl_path": "/api/v1/crates/rand/0.1.3/download",
            "readme_path": "/api/v1/crates/rand/0.1.3/readme",
            "updated_at": "2017-11-30T03:19:24.310777+00:00",
            "created_at": "2015-02-20T17:51:03.914107+00:00",
            "downloads": 7758,
            "features": {},
            "yanked": false,
            "license": "MIT/Apache-2.0",
            "links": {
                "dependencies": "/api/v1/crates/rand/0.1.3/dependencies",
                "version_downloads": "/api/v1/crates/rand/0.1.3/downloads",
                "authors": "/api/v1/crates/rand/0.1.3/authors"
            },
            "crate_size": null,
            "published_by": null,
            "audit_actions": []
        },
        {
            "id": 4371,
            "crate": "rand",
            "num": "0.1.2",
            "dl_path": "/api/v1/crates/rand/0.1.2/download",
            "readme_path": "/api/v1/crates/rand/0.1.2/readme",
            "updated_at": "2017-11-30T03:14:27.545115+00:00",
            "created_at": "2015-02-03T11:15:19.001762+00:00",
            "downloads": 7001,
            "features": {},
            "yanked": false,
            "license": "MIT/Apache-2.0",
            "links": {
                "dependencies": "/api/v1/crates/rand/0.1.2/dependencies",
                "version_downloads": "/api/v1/crates/rand/0.1.2/downloads",
                "authors": "/api/v1/crates/rand/0.1.2/authors"
            },
            "crate_size": null,
            "published_by": null,
            "audit_actions": []
        },
        {
            "id": 4362,
            "crate": "rand",
            "num": "0.1.1",
            "dl_path": "/api/v1/crates/rand/0.1.1/download",
            "readme_path": "/api/v1/crates/rand/0.1.1/readme",
            "updated_at": "2017-11-30T03:33:14.186028+00:00",
            "created_at": "2015-02-03T06:17:14.169972+00:00",
            "downloads": 1663,
            "features": {},
            "yanked": false,
            "license": "MIT/Apache-2.0",
            "links": {
                "dependencies": "/api/v1/crates/rand/0.1.1/dependencies",
                "version_downloads": "/api/v1/crates/rand/0.1.1/downloads",
                "authors": "/api/v1/crates/rand/0.1.1/authors"
            },
            "crate_size": null,
            "published_by": null,
            "audit_actions": []
        }
    ],
    "keywords": [
        {
            "id": "random",
            "keyword": "random",
            "created_at": "2014-11-21T00:22:50.038243+00:00",
            "crates_cnt": 171
        },
        {
            "id": "rng",
            "keyword": "rng",
            "created_at": "2015-02-02T03:37:04.452064+00:00",
            "crates_cnt": 58
        }
    ],
    "categories": [
        {
            "id": "no-std",
            "category": "No standard library",
            "slug": "no-std",
            "description": "Crates that are able to function without the Rust standard library.\n",
            "created_at": "2017-02-10T01:52:09.447906+00:00",
            "crates_cnt": 2300
        },
        {
            "id": "algorithms",
            "category": "Algorithms",
            "slug": "algorithms",
            "description": "Rust implementations of core algorithms such as hashing, sorting, searching, and more.",
            "created_at": "2017-01-17T19:13:05.112025+00:00",
            "crates_cnt": 999
        }
    ]
}
"#;
}
