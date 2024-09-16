use crate::config::{Config, Sub};
use directories::BaseDirs;
use nix::unistd::execvp;
use std::{
    env::{self},
    ffi::CString,
    fs,
    path::PathBuf,
    process,
};

pub fn template_folder_path() -> Result<String, Box<dyn std::error::Error>> {
    let config = Config::read()?;
    let template_path = alternate_path(config.general.template_folder_path);
    Ok(template_path)
}

pub fn command_folder_path(command: Sub) -> Result<String, Box<dyn std::error::Error>> {
    let config = Config::read()?;

    match command {
        Sub::Note => {
            let note_path = alternate_path(config.note.folder_path);
            Ok(note_path)
        }
        Sub::Journal => {
            let journal_path = alternate_path(config.journal.folder_path);
            Ok(journal_path)
        }
    }
}

pub fn templates_in_folder() -> Option<Vec<String>> {
    let path = template_folder_path().ok()?;

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
    command: Sub,
    name: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    let templates_vec = templates_in_folder();

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

    // Check if there's already a note with the same name specified by the user on the folder path
    let path = command_folder_path(command)?;

    let dir_contents: Vec<String> = fs::read_dir(path)?
        .filter_map(|entry| entry.ok())
        .filter_map(|entry| entry.file_name().into_string().ok())
        .filter(|name| name.ends_with(".md"))
        .map(|name| name.trim_end_matches(".md").to_string())
        .collect();

    if dir_contents.contains(&name.to_owned()) {
        eprintln!("There is already a note with the name: '{}'", &name);
        process::exit(1)
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

fn alternate_path(path: String) -> String {
    if path.starts_with("~/") {
        if let Some(base_dirs) = BaseDirs::new() {
            let home_dir = base_dirs.home_dir().to_str().unwrap();

            return path.replacen('~', home_dir, 1);
        }
    }

    path
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_alternate_path() {
        if let Some(base_dirs) = BaseDirs::new() {
            let home_dir = base_dirs.home_dir().to_str().unwrap();

            let path_formated = format!("{}{}", home_dir, "/foo/bar");

            assert_eq!(path_formated, alternate_path("~/foo/bar".to_owned()))
        }

        assert_eq!("/foo/bar", alternate_path("/foo/bar".to_owned()));
    }
}
