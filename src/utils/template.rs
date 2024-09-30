use std::{
    collections::HashMap,
    env, fs,
    path::{Path, PathBuf},
    process,
};

use crate::{
    config::{Config, Sub},
    utils::alternate_path,
};

use super::{command_folder_path, current_date_formatted, run_editor};

pub fn template_folder_path() -> Result<String, Box<dyn std::error::Error>> {
    // TODO: Refactor this for testing purposes?
    let config = Config::read()?;
    let template_path = alternate_path(config.general.template_folder_path);
    Ok(template_path)
}

fn specific_template(sub: Sub) -> Option<Vec<HashMap<String, String>>> {
    let config = Config::read().ok()?;
    let mut output = Vec::new();

    match sub {
        // TODO: Add specific templates for regular notes.
        Sub::Note => todo!(),
        Sub::Journal => {
            if let Some(journal_entries) = config.journal {
                journal_entries.iter().for_each(|entry| {
                    let mut entry_map = HashMap::new();

                    entry_map.insert("name".to_string(), entry.name.to_string());
                    entry_map.insert("format".to_string(), entry.format.to_string());
                    entry_map.insert("template".to_string(), entry.template.to_string());
                    entry_map.insert("folder_path".to_string(), entry.folder_path.to_string());

                    output.push(entry_map)
                })
            } else {
                eprintln!("No templates found in configuration file");
                process::exit(1);
            }
        }
    }

    Some(output)
}

pub fn specific_template_info(sub: Sub, name: &str) -> Option<HashMap<String, String>> {
    let mut template_info = HashMap::new();

    match sub {
        Sub::Note => todo!(),
        Sub::Journal => {
            if let Some(entries) = specific_template(Sub::Journal) {
                // Flag to track if the name was found
                let mut found = false;

                entries.iter().enumerate().for_each(|(_index, entry)| {
                    // Use the get method to get the value of the name key
                    if let Some(name_value) = entry.get("name") {
                        // Check if the name matches
                        if name_value == name {
                            found = true;

                            // Access other values in the HashMap
                            if let Some(format_value) = entry.get("format") {
                                template_info
                                    .insert("format".to_string(), format_value.to_string());
                            }
                            if let Some(template_value) = entry.get("template") {
                                template_info
                                    .insert("template".to_string(), template_value.to_string());
                            }
                            if let Some(folder_path_value) = entry.get("folder_path") {
                                template_info.insert(
                                    "folder_path".to_string(),
                                    folder_path_value.to_string(),
                                );
                            }
                        }
                    }
                });

                if !found {
                    eprintln!("No entry found for the name '{}'", name);
                    process::exit(1);
                }
            }
            Some(template_info)
        }
    }
}

