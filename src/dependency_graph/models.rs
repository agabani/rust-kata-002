use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub(crate) struct QueryParams {
    pub(crate) name: Option<String>,
    pub(crate) version: Option<String>,
}

#[derive(Deserialize, Serialize)]
pub(crate) struct QueryResult {
    #[serde(skip_serializing_if = "Option::is_none", rename = "data")]
    pub(crate) data: Option<Vec<Node>>,
}

#[derive(Deserialize, Serialize)]
pub(crate) struct Node {
    #[serde(rename = "name")]
    pub(crate) name: String,
    #[serde(rename = "version")]
    pub(crate) version: String,
    #[serde(skip_serializing_if = "Option::is_none", rename = "edges")]
    pub(crate) edges: Option<Vec<Edge>>,
}

#[derive(Deserialize, Serialize)]
pub(crate) struct Edge {
    #[serde(rename = "relationship")]
    pub(crate) relationship: String,
    #[serde(rename = "node")]
    pub(crate) node: Node,
}
