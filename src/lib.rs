#![allow(unused_parens)]

// Crates
extern crate serde;
extern crate crossterm;

// Imports
use self::serde::Deserialize;
use self::crossterm::style::Colorize;
use std::path::Path;
use std::fmt::Display;
use std::fs;

// Structures
#[derive(Deserialize)]
pub struct Configuration
{
    pub out: Option<String>,
    pub source: Option<String>,
    pub test_ending: Option<String>,
    pub verbose: Option<bool>
}

pub struct Settings
{
    pub out: String,
    pub source: String,
    pub test_ending: String,
    pub verbose: bool
}

pub enum OutputType
{
    Info,
    Verbose,
    Error
}

// Functions
pub fn output(output_type: OutputType, message: impl Display)
{
    match output_type {
        OutputType::Info => println!("\t{}: {}", "INFO".yellow(), message),
        OutputType::Verbose => println!("\t{}: {} {}", "INFO".yellow(), message, "(verbose)".green()),
        OutputType::Error => panic!("\t{}: {}", "ERROR".red(), message)
    }
}

pub fn assert(expression: bool, message: impl Display)
{
    if (!expression)
    {
        output(OutputType::Error, message);
    }
}

pub fn config_exists(dir: &str) -> bool
{
    let path = Path::new(dir);

    if (path.exists())
    {
        return true;
    }

    output(OutputType::Info, "No trmv.config.toml found, using defaulted settings");
    return false;
}

pub fn run_dir_checks(dir: &String, should_exist: bool)
{
    let path = Path::new(&dir);

    if (!path.exists() && should_exist)
    {
        output(OutputType::Error, format!("{} directory does not exist!", dir));
    }

    assert(path.is_file() == false, format!("{} must be a directory, not a file!", dir));
}

pub fn copy_with_settings(settings: &Settings)
{
    if (Path::new(&settings.out).exists())
    {
        fs::remove_dir_all(&settings.out).unwrap();
    }

    fs::create_dir(&settings.out).unwrap();

    if (settings.verbose) 
    {
        output(OutputType::Verbose, format!("Entering \"{}\" to copy to \"{}\"", &settings.source, &settings.out));        
    }

    for file in fs::read_dir(&settings.source).unwrap()
    {
        let entry = file.unwrap();
        let name = entry.file_name().into_string().unwrap();

        let path = format!("{}/{}", settings.source, name);
        let out_path = format!("{}/{}", settings.out, name);

        let data = entry.metadata().unwrap();

        if (!name.ends_with(&settings.test_ending))
        {
            if (data.is_file())
            {
                fs::copy(&path, &out_path).unwrap();

                if (settings.verbose)
                {
                    output(OutputType::Verbose, format!("Copied \"{}\" to \"{}\"", &path, &out_path));
                }
            }
            else
            {
                copy_with_settings(&Settings {
                    source: path,
                    out: out_path,
                    test_ending: settings.test_ending.clone(),
                    verbose: settings.verbose
                })
            }
        }
        else
        {
            output(OutputType::Info, format!("Ignored \"{}\"", &path));
        }
    }

    if (settings.verbose) 
    {
        output(OutputType::Verbose, format!("Leaving directory \"{}\"", &settings.source));        
    }
}