#![allow(clippy::needless_return)]
use clap::{arg, command, Parser};
use models::marketplace_response::Extension;
use util::{get_extension_metadata, ExtensionMetadata};

use crate::{util::{get_vscode_extensions, generate_markdown_table}};

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
  let extensions_ids = get_vscode_extensions();
  let mut extensions: Vec<ExtensionMetadata> = Vec::new();

  for id in &extensions_ids {
    let extension_data = get_extension_metadata(String::from(id)).await?;
    if let Some(data) = extension_data {
      extensions.push(data);
    }
  }

  let table_data = generate_markdown_table(extensions);

  // output to the term
  println!("{}", table_data);

  // check if we have an option
  if let Some(f) = args.file {
    println!("Found file: {}", f);
  }


  Ok(())
}
