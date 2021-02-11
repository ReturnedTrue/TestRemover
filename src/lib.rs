#![allow(unused_parens)]

// Crates
extern crate serde;

// Imports
use self::serde::Deserialize;
use std::path::Path;
use std::fs;

// Structures
#[derive(Deserialize)]
pub struct Configuration
{
    pub out: Option<String>,
    pub source: Option<String>,
    pub test_ending: Option<String>
}

pub struct Settings
{
    pub out: String,
    pub source: String,
    pub test_ending: String
}

// Functions
pub fn config_exists(dir: &str) -> bool
{
    let path = Path::new(dir);

    if (path.exists())
    {
        return true;
    }

    println!("NOTE: No trmv.config.toml found, using defaulted settings");
    return false;
}

pub fn run_dir_checks(dir: &String, should_exist: bool)
{
    let path = Path::new(&dir);

    if (!path.exists() && should_exist)
    {
        panic!("ERROR: {} directory does not exist!", dir)
    }

    assert_eq!(path.is_file(), false, "ERROR: {} must be a directory, not a file!", dir);
}

pub fn copy_with_settings(settings: &Settings)
{
    fs::remove_dir_all(&settings.out).unwrap();
    fs::create_dir(&settings.out).unwrap();

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
                fs::copy(path, out_path).unwrap();
            }
            else
            {
                fs::create_dir(&path).unwrap();

                copy_with_settings(&Settings {
                    source: path,
                    out: out_path,
                    test_ending: settings.test_ending.clone()
                })
            }
        }
    }
}