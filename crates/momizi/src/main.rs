use std::fs::File;
use std::io::{BufRead, BufReader};
use std::process::Command;

use clap::error::ErrorKind;
use clap::{CommandFactory, Parser, ValueEnum};

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum, Debug)]
enum OutputType {
    /// Assume the executable outputs in the STDOUT.
    STDOUT,
    /// Assume the executable outputs in a file.
    FILE,
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum, Debug)]
enum Mode {
    /// Receive the given result as boolean. (passed or not)
    BOOL,
    /// Receive the given score by percentage.
    SCORE,
    /// Compare the result.
    COMPARE,
}

#[derive(Parser, Debug)]
#[command(
    name = "Momizi Executer",
    version = "0.1.0 Highly Responsive to Prayers",
    about = "A small executer fits in judge tasks."
)]
struct Args {
    #[arg(short, long, value_enum, default_value = "stdout")]
    /// Decide where the executable will output.
    output: OutputType,

    #[arg(short, long, value_enum, default_value = "compare")]
    /// Decide how to receive the result.
    mode: Mode,

    #[arg(long)]
    /// Outputs the log in stderr or not.
    log: bool,

    #[arg(short, long)]
    /// The file executable produces.
    file: Option<String>,

    #[arg(short, long)]
    /// The answer for comparison.
    answer: Option<String>,

    #[arg()]
    /// The arg to execute.
    arg: String,
}

fn main() {
    let args = Args::parse();

    let mut cmd = Args::command();

    let child = Command::new("sh")
        .arg("-c")
        .arg(args.arg.as_str())
        .spawn()
        .unwrap();

    let _stdout = child.stdout.unwrap();

    match args.mode {
        Mode::COMPARE => match args.output {
            
            },
            OutputType::STDOUT => {
                let mut output = BufReader::new(_stdout);
                let ansreader = BufReader::new(
                    File::open(args.answer.unwrap_or_else(|| {
                        cmd.error(
                            ErrorKind::MissingRequiredArgument,
                            "Missing answer file for comparison",
                        )
                        .exit()
                    }))
                    .unwrap_or_else(|err| {
                        println!("File error {err}");
                        cmd.error(
                            ErrorKind::InvalidValue,
                            "Cannot read answer file for comparison",
                        )
                        .exit()
                    }),
                );
                let mut line = String::new();
                let mut flag = true;
                for aline in ansreader.lines() {
                    if output.read_line(&mut line).is_err() {
                        flag = false;
                        break;
                    }
                    if aline.unwrap() != line {
                        flag = false;
                        break;
                    }
                }
                eprintln!("{flag}");
            }
        },
        Mode::BOOL => {
            let res = match args.output {
                OutputType::FILE => {
                    let mut freader = BufReader::new(
                        File::open(args.file.unwrap_or_else(|| {
                            cmd.error(
                                ErrorKind::MissingRequiredArgument,
                                "Missing output file for comparison",
                            )
                            .exit()
                        }))
                        .unwrap_or_else(|err| {
                            println!("File error {err}");
                            cmd.error(
                                ErrorKind::InvalidValue,
                                "Cannot read output file for comparison",
                            )
                            .exit()
                        }),
                    );
                    let mut resRaw = String::new();
                    freader.read_line(&mut resRaw);
                    resRaw.to_ascii_lowercase() == "true"
                },
                OutputType::STDOUT => {
                    let mut output = BufReader::new(_stdout);
                    let mut resRaw = String::new();
                    output.read_line(&mut resRaw);
                    resRaw.to_ascii_lowercase() == "true"
                }
            };
            eprintln!("{res}");
        }
        Mode::SCORE => {
            // to be implement
        }
    }
}
