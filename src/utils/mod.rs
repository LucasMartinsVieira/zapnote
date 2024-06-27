use std::fs;

use crate::config::Config;

pub fn get_template_folder_path() -> Result<String, Box<dyn std::error::Error>> {
    let config_path = Config::get_default_config_path();
    let mut template_path = String::from("");

    if let Some(path) = config_path {
        let config_file_contents = fs::read_to_string(path)?;
        let config: Config = toml::from_str(&config_file_contents)?;
        template_path = config.general.template_folder_path;
    }

    Ok(template_path)
}
