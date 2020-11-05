//------------------------------------------------------------------------------
// Project: jrnlvw
// File name: main.rs
// File Description: Jrnlvw main file
// License: MIT
//------------------------------------------------------------------------------
#[macro_use]
extern crate clap;
extern crate chrono;
extern crate serde;
extern crate serde_json;
#[macro_use]
extern crate failure;

use std::process;

mod cli;
mod opt;
mod parser;

//------------------------------------------------------------------------------

fn main() {
    // get cli arguments
    let matches = cli::create_cli().get_matches();

    let options = match opt::CliOptions::from_matches(&matches) {
        Ok(opt) => Ok(opt),
        Err(err) => {
            eprintln!("Failed to get cli-options: {}", err);
            Err(err)
        }
    };

    if options.is_err() {
        process::exit(-1);
    }

    // Parse logfile based on cli options
    let parsed_file = match parser::ParsedLogfile::new(&options.unwrap()) {
        Ok(f) => Ok(f),
        Err(err) => {
            eprintln!("Failed to parse logfile: {}", err);
            Err(err)
        }
    };

    if parsed_file.is_err() {
        println!("Unable to continue...");
        process::exit(-1);
    }

    // Display logs
    let logfile = parsed_file.unwrap();
    logfile.print();

    process::exit(0);
}
