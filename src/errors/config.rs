use std::convert::From;
use std::{error::Error, fmt::Display};

#[derive(Debug)]
pub enum ConfigError {
    ConfigPathNotFound,
    ParseError(toml::de::Error),
    ReadError(std::io::Error),
}

impl Display for ConfigError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ConfigError::ConfigPathNotFound => write!(f, "config path not found"),
            ConfigError::ParseError(error) => {
                write!(f, "failed to parse config file: {}", error)
            }
            ConfigError::ReadError(error) => {
                write!(f, "failed to read the config file: {}", error)
            }
        }
    }
}

impl From<toml::de::Error> for ConfigError {
    fn from(value: toml::de::Error) -> Self {
        ConfigError::ParseError(value)
    }
}

impl From<std::io::Error> for ConfigError {
    fn from(value: std::io::Error) -> Self {
        ConfigError::ReadError(value)
    }
}

impl Error for ConfigError {}
