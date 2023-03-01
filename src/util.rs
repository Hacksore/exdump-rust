use std::process::{Command, Stdio};

pub fn get_vscode_extensions() -> Vec<String> {
  let output = Command::new("code")
    .arg("--list-extensions")
    .stdout(Stdio::piped())
    .output()
    .expect("Failed to run command");

  let output_string = String::from_utf8(output.stdout).expect("Invalid UTF-8 output");

  output_string.lines().map(|s| s.to_string()).collect()
}
