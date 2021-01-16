use crate::errors::RustKataResult;

#[async_trait::async_trait]
pub trait CrateRegistry {
    async fn get_crate(&self, crate_name: &str) -> RustKataResult<get_crate::Response>;
    async fn get_crate_dependencies(
        &self,
        crate_name: &str,
        crate_version: &str,
    ) -> RustKataResult<get_crate_dependencies::Response>;
}

pub mod get_crate {
    use std::collections::HashMap;

    use serde::{Deserialize, Serialize};

    #[derive(Debug, Deserialize, Serialize)]
    pub struct Response {
        #[serde(rename = "crate")]
        pub crate_: CrateResponse,
        #[serde(rename = "versions")]
        pub versions: Vec<VersionResponse>,
        #[serde(rename = "keywords")]
        pub keywords: Vec<KeywordResponse>,
        #[serde(rename = "categories")]
        pub categories: Vec<CategoryResponse>,
    }

    #[derive(Debug, Deserialize, Serialize)]
    pub struct CrateResponse {
        #[serde(rename = "id")]
        pub id: String,
        #[serde(rename = "name")]
        pub name: String,
        #[serde(rename = "updated_at")]
        pub updated_at: String,
        #[serde(rename = "versions")]
        pub versions: Vec<i64>,
        #[serde(rename = "keywords")]
        pub keywords: Vec<String>,
        #[serde(rename = "categories")]
        pub categories: Vec<String>,
        #[serde(rename = "badges")]
        pub badges: Vec<CrateBadgeResponse>,
        #[serde(rename = "created_at")]
        pub created_at: String,
        #[serde(rename = "downloads")]
        pub downloads: i64,
        #[serde(rename = "recent_downloads")]
        pub recent_downloads: i64,
        #[serde(rename = "max_version")]
        pub max_version: String,
        #[serde(rename = "newest_version")]
        pub newest_version: String,
        #[serde(rename = "description")]
        pub description: String,
        #[serde(rename = "homepage")]
        pub homepage: Option<String>,
        #[serde(rename = "documentation")]
        pub documentation: Option<String>,
        #[serde(rename = "repository")]
        pub repository: String,
        #[serde(rename = "links")]
        pub links: CrateLinksResponse,
        #[serde(rename = "exact_match")]
        pub exact_match: bool,
    }

    #[derive(Debug, Deserialize, Serialize)]
    pub struct CrateBadgeResponse {
        #[serde(rename = "badge_type")]
        pub badge_type: String,
        #[serde(rename = "attributes")]
        pub attributes: CrateBadgeAttributesResponse,
    }

    #[derive(Debug, Deserialize, Serialize)]
    pub struct CrateBadgeAttributesResponse {
        #[serde(rename = "service")]
        pub service: Option<String>,
        #[serde(rename = "repository")]
        pub repository: String,
        #[serde(rename = "branch")]
        pub branch: String,
    }

    #[derive(Debug, Deserialize, Serialize)]
    pub struct CrateLinksResponse {
        #[serde(rename = "version_downloads")]
        pub version_downloads: String,
        #[serde(rename = "versions")]
        pub versions: Option<String>,
        #[serde(rename = "owners")]
        pub owners: String,
        #[serde(rename = "owner_team")]
        pub owner_team: String,
        #[serde(rename = "owner_user")]
        pub owner_user: String,
        #[serde(rename = "reverse_dependencies")]
        pub reverse_dependencies: String,
    }

    #[derive(Debug, Deserialize, Serialize)]
    pub struct VersionResponse {
        #[serde(rename = "id")]
        pub id: i64,
        #[serde(rename = "crate")]
        pub crate_: String,
        #[serde(rename = "num")]
        pub num: String,
        #[serde(rename = "dl_path")]
        pub dl_path: String,
        #[serde(rename = "readme_path")]
        pub readme_path: String,
        #[serde(rename = "updated_at")]
        pub updated_at: String,
        #[serde(rename = "created_at")]
        pub created_at: String,
        #[serde(rename = "downloads")]
        pub downloads: i64,
        #[serde(rename = "features")]
        pub features: HashMap<String, Vec<String>>,
        #[serde(rename = "yanked")]
        pub yanked: bool,
        #[serde(rename = "license")]
        pub license: String,
        #[serde(rename = "links")]
        pub links: VersionLinksResponse,
        #[serde(rename = "crate_size")]
        pub crate_size: Option<i64>,
        #[serde(rename = "published_by")]
        pub published_by: Option<UserResponse>,
        #[serde(rename = "audit_actions")]
        pub audit_actions: Vec<VersionAuditActionResponse>,
    }

    #[derive(Debug, Deserialize, Serialize)]
    pub struct VersionLinksResponse {
        #[serde(rename = "dependencies")]
        pub dependencies: String,
        #[serde(rename = "version_downloads")]
        pub version_downloads: String,
        #[serde(rename = "authors")]
        pub authors: String,
    }

    #[derive(Debug, Deserialize, Serialize)]
    pub struct VersionAuditActionResponse {
        #[serde(rename = "action")]
        pub action: String,
        #[serde(rename = "user")]
        pub user: UserResponse,
        #[serde(rename = "time")]
        pub time: String,
    }

    #[derive(Debug, Deserialize, Serialize)]
    pub struct UserResponse {
        #[serde(rename = "id")]
        pub id: i64,
        #[serde(rename = "login")]
        pub login: String,
        #[serde(rename = "name")]
        pub name: Option<String>,
        #[serde(rename = "avatar")]
        pub avatar: String,
        #[serde(rename = "url")]
        pub url: String,
    }

    #[derive(Debug, Deserialize, Serialize)]
    pub struct KeywordResponse {
        #[serde(rename = "id")]
        pub id: String,
        #[serde(rename = "keyword")]
        pub keyword: String,
        #[serde(rename = "created_at")]
        pub created_at: String,
        #[serde(rename = "crates_cnt")]
        pub crates_cnt: i64,
    }

    #[derive(Debug, Deserialize, Serialize)]
    pub struct CategoryResponse {
        #[serde(rename = "id")]
        pub id: String,
        #[serde(rename = "category")]
        pub category: String,
        #[serde(rename = "slug")]
        pub slug: String,
        #[serde(rename = "description")]
        pub description: String,
        #[serde(rename = "created_at")]
        pub created_at: String,
        #[serde(rename = "crates_cnt")]
        pub crates_cnt: i64,
    }
}

pub mod get_crate_dependencies {
    use serde::{Deserialize, Serialize};

    #[derive(Debug, Deserialize, Serialize)]
    pub struct Response {
        #[serde(rename = "dependencies")]
        pub dependencies: Vec<DependencyResponse>,
    }

    #[derive(Debug, Deserialize, Serialize)]
    pub struct DependencyResponse {
        #[serde(rename = "id")]
        pub id: i64,
        #[serde(rename = "version_id")]
        pub version_id: i64,
        #[serde(rename = "crate_id")]
        pub crate_id: String,
        #[serde(rename = "req")]
        pub req: String,
        #[serde(rename = "optional")]
        pub optional: bool,
        #[serde(rename = "default_features")]
        pub default_features: bool,
        #[serde(rename = "features")]
        pub features: Option<Vec<String>>,
        #[serde(rename = "target")]
        pub target: Option<String>,
        #[serde(rename = "kind")]
        pub kind: String,
        #[serde(rename = "downloads")]
        pub downloads: i64,
    }
}
