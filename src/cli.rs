#[derive(Debug)]
pub struct CliConfig {
    pub config_path: String,
    pub output_path: String,
    pub debug_output: bool,
}

impl CliConfig {
    pub fn new() -> CliConfig {
        CliConfig {
            config_path: "config.toml".to_string(),
            output_path: "fake_data.json".to_string(),
            debug_output: false,
        }
    }

    pub fn build(mut args: impl Iterator<Item = String>) -> Result<CliConfig, &'static str> {
        args.next();

        let mut cli_config = CliConfig::new();

        while let Some(arg) = args.next() {
            match arg {
                arg if arg == "-c" || arg == "--config" => {
                    cli_config.config_path = match args.next() {
                        Some(arg) => arg,
                        None => return Err("No config path provided for -c / --config."),
                    }
                }
                arg if arg == "-o" || arg == "--output" => {
                    cli_config.output_path = match args.next() {
                        Some(arg) => arg,
                        None => return Err("No output path provided for -o / --output."),
                    }
                }
                arg if arg == "-d" || arg == "--debug" => {
                    cli_config.debug_output = true;
                }
                _ => {}
            }
        }

        Ok(cli_config)
    }
}

pub struct DebugLogger {
    enable_debug: bool,
}

impl DebugLogger {
    pub fn new(enable_debug: bool) -> DebugLogger {
        DebugLogger { enable_debug }
    }

    pub fn print(&self, message: String) {
        if self.enable_debug {
            println!("{}", message);
        }
    }
}
