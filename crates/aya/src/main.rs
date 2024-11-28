use std::{fs::File, io::BufReader, process};

use clap::Parser;

mod task;

#[derive(Parser, Debug)]
#[command(
    name = "Aya Runner",
    version = "0.1.0 Highly Responsive to Prayers",
    about = "A configurable runner fits in online judge tasks."
)]
struct Args {
    #[arg(short, long, value_name = "FILE", default_value = "./config.json")]
    /// The path to the config file
    config: std::path::PathBuf,

    #[arg(short, long, value_name = "DIRECTORY", default_value = "./")]
    /// The path to the working directory
    path: std::path::PathBuf,

    #[arg(short, long, value_name = "RESULT_TYPE", default_value = "SIMPLE")]
    /// The result type of the task
    result_type: String,

    #[arg(short, long, value_name = "FILE", default_value = "true")]
    /// Decide whether to print the task log into stderr stream
    log: bool,
}

enum ConfigType {
    Json,
    Yaml,
    Toml,
}

fn main() {
    let args = Args::parse();

    let config_path = args.config;

    let log = args.log;

    let config_type = match config_path
        .extension()
        .unwrap_or_else(|| {
            eprintln!("Problem parsing config arguments");
            process::exit(1);
        })
        .to_str()
        .unwrap_or_else(|| {
            eprintln!("Problem parsing config arguments");
            process::exit(1);
        }) {
        "json" => ConfigType::Json,
        "yaml" => ConfigType::Yaml,
        "toml" => ConfigType::Toml,
        _ => ConfigType::Json,
    };

    let working_dir = args.path;

    let config_raw = File::open(config_path).unwrap_or_else(|err| {
        eprintln!("Problem opening config file with error: {err}");
        process::exit(1);
    });

    let config_reader = BufReader::new(config_raw);

    //let config = match config_type 
}
