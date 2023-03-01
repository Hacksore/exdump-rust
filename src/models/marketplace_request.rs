
use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MarketplaceRequest {
  pub filters: Vec<Filter>,
  pub flags: i64,
}

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Filter {
  pub criteria: Vec<Criterum>,
  pub direction: i64,
  pub page_size: i64,
  pub page_number: i64,
  pub sort_by: i64,
  pub sort_order: i64,
}

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Criterum {
  pub filter_type: i64,
  pub value: String,
}
