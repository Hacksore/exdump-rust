use std::process::{Command, Stdio};

use crate::models::marketplace_request::{Criterum, Filter, MarketplaceRequest};
use crate::models::marketplace_response::MarketplaceResponse;

const VSCODE_API_URL: &str =
  "https://marketplace.visualstudio.com/_apis/public/gallery/extensionquery";

pub fn get_vscode_extensions() -> Vec<String> {
  let output = Command::new("code")
    .arg("--list-extensions")
    .stdout(Stdio::piped())
    .output()
    .expect("Failed to run command");

  let output_string = String::from_utf8(output.stdout).expect("Invalid UTF-8 output");

  output_string.lines().map(|s| s.to_string()).collect()
}

pub async fn get_extension_metadata(
  id: String,
) -> Result<Option<MarketplaceResponse>, surf::Error> {
  // get data for first one
  let mut filters = vec![];
  let mut criteria_list: Vec<Criterum> = Vec::new();

  let criteria = Criterum {
    filter_type: 7,
    value: id,
  };

  criteria_list.push(criteria);

  filters.push(Filter {
    criteria: criteria_list,
    direction: 2,
    page_size: 100,
    page_number: 1,
    sort_by: 0,
    sort_order: 0,
  });

  let filter_data = &MarketplaceRequest {
    filters,
    flags: 2151,
  };

  let mut res = surf::post(VSCODE_API_URL)
    .header("Content-Type", "application/json")
    .header(
      "Accept",
      "application/json;api-version=7.1-preview.1;excludeUrls=true",
    )
    .body_json(filter_data)?
    .await?;

  // TODO: fix auto parse and not use serde directly
  let result: MarketplaceResponse = res.body_json().await?;

  Ok(Some(result))

}
