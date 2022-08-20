use clap::{App, Arg, ArgMatches};

use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

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
                .help("path to input file")
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

fn print_file_contents(filename: &str, all_caps: &bool) {
    let f = File::open(filename).unwrap();
    let reader = BufReader::new(f);
    for buf_line in reader.lines() {
        let line = buf_line.unwrap();
        match all_caps {
            true => println!("{}", line.to_uppercase()),
            false => println!("{}", line),
        }
    }
}

fn main() {
    let matches = get_args();

    let all_caps = matches.is_present("all_caps");

    if let Some(filename) = matches.value_of("input_file") {
        print_file_contents(&filename, &all_caps);
    }
}
