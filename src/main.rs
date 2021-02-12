#![allow(unused_parens)]

// Modules
mod lib;
mod constants;

// Crates
extern crate toml;

// Imports
use lib::*;
use constants::*;
use std::fs;
use std::time::SystemTime;

// Functions
fn main() 
{
    output(OutputType::Info, "Program started");
    let time = SystemTime::now();

    let settings: Settings;
    let mut config = Configuration {
        out: None,
        source: None,
        test_ending: None,
        verbose: None
    };

    if (config_exists(CONFIGURATION_PATH))
    {
        let read_config = fs::read(CONFIGURATION_PATH).unwrap();

        config = toml::from_slice(read_config.as_slice()).unwrap();
    }

    settings = Settings {
        out: config.out.unwrap_or(String::from(OUT_DIRECTORY)),
        source: config.source.unwrap_or(String::from(SOURCE_DIRECTORY)),
        test_ending: config.test_ending.unwrap_or(String::from(TEST_ENDING)),
        verbose: config.verbose.unwrap_or(IS_VERBOSE)
    };

    run_dir_checks(&settings.out, false);
    run_dir_checks(&settings.source, true);

    copy_with_settings(&settings);

    output(OutputType::Info, format!("Finished, time taken: {}ms", time.elapsed().unwrap().as_millis()));
}