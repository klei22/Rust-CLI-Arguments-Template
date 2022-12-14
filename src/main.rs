use clap::{App, Arg, ArgMatches};

use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn get_args() -> ArgMatches {
    App::new("cli_template")
        .version("0.1.0")
        .author("name <your-email-address@address.com>")
        .about("template program for clap cli flags")
        .arg(
            Arg::with_name("input_file")
                .short('f')
                .long("input")
                .value_name("INPUT_FILE")
                .help("path to input file, or '-' for stdin")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("all_caps")
                .short('A')
                .help("set for all caps")
                .takes_value(false),
        )
        .get_matches()
}

type ResultOrError<T> = Result<T, Box<dyn Error>>;

fn open(filename: &str) -> ResultOrError<Box<dyn BufRead>> {
    match filename {
        "-" => Ok(Box::new(BufReader::new(std::io::stdin()))),
        _ => Ok(Box::new(BufReader::new(File::open(filename)?))),
    }
}

fn print_file_contents(filename: &str, all_caps: &bool) -> ResultOrError<()> {
    match open(&filename) {
        Err(e) => eprintln!("{}: {}", filename, e),
        Ok(file) => {
            for line_result in file.lines() {
                let line = line_result?;
                match all_caps {
                    true => println!("{}", line.to_uppercase()),
                    false => println!("{}", line),
                }
            }
        }
    }
    Ok(())
}

    fn main() {
        let matches = get_args();

        let all_caps = matches.is_present("all_caps");

    if let Some(filename) = matches.value_of("input_file") {
        print_file_contents(filename, &all_caps).unwrap();
    }
}
