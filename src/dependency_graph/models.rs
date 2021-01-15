use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct QueryParams {
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "version")]
    pub version: String,
}

#[derive(Debug, Deserialize, PartialEq, Serialize)]
pub struct QueryResult {
    #[serde(skip_serializing_if = "Option::is_none", rename = "data")]
    pub data: Option<Vec<Node>>,
}

#[derive(Debug, Deserialize, PartialEq, Serialize)]
pub struct Node {
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "version")]
    pub version: String,
    #[serde(skip_serializing_if = "Option::is_none", rename = "edges")]
    pub edges: Option<Vec<Edge>>,
}

#[derive(Debug, Deserialize, PartialEq, Serialize)]
pub struct Edge {
    #[serde(rename = "relationship")]
    pub relationship: String,
    #[serde(rename = "node")]
    pub node: Node,
}
