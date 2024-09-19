use crate::config::{Config, Sub};
use chrono::Local;
use directories::BaseDirs;
use nix::unistd::execvp;
use std::{ffi::CString, fs, process};

pub mod template;

pub fn command_folder_path(command: Sub) -> Result<String, Box<dyn std::error::Error>> {
    let config = Config::read()?;

    match command {
        Sub::Note => {
            let note_path = alternate_path(config.general.note_folder_path);
            Ok(note_path)
        }
        Sub::Journal => {
            let journal_path = alternate_path(config.general.journal_folder_path);
            Ok(journal_path)
        }
    }
}

pub fn check_note_name(name: &str, command: Sub) -> Result<(), Box<dyn std::error::Error>> {
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

pub fn current_date_formatted(format: &str) -> String {
    let current_date = Local::now();

    let date_formatted = current_date.format(format);

    date_formatted.to_string()
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
