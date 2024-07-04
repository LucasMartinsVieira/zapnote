use std::{
    fs::{self, File},
    io::{self, Write},
    path::PathBuf,
};

use directories::ProjectDirs;
use serde::Deserialize;

use crate::errors::config::ConfigError;

#[derive(Deserialize)]
pub struct Config {
    pub general: GeneralConfig,
    pub note: NoteConfig,
    pub journal: JournalConfig,
}

#[derive(Deserialize)]
pub struct GeneralConfig {
    pub template_folder_path: String,
    pub default_editor: Option<String>,
}

#[derive(Deserialize)]
pub struct NoteConfig {
    pub folder_path: String,
}

#[derive(Deserialize)]
pub struct JournalConfig {
    pub folder_path: String,
}

pub enum Subcommand {
    Note,
    Journal,
}

impl Config {
    pub fn get_default_path() -> Option<PathBuf> {
        let dirs = ProjectDirs::from("", "", "sb")?;

        let mut path = dirs.config_dir().to_owned();
        path.push("sb.toml");

        Some(path)
    }
    pub fn read_config() -> Result<Config, ConfigError> {
        let config_path = Self::get_default_path();

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
        let default_path = Self::get_default_path()?;
        let default_parent = default_path.parent();

        if let Some(parent) = default_parent {
            if let Err(err) = fs::create_dir_all(parent) {
                println!("error creating config directory: {:?}", err);
                return Some(default_path);
            }
        } else {
            println!("default parent directory not found.");
        }

        // Creates config file if doesn't exist.
        let new_file = File::options()
            .read(true)
            .write(true)
            .create_new(true)
            .open(&default_path);

        match new_file {
            Ok(mut new_file) => {
                let default_file = include_bytes!("../resources/default-sb.toml");
                match new_file.write_all(default_file) {
                    Ok(()) => {
                        println!("wrote default configuration file at {:?}", &default_path)
                    }
                    Err(err) => {
                        println!("error writting default config file {:?}", err)
                    }
                }
            }
            Err(err) if err.kind() == io::ErrorKind::AlreadyExists => {}
            Err(err) => println!("error creating config file at {:?}: {err:?}", &default_path),
        }

        Some(default_path)
    }
}
