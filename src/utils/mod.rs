use std::{fs, path::PathBuf};

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

pub fn get_templates_in_folder() -> Option<Vec<String>> {
    let path = get_template_folder_path().ok()?;

    // Search in template directory for markdown files, put them in a Vec<String> and remove .md
    // from the files name
    let dir_contents: Vec<String> = fs::read_dir(path)
        .ok()?
        .filter_map(|entry| entry.ok())
        .filter_map(|entry| entry.file_name().into_string().ok())
        .filter(|name| name.ends_with(".md"))
        .map(|name| name.trim_end_matches(".md").to_string())
        .collect();

    Some(dir_contents)
}

pub fn check_template(
    template_name: String,
    command: SubcommandType,
    name: String,
) -> Result<(), Box<dyn std::error::Error>> {
    let templates_vec = get_templates_in_folder();

    // Check if template specified by user exists on template folder
    match templates_vec {
        Some(vec) => {
            if !vec.contains(&template_name) {
                eprintln!(
                    "template '{}' doesn't exist in template folder",
                    template_name
                );
                std::process::exit(1)
            }
        }
        None => {
            eprintln!("No templates found on folder");
            std::process::exit(1)
        }
    }

    // Check if there's already a note with the same name specified by the user on the folder path
    let path = get_command_folder_path(command)?;

    let dir_contents: Vec<String> = fs::read_dir(&path)?
        .filter_map(|entry| entry.ok())
        .filter_map(|entry| entry.file_name().into_string().ok())
        .filter(|name| name.ends_with(".md"))
        .map(|name| name.trim_end_matches(".md").to_string())
        .collect();

    if dir_contents.contains(&name) {
        eprintln!(
            "There is already a note with the name: '{}' on that location",
            &name
        );
        std::process::exit(1)
    }

    Ok(())
}

pub fn get_template_file_contents(template_name: String) -> Option<String> {
    let template_folder_path = get_template_folder_path().ok();

    if let Some(path) = template_folder_path {
        let mut template_file_path = PathBuf::from(path);
        let template_name_with_extension = format!("{template_name}.md");

        template_file_path.push(&template_name_with_extension);

        let template_file_contents = fs::read_to_string(template_file_path).ok()?;

        Some(template_file_contents)
    } else {
        None
    }
}
