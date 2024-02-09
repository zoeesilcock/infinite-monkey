use std::env;
use std::error::Error;
use std::fs::File;
use std::process;

use crate::cli::CliConfig;
use crate::config::Config;
use crate::generate::generate_rows;

mod cli;
mod config;
mod generate;

fn main() -> Result<(), Box<dyn Error>> {
    let cli_config = CliConfig::build(env::args()).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {err}");
        process::exit(1);
    });

    if cli_config.debug_output {
        println!("\n\nCLI arguments: {:?}", cli_config);
    }

    let config_file = std::fs::read_to_string(cli_config.config_path)?;
    let config: Config = toml::from_str(&config_file[..]).unwrap();

    if cli_config.debug_output {
        println!("\n\nConfig: {:?}", config);
    }

    let fake_data = generate_rows(&config);

    if cli_config.debug_output {
        let json = serde_json::to_string_pretty(&fake_data).unwrap();
        println!("\n\nOutput: {}", json);
    }

    let file = File::create(cli_config.output_path).unwrap();
    serde_json::to_writer_pretty(file, &fake_data)?;

    println!(
        "Successfully generated {} rows with {} columns each, based on {} pools.",
        fake_data.data.len(),
        config.columns.len(),
        config.pools.len(),
    );

    Ok(())
}
