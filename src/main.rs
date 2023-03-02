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
  for id in &extensions {
    // test getting data
    let extension_data = get_extension_metadata(String::from(id)).await?;

    if let Some(ext) = extension_data {
      println!("{:#?}", ext)
    }
  }

  // check if we have an option
  if let Some(f) = args.file {
    println!("Found file: {}", f);
  }


  Ok(())
}
