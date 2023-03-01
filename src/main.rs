#![allow(clippy::needless_return)]
use clap::{arg, command, Parser};
use models::marketplace_request::Criterum;

use crate::models::marketplace_request::{Filter, MarketplaceRequest};
use crate::models::marketplace_response::{MarketplaceResponse};
use crate::util::get_vscode_extensions;

pub mod util;
pub mod models;

/// Simple program to greet a person
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
  /// Name of the files to output to
  #[arg(short, long)]
  file: Option<String>,
}

const VSCODE_API_URL: &str =
  "https://marketplace.visualstudio.com/_apis/public/gallery/extensionquery";

#[tokio::main]
async fn main() -> Result<(), surf::Error> {
  let args = Args::parse();
  let extensions = get_vscode_extensions();

  // list the extensions in a loop
  for i in &extensions {
    println!("{:?}", i);
  }

  // check if we have an option
  if let Some(f) = args.file {
    println!("Found file: {}", f);
  }

  // get data for first one
  let mut filters = vec![];
  let mut criteria_list: Vec<Criterum> = Vec::new();

  let criteria = Criterum {
    filter_type: 7,
    value: "zamerick.vscode-caddyfile-syntax".to_owned(),
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

  let MarketplaceResponse { results } = res.body_json().await?;
  println!("{:#?}", results.get(0));

  Ok(())
}
