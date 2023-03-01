use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MarketplaceResponse {
    pub results: Vec<Result>,
}

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Result {
    pub extensions: Vec<Extension>,
    pub paging_token: Option<Value>,
    pub result_metadata: Vec<ResultMetadaum>,
}

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Extension {
    pub publisher: Publisher,
    pub extension_id: String,
    pub extension_name: String,
    pub display_name: String,
    pub flags: String,
    pub last_updated: String,
    pub published_date: String,
    pub release_date: String,
    pub short_description: String,
    pub versions: Vec<Version>,
    pub categories: Vec<String>,
    pub tags: Vec<String>,
    pub installation_targets: Vec<InstallationTarget>,
    pub deployment_type: i64,
}

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Publisher {
    pub publisher_id: String,
    pub publisher_name: String,
    pub display_name: String,
    pub flags: String,
    pub domain: String,
    pub is_domain_verified: bool,
}

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Version {
    pub version: String,
    pub target_platform: Option<String>,
    pub flags: String,
    pub last_updated: String,
    pub files: Vec<File>,
    pub asset_uri: String,
    pub fallback_asset_uri: String,
}

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct File {
    pub asset_type: String,
    pub source: String,
}

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct InstallationTarget {
    pub target: String,
    pub target_version: String,
}

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ResultMetadaum {
    pub metadata_type: String,
    pub metadata_items: Vec<MetadataItem>,
}

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MetadataItem {
    pub name: String,
    pub count: i64,
}