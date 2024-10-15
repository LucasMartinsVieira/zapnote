use crate::errors::config::ConfigError;
use directories::ProjectDirs;
use serde::Deserialize;
use std::{
    fs::{self, File},
    io::{self, Write},
    path::PathBuf,
    process,
};

#[derive(Deserialize)]
pub struct Config {
    pub general: GeneralConfig,
    pub journal: Option<Vec<JournalConfig>>,
}

#[derive(Deserialize)]
pub struct GeneralConfig {
    pub template_folder_path: String,
    pub editor: Option<String>,
    pub note_folder_path: String,
    pub journal_folder_path: String,
}

#[derive(Debug, Deserialize)]
pub struct JournalConfig {
    pub name: String,
    pub format: String,
    pub template: String,
    pub folder_path: String,
}

pub enum Sub {
    Note,
    Journal,
}

impl Config {
    pub fn default_path() -> Option<PathBuf> {
        let dirs = ProjectDirs::from("", "", "zapnote")?;

        let mut path = dirs.config_dir().to_owned();
        path.push("zapnote.toml");

        Some(path)
    }
    pub fn read() -> Result<Config, ConfigError> {
        let config_path = Self::default_path();

        if let Some(path) = config_path {
            let config_file_contents = fs::read_to_string(path)?;
            let config: Config = toml::from_str(&config_file_contents)?;
            Ok(config)
        } else {
            Err(ConfigError::ConfigPathNotFound)
        }
    }
    pub fn load() -> Option<PathBuf> {
        // Creates config directory if doesn't exist.
        let default_path = Self::default_path()?;
        let default_parent = default_path.parent();

        if let Some(parent) = default_parent {
            if let Err(err) = fs::create_dir_all(parent) {
                eprintln!("error creating config directory: {:?}", err);
                return Some(default_path);
            }
        } else {
            eprintln!("default parent directory not found.");
        }

        // Creates config file if doesn't exist.
        let new_file = File::options()
            .read(true)
            .write(true)
            .create_new(true)
            .open(&default_path);

        match new_file {
            Ok(mut new_file) => {
                let default_file = include_bytes!("../resources/default-zapnote.toml");
                match new_file.write_all(default_file) {
                    Ok(()) => {
                        println!("default configuration file not found");
                        println!("wrote default configuration file at {:?}", &default_path);
                        process::exit(0);
                    }
                    Err(err) => {
                        eprintln!("default configuration file not found");
                        eprintln!("error writting default config file {:?}", err)
                    }
                }
            }
            Err(err) if err.kind() == io::ErrorKind::AlreadyExists => {}
            Err(err) => println!("error creating config file at {:?}: {err:?}", &default_path),
        }

        Some(default_path)
    }
}
