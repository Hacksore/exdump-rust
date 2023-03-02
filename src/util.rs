use std::process::{Command, Stdio};

use crate::models::marketplace_request::{Criterum, Filter, MarketplaceRequest};
use crate::models::marketplace_response::{Extension, MarketplaceResponse};

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

#[derive(Debug)]
pub struct ExtensionMetadata {
  pub id: String,
  pub display_name: String,
  pub latest_version: String,
  pub url: String,
  pub desc: String,
}

pub async fn get_extension_metadata(id: String) -> Result<Option<ExtensionMetadata>, surf::Error> {
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

  let result: Option<MarketplaceResponse> = res.body_json().await?;

  // TODO: optimize with d0nut
  if let Some(ext) = result {
    if let Some(ext) = ext.results.get(0) {
      if let Some(ext) = ext.extensions.get(0) {        
        let thing = ExtensionMetadata {
          // TODO: d0nut
          id: ext.extension_name.clone(),
          display_name: ext.display_name.clone(),
          desc: ext.short_description.clone(),
          // TODO: more d0nut
          latest_version: match ext.versions.get(0) {
            Some(data) => data.version.clone(),
            None => String::from("unknown"),
        },
          url: create_link(ext.clone()),
        };
        return Ok(Some(thing));
      }
    }
  }

  Ok(None)
}

pub fn create_link(ext: Extension) -> String {
  format!(
    "https://marketplace.visualstudio.com/items?itemName={}",
    get_extension_id(ext)
  )
}

pub fn get_extension_id(extension: Extension) -> String {
  return format!(
    "{}.{}",
    extension.publisher.publisher_name, extension.extension_name
  );
}

pub fn generate_markdown_table(extensions: Vec<ExtensionMetadata>) -> String {
  let mut output = String::from("");
  output.push_str("| Icon | Extension |\n ---- | ---- |\n");

  for ext in &extensions {
    output.push_str(&format!("| {} | {} |\n", ext.display_name, ext.url));
  }

  return output;
}
