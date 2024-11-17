use std::fs::File;
use std::io::{BufReader, Read, Seek};
use std::process::Command;

use clap::{Parser, ValueEnum};

pub mod util;

use crate::util::find_blankless_end;

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
    /// The answer file for comparison.
    answer: Option<String>,

    #[arg()]
    /// The arg to execute.
    arg: String,
}

fn main() {
    let args = Args::parse();

    // let mut cmd = Args::command();

    let child = Command::new("sh")
        .arg("-c")
        .arg(args.arg.as_str())
        .output()
        .expect("failed to spawn child process");

    let mut _stdout = String::from_utf8(child.stdout).unwrap_or_else(|_err| {
        eprintln!("[Error] invalid child output.");
        std::process::exit(1);
    });

    match args.mode {
        Mode::COMPARE => {
            let mut f = File::open(args.answer.unwrap_or_else(|| {
                eprintln!("[Error] No answer file provided for compare mode.");
                std::process::exit(1);
            }))
            .unwrap_or_else(|err| {
                eprintln!("[Error] Cannot open answer file specified for compare mode.");
                eprintln!("{err}");
                std::process::exit(1);
            });

            let fc = f.try_clone().unwrap_or_else(|_err| {
                eprintln!("[Error] Error when reading the answer file.");
                std::process::exit(1);
            });

            let len = find_blankless_end(fc).unwrap_or_else(|_err| {
                eprintln!("[Error] Error reading the answer file.");
                std::process::exit(1);
            });

            f.seek(std::io::SeekFrom::Start(0)).unwrap_or_else(|_err| {
                eprintln!("[Error] Error when reading the answer file.");
                std::process::exit(1);
            });

            let mut freader = BufReader::new(f);

            match args.output {
                OutputType::FILE => {
                    let f = File::open(args.file.unwrap_or_else(|| {
                        eprintln!("[Error] No output file provided for compare mode.");
                        std::process::exit(1);
                    }))
                    .unwrap_or_else(|err| {
                        eprintln!("[Error] Cannot open output file specified for compare mode.");
                        eprintln!("{err}");
                        std::process::exit(1);
                    });
                    let mut rd = BufReader::new(f);

                    for _i in 1..len {
                        let mut a = String::new();
                        let mut b = String::new();

                        let reference = rd.by_ref();
                        reference
                            .take(1)
                            .read_to_string(&mut a)
                            .unwrap_or_else(|_err| {
                                eprintln!("[Error] Error when reading the output file.");
                                std::process::exit(1);
                            });

                        let reference = freader.by_ref();
                        reference
                            .take(1)
                            .read_to_string(&mut b)
                            .unwrap_or_else(|_err| {
                                eprintln!("[Error] Error when reading the answer file.");
                                std::process::exit(1);
                            });

                        if a != b {
                            println!("0");
                            std::process::exit(0);
                        }
                    }

                    println!("100");
                    std::process::exit(0);
                }
                OutputType::STDOUT => {
                    for i in 1..len {
                        let mut b = String::new();

                        let a = _stdout[(i - 1) as usize .. i as usize].to_string();

                        let reference = freader.by_ref();
                        reference
                            .take(1)
                            .read_to_string(&mut b)
                            .unwrap_or_else(|_err| {
                                eprintln!("[Error] Error when reading the answer file.");
                                std::process::exit(1);
                            });

                        if a != b {
                            println!("0");
                            std::process::exit(0);
                        }
                    }

                    println!("100");
                    std::process::exit(0);
                }
            }
        }
        Mode::BOOL => {
            // Support true or false, case-insensitive
            // Support 0 or 1
            let result: String = match args.answer {
                None => match args.output {
                    OutputType::FILE => match args.file {
                        None => {
                            eprintln!("[Error] Missing output file arg for file mode.");
                            eprintln!("Exiting...");
                            std::process::exit(1);
                        }
                        Some(file) => {
                            let mut f = File::open(file).unwrap_or_else(|_err| {
                                eprintln!("[Error] Cannot open provided file.");
                                eprintln!("Exiting...");
                                std::process::exit(1);
                            });
                            let mut res = String::new();
                            f.read_to_string(&mut res).unwrap_or_else(|_err| {
                                eprintln!("[Error] Error when reading the output file.");
                                std::process::exit(1);
                            });
                            res
                        }
                    },
                    OutputType::STDOUT => match args.file {
                        None => _stdout,
                        Some(_) => {
                            eprintln!("[Error] STDOUT mode doesn't require a file arg.");
                            eprintln!("Exiting...");
                            std::process::exit(1);
                        }
                    },
                },
                Some(_) => {
                    eprintln!("[Error] Bool mode doesn't require a answer arg.");
                    eprintln!("Exiting...");
                    std::process::exit(1);
                }
            };
            match result.to_ascii_lowercase().as_str() {
                "0" => {
                    println!("0");
                    std::process::exit(0);
                }
                "1" => {
                    println!("100");
                    std::process::exit(0);
                }
                "true" => {
                    println!("100");
                    std::process::exit(0);
                }
                "false" => {
                    println!("0");
                    std::process::exit(0);
                }
                _ => {
                    eprintln!("[Error] illegal result for bool mode.");
                    println!("0");
                    std::process::exit(1);
                }
            }
        }
        Mode::SCORE => {
            // Support integer or decimal
            let result: String = match args.answer {
                None => match args.output {
                    OutputType::FILE => match args.file {
                        None => {
                            eprintln!("[Error] Missing output file arg for file mode.");
                            eprintln!("Exiting...");
                            std::process::exit(1);
                        }
                        Some(file) => {
                            let mut f = File::open(file).unwrap_or_else(|_err| {
                                eprintln!("[Error] Cannot open provided file.");
                                eprintln!("Exiting...");
                                std::process::exit(1);
                            });
                            let mut res = String::new();
                            f.read_to_string(&mut res).unwrap_or_else(|_err| {
                                eprintln!("[Error] Error when reading the output file.");
                                std::process::exit(1);
                            });
                            res
                        }
                    },
                    OutputType::STDOUT => match args.file {
                        None => _stdout,
                        Some(_) => {
                            eprintln!("[Error] STDOUT mode doesn't require a file arg.");
                            eprintln!("Exiting...");
                            std::process::exit(1);
                        }
                    },
                },
                Some(_) => {
                    eprintln!("[Error] Score mode doesn't require a answer arg.");
                    eprintln!("Exiting...");
                    std::process::exit(1);
                }
            };
            let res = result.parse::<f64>().unwrap_or_else(|_err| {
                eprintln!("[Error] Cannot parse the result.");
                eprintln!("Exiting...");
                std::process::exit(1);
            });

            println!("{res}");
            std::process::exit(0);
        }
    }
}
