use crate::config::{Config, Subcommand};
use nix::unistd::execvp;
use std::{env, ffi::CString, fs, path::PathBuf, process};

pub fn get_template_folder_path() -> Result<String, Box<dyn std::error::Error>> {
    let config = Config::read_config()?;
    Ok(config.general.template_folder_path)
}

pub fn get_command_folder_path(command: Subcommand) -> Result<String, Box<dyn std::error::Error>> {
    let config = Config::read_config()?;

    match command {
        Subcommand::Note => Ok(config.note.folder_path),
        Subcommand::Journal => Ok(config.journal.folder_path),
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
    template: &str,
    command: Subcommand,
    name: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    let templates_vec = get_templates_in_folder();

    // Check if template specified by user exists on template folder
    match templates_vec {
        Some(vec) => {
            if !vec.contains(&template.to_owned()) {
                eprintln!("template '{}' doesn't exist in template folder", template);
                process::exit(1)
            }
        }
        None => {
            eprintln!("No templates found on folder");
            process::exit(1)
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

    if dir_contents.contains(&name.to_owned()) {
        eprintln!(
            "There is already a note with the name: '{}' on that location",
            &name
        );
        process::exit(1)
    }

    Ok(())
}

pub fn get_template_file_contents(template: String) -> Option<String> {
    let template_folder_path = get_template_folder_path().ok();

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

// TODO: Do better error handling in this function
pub fn insert_template_into_file(
    template: String,
    name: String,
    command: Subcommand,
) -> Result<(), Box<dyn std::error::Error>> {
    let command_path_str = get_command_folder_path(command)?;
    let full_path = format!("{command_path_str}/{name}.md");

    let command_path_buf = PathBuf::from(full_path);
    let path = command_path_buf.to_str().unwrap();

    let template_file_contents = get_template_file_contents(template);

    if let Some(contents) = template_file_contents {
        fs::write(path, contents).unwrap();
    }

    let no_editor = env::var("SB_NO_EDITOR")?;
    let parsed_no_editor: bool = no_editor.parse().unwrap_or(false);

    if !parsed_no_editor {
        let config = Config::read_config()?;
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
    }

    Ok(())
}

fn run_editor(editor: &str, path: &str) {
    // TODO: Do better error handling
    let editor_cstr = CString::new(editor).expect("CString::new failed editor");
    let path_cstr = CString::new(path).expect("CString::new failed path");
    let args = [editor_cstr.clone(), path_cstr];

    execvp(&editor_cstr, &args).unwrap_or_else(|err| {
        eprintln!("error executing {}: {}", editor, err);
        process::exit(1);
    });
}
