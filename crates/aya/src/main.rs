use clap::Parser;

#[derive(Parser, Debug)]
#[command(name = "Aya Runner",  version = "0.1.0 Highly Responsive to Prayers", about = "A configurable runner fits in online judge tasks.")]
struct Args {
    
    #[arg(short, long, value_name = "FILE", default_value = "./config.json")]
    /// The path to the config file
    config: std::path::PathBuf,

    #[arg(short, long, value_name = "DIRECTORY", default_value = "./")]
    /// The path to the working directory
    path: std::path::PathBuf,
}

enum ConfigType {
    Json,
    Yaml,
    Toml
}

fn main() {
    let args = Args::parse();

    let config_dir = args.config;

    let config_type = config_dir.extension().unwrap();

    let working_dir = args.path;

    let config_raw = std::fs::read_to_string(config_dir).unwrap();
    
}
