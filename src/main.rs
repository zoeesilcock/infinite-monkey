use std::env;
use std::fs::File;
use std::process;

use crate::cli::{CliConfig, DebugLogger};
use crate::config::Config;
use crate::generate::generate_rows;

mod cli;
mod config;
mod generate;

fn main() {
    let cli_config = CliConfig::build(env::args()).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {err}");
        process::exit(1);
    });
    let debug_logger = DebugLogger::new(cli_config.debug_output);

    debug_logger.print(format!("\n\nCLI arguments: {:?}", cli_config));

    let config_file = std::fs::read_to_string(cli_config.config_path).unwrap_or_else(|err| {
        eprintln!("Problem reading config file: {err}");
        process::exit(1);
    });
    let config: Config = toml::from_str(&config_file[..]).unwrap_or_else(|err| {
        eprintln!("Problem parsing config file: {err}");
        process::exit(1);
    });

    debug_logger.print(format!("\n\nConfig: {:?}", config));

    let fake_data = generate_rows(&config);
    let file = File::create(cli_config.output_path).unwrap_or_else(|err| {
        eprintln!("Problem creating output file: {err}");
        process::exit(1);
    });
    serde_json::to_writer_pretty(file, &fake_data).unwrap_or_else(|err| {
        eprintln!("Problem serializing output data: {err}");
        process::exit(1);
    });

    println!(
        "Successfully generated {} rows with {} columns each, based on {} pools.",
        fake_data.data.len(),
        config.columns.len(),
        config.pools.len(),
    );
}
