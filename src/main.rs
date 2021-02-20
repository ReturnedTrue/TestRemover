#![allow(unused_parens)]

// Modules
mod util;
mod constants;

// Crates
extern crate toml;

// Imports
use util::*;
use constants::*;
use std::time::SystemTime;

// Functions
fn main() 
{
    let flag_verbose = handle_flags();

    output(OutputType::Info, "Program started");
    let time = SystemTime::now();

    let config = read_config(CONFIGURATION_PATH);
    let settings = Settings {
        out: config.out.unwrap_or(OUT_DIRECTORY.to_string()),
        source: config.source.unwrap_or(SOURCE_DIRECTORY.to_string()),
        test_ending: config.test_ending.unwrap_or(TEST_ENDING.to_string()),
        verbose: flag_verbose || config.verbose.unwrap_or(IS_VERBOSE)
    };

    run_dir_checks(&settings.out, false);
    run_dir_checks(&settings.source, true);

    copy_with_settings(&settings);

    output(OutputType::Info, format!("Finished, time taken: {}ms", time.elapsed().unwrap().as_millis()));
}