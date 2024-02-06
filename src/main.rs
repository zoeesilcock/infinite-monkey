use crate::config::Config;
use crate::generate::generate_rows;
use std::fs::File;

mod config;
mod generate;

fn main() -> serde_json::Result<()> {
    let config_file = std::fs::read_to_string("example_config.toml").unwrap();
    let config: Config = toml::from_str(&config_file[..]).unwrap();

    println!("{:?}", config);

    let fake_data = generate_rows();

    // let json = serde_json::to_string_pretty(&fake_data).unwrap();
    // println!("{}", json);

    let file = File::create("fake_data.json").unwrap();
    serde_json::to_writer_pretty(file, &fake_data)
}
