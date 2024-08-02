use std::fs;

use clap::{Parser, Subcommand};
use serde::{Deserialize, Serialize};

#[derive(Debug, Parser)]
#[command(author = "Matter Labs", version, about = "Proxy", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Command,
    #[arg(long, default_value = "8015")]
    /// Port to listen on - default: 8011
    port: u16,

    #[arg(long)]
    sequencer_url: String,

    #[arg(long, default_value = "config.yaml")]
    config_file_path: String,
}

#[derive(Debug, Subcommand)]
enum Command {
    #[command(name = "run")]
    Run,
}

#[derive(Serialize, Deserialize, Debug)]
struct Config {
    // If enabled, allows anyone to create new contracts.
    allow_contract_creation: bool,

    // List of contracts that can be called with 'call'.
    contract_call_whitelist: Vec<String>,
}

fn parse_config(path: &str) -> eyre::Result<Config> {
    let file_content = fs::read_to_string(path).expect("Unable to read file");

    // Parse the YAML string into the Config struct
    let config: Config = serde_yaml::from_str(&file_content)?;

    return Ok(config);
}

fn main() {
    let opt = Cli::parse();
    let config = parse_config(&opt.config_file_path).unwrap();
    println!("config: {:?}", config);
}
