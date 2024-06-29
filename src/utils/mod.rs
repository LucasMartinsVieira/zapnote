use crate::config::{Config, SubcommandType};

pub fn get_template_folder_path() -> Result<String, Box<dyn std::error::Error>> {
    let config = Config::read_config()?;
    Ok(config.general.template_folder_path)
}

pub fn get_command_folder_path(
    command: SubcommandType,
) -> Result<String, Box<dyn std::error::Error>> {
    let config = Config::read_config()?;

    match command {
        SubcommandType::Note => Ok(config.note.folder_path),
        SubcommandType::Journal => Ok(config.journal.folder_path),
    }
}
