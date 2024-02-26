//#![warn(clippy::pedantic)]
use clap::{Parser, Subcommand};
use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::Read;

#[derive(Serialize, Deserialize, Clone)]
pub struct UnityJSON {
    pub asset: String,
    pub stuff: Vec<String>,
}
impl UnityJSON {
    fn to_markdown(&self) -> String {
        let mut md = String::new();
        md.push_str("# ");
        md.push_str(&self.asset);
        md.push('\n');
        for item in &self.stuff {
            md.push_str(" - ");
            md.push_str(item);
            md.push('\n');
        }
        md
    }
}

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand, Debug)]
enum Commands {
    /// Give me a JSON file, and I'll give you markdown
    ToMarkdown {
        /// Path to a JSON file
        json_path: String,
    },
}

fn main() {
    let cli = Cli::parse();

    if let Some(command) = cli.command {
        match command {
            Commands::ToMarkdown { json_path } => process_json_to_markdown(&json_path),
        }
    }
}

fn process_json_to_markdown(json_path: &String) {
    let mut json_contents = String::new();
    let mut file = File::open(json_path).unwrap_or_else(|err| panic!("{err} {json_path}"));
    file.read_to_string(&mut json_contents)
        .unwrap_or_else(|err| panic!("Could not read json contents: {err}"));
    let unity_json: UnityJSON = serde_json::from_str(&json_contents)
        .unwrap_or_else(|err| panic!("Could not parse json: {err}"));
    println!("{}", unity_json.to_markdown());
}
