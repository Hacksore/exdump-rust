#![allow(clippy::needless_return)]
use clap::{arg, command, Parser};
use util::get_extension_metadata;

use crate::{util::get_vscode_extensions};

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

  // test getting data
  let extension_data = get_extension_metadata(String::from("tauri-apps.tauri-vscode")).await?;

  // check results
  if let Some(data) = extension_data {
    // get first result
    if let Some(result) = data.results.get(0) {     
      let first_extension = result.extensions.get(0);
      // check if the first one is valid; 
      if let Some(extension) = first_extension {     
        println!("{:?}", extension.display_name);
      }
    }
  }

  Ok(())
}
