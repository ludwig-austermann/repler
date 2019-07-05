extern crate clap;

use std::process;
use clap::{Arg, App};

use repler;
use repler::Config;

fn main() {
    let matches = App::new("Repler")
        .version("0.1")
        .author("Ludwig Austermann <ludwig.austermann@gmail.com>")
        .about("an automatic in file replacer")
        .arg(Arg::with_name("TARGET FILE")
            .help("the name of the target file")
            .required(true)
            .index(1))
        .arg(Arg::with_name("PATTERN FILE")
            .help("the name of the pattern file")
            .required(true)
            .index(2))
        .arg(Arg::with_name("v")
            .short("v")
            .multiple(true)
            .help("Sets the level of verbosity"))
        .get_matches();

    let config = Config {
        target_filename: matches.value_of("TARGET FILE").unwrap().to_string(), // required
        pattern_filename: matches.value_of("PATTERN FILE").unwrap().to_string(), // required
        verbosity: matches.occurrences_of("v"),
    };

    if let Err(e) = repler::run(config) {
        eprintln!("Application error: {}", e);

        process::exit(1);
    }
}