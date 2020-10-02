use std::fs::File;
use std::io::{BufRead, BufReader};
use std::io;

use clap::{App, Arg, ArgMatches};
use regex::Regex;

fn main() {
    let args = create_cli_app();

    let pattern = args.value_of("pattern").unwrap();
    let regex = Regex::new(pattern).unwrap();

    let input = args.value_of("input").unwrap_or("-");
    if input == "-" {
        let stdin = io::stdin();
        let reader = stdin.lock();
        process_lines(regex, reader);
    } else {
        let f = File::open(input).unwrap();
        let reader = BufReader::new(f);
        process_lines(regex, reader);
    }
}

fn create_cli_app() -> ArgMatches<'static> {
    App::new("grep-lite")
        .version("0.1")
        .about("searches for patterns")
        .arg(Arg::with_name("pattern")
            .help("The pattern to search for")
            .takes_value(true)
            .required(true))
        .arg(Arg::with_name("input")
            .help("File to search on")
            .takes_value(true)
            .required(false))
        .get_matches()
}

fn process_lines<T: BufRead + Sized>(regex: Regex, reader: T) {
    for (i, line_) in reader.lines().enumerate() {
        let line = line_.unwrap();
        match regex.find(&line) {
            Some(_) => println!("{}: {}", i + 1, line),
            None => (),
        }
    }
}