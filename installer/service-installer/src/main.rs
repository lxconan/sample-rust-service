use clap::{Arg, App, ArgMatches};
use colored::Colorize;
use std::process::exit;

fn main() {
    let matches:ArgMatches = App::new("Windows Service Installer")
        .arg(
            Arg::with_name("action type")
                .short("a")
                .long("action")
                .required(true)
                .multiple(false)
                .possible_values(&["create", "remove"])
                .takes_value(true)
        )
        .get_matches();

    match matches.value_of("action type") {
        None => {
            // Will not be here actually.
            exit(1);
        }
        Some(_) => {
            exit(0);
        }
    }
}