pub fn templates_in_folder(path: String) -> Option<Vec<String>> {
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

pub fn check_template(template: &str) -> Result<(), Box<dyn std::error::Error>> {
    let path = template_folder_path()?;
    let templates_vec = templates_in_folder(path);

    // Check if template specified by user exists on template folder
    match templates_vec {
        Some(vec) => {
            if !vec.contains(&template.to_owned()) {
                eprintln!("template '{}' doesn't exist in template folder", template);
                println!();

                println!("Available templates: ");
                println!();

                // Iterate over the vector of template names and print each template name.
                vec.iter()
                    .for_each(|template_name| println!("{}", template_name));
                process::exit(1)
            }
        }
        None => {
            eprintln!("No templates found on folder");
            process::exit(1)
        }
    }
    Ok(())
}

pub fn template_file_contents(template: String) -> Option<String> {
    let template_folder_path = template_folder_path().ok();

    if let Some(path) = template_folder_path {
        let mut template_file_path = PathBuf::from(path);
        let template_name_with_extension = format!("{template}.md");

        template_file_path.push(&template_name_with_extension);

        let template_file_contents = fs::read_to_string(template_file_path).ok()?;

        Some(template_file_contents)
    } else {
        None
    }
}

// TODO: Do a function for inserting the template into the file, being a specific template from the
// config file or not
pub fn insert_template_into_file(
    template: String,
    name: String,
    command: Sub,
) -> Result<(), Box<dyn std::error::Error>> {
    let command_path_str = command_folder_path(command)?;
    let full_path = format!("{command_path_str}/{name}.md");

    let command_path_buf = PathBuf::from(full_path);
    let path = command_path_buf.to_str().unwrap();

    let template_file_contents = template_file_contents(template);

    if let Some(contents) = template_file_contents {
        if let Some(parent) = Path::new(&path).parent() {
            if let Err(err) = fs::create_dir_all(parent) {
                eprintln!("error creating directories: {:?}", err);
                process::exit(1);
            }
        }

        if let Err(err) = fs::write(path, contents) {
            eprintln!("error writing template into file: {:?}", err);
            process::exit(1)
        }
    }

    let no_editor = env::var("ZAPNOTE_NO_EDITOR")?;
    let parsed_no_editor: bool = no_editor.parse().unwrap_or(false);

    // If the flag --no-editor is passed by user, the program exist with status code 0, before
    // running the run_editor function.
    if parsed_no_editor {
        process::exit(0);
    }

    let config = Config::read()?;
    let default_editor = config.general.editor;

    match default_editor.as_deref() {
        Some("") | None => {
            let editor = env::var("EDITOR").unwrap_or("vi".to_string());
            run_editor(&editor, path);
        }
        Some(editor) => {
            run_editor(editor, path);
        }
    }

    Ok(())
}

pub fn insert_template_journal(
    template_hashmap: HashMap<String, String>,
) -> Result<(), Box<dyn std::error::Error>> {
    // TODO: Do better error handling
    let format = template_hashmap.get("format").unwrap();
    let template = template_hashmap.get("template").unwrap();
    let folder_path = template_hashmap.get("folder_path").unwrap();

    let date_formatted = current_date_formatted(format);
    let command_path_str = command_folder_path(Sub::Journal).unwrap();
    let full_path = format!("{command_path_str}/{folder_path}/{date_formatted}.md");

    let command_path_buf = PathBuf::from(full_path);
    let path = command_path_buf.to_str().unwrap();

    let template_file_contents = template_file_contents(template.to_string());

    if let Some(contents) = template_file_contents {
        if let Some(parent) = Path::new(&path).parent() {
            if let Err(err) = fs::create_dir_all(parent) {
                eprintln!("error creating directories: {:?}", err);
                process::exit(1);
            }
        }

        if let Err(err) = fs::write(path, contents) {
            eprintln!("error writing template into file: {:?}", err);
            process::exit(1)
        }
    }

    let no_editor = env::var("ZAPNOTE_NO_EDITOR")?;
    let parsed_no_editor: bool = no_editor.parse().unwrap_or(false);

    // If the flag --no-editor is passed by user, the program exist with status code 0, before
    // running the run_editor function.
    if parsed_no_editor {
        process::exit(0);
    }

    let config = Config::read()?;
    let default_editor = config.general.editor;

    match default_editor.as_deref() {
        Some("") | None => {
            let editor = env::var("EDITOR").unwrap_or("vi".to_string());
            run_editor(&editor, path);
        }
        Some(editor) => {
            run_editor(editor, path);
        }
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use tempfile::TempDir;

    #[test]
    fn test_templates_in_folder_only_markdown_files() {
        let temp_dir = TempDir::new().unwrap();
        let template_dir = temp_dir.path().join("templates");
        fs::create_dir(&template_dir).unwrap();

        let markdown_files = ["template1.md", "template2.md"];
        markdown_files
            .iter()
            .for_each(|file| fs::write(template_dir.join(file), "content").unwrap());

        let template_dir_str = template_dir.to_str().unwrap();
        let template_dir_string = template_dir_str.to_string();

        let templates = templates_in_folder(template_dir_string).unwrap();
        assert_eq!(
            templates,
            vec!["template1".to_string(), "template2".to_string()]
        )
    }

    #[test]
    fn test_templates_in_folder_with_markdown_non_markdown() {
        let temp_dir = TempDir::new().unwrap();
        let template_dir = temp_dir.path().join("templates");
        fs::create_dir(&template_dir).unwrap();

        let files = [
            "template1.md",
            "template2.md",
            "not_a_template.pdf",
            "not_a_template.txt",
            "not_a_template.png",
        ];

        files
            .iter()
            .for_each(|file| fs::write(template_dir.join(file), "content").unwrap());

        let template_dir_str = template_dir.to_str().unwrap();
        let template_dir_string = template_dir_str.to_string();

        let templates = templates_in_folder(template_dir_string).unwrap();
        assert_eq!(
            templates,
            vec!["template1".to_string(), "template2".to_string()]
        )
    }

    #[test]
    fn test_templates_in_folder_no_markdown_files() {
        let temp_dir = TempDir::new().unwrap();
        let template_dir = temp_dir.path().join("templates");
        fs::create_dir(&template_dir).unwrap();

        let files = ["file1.txt", "file2.pdf"];

        files
            .iter()
            .for_each(|file| fs::write(template_dir.join(file), "content").unwrap());

        let template_dir_str = template_dir.to_str().unwrap();
        let template_dir_string = template_dir_str.to_string();

        let templates = templates_in_folder(template_dir_string);

        assert_eq!(templates, Some(vec![]));
    }

    #[test]
    fn test_templates_in_folder_not_exists() {
        let templates = templates_in_folder("this/should/not/exist".to_string());

        assert!(templates.is_none());
    }
}
