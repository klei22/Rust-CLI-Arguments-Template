use clap::{App, Arg, ArgMatches};

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
                .takes_value(true)
        )
        .arg(
            Arg::with_name("quiet_flag")
                .short('q')
                .help("quiet flag, do not emit output")
                .takes_value(false),
        )
        .get_matches()
}

fn main() {
    let matches = get_args();

    let quiet = matches.is_present("quiet_flag");

    if !quiet {
        if let Some(i) = matches.value_of("input_file") {
            println!("Value for input: {}", i);
        }
    }
}
