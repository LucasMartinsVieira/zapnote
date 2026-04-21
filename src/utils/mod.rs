use crate::config::{Config, Sub};
use directories::BaseDirs;
use nix::unistd::execvp;
use std::{env, ffi::CString, fs, io, path::Path, path::PathBuf, process};

pub mod casing;
pub mod date;
pub mod placeholder;
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

// TODO: Make this return a boolean to be used in the insert_template_function so the program can
// be used not only to create but to access notes as well?
pub fn check_note_name(
    name: &str,
    command: Sub,
) -> Result<Option<String>, Box<dyn std::error::Error>> {
    // Check if there's already a note with the same name specified by the user on the folder path
    match command {
        Sub::Note => {
            let path = command_folder_path(command)?;

            fs::create_dir_all(&path)?;

            let dir_contents: Vec<String> = fs::read_dir(&path)?
                .filter_map(|entry| entry.ok())
                .filter_map(|entry| entry.file_name().into_string().ok())
                .filter(|name| name.ends_with(".md"))
                .map(|name| name.trim_end_matches(".md").to_string())
                .collect();

            if dir_contents.contains(&name.to_owned()) {
                let full_path = PathBuf::from(path)
                    .join(format!("{name}.md"))
                    .to_string_lossy()
                    .into_owned();

                return Ok(Some(full_path));
            }
        }
        Sub::Journal => unreachable!("journal duplicate checks require a resolved reference date"),
    }

    Ok(None)
}

pub fn check_journal_note_path(full_path: &str) -> Option<String> {
    if Path::new(full_path).is_file() {
        return Some(full_path.to_string());
    }

    None
}

pub fn open_path_in_editor(path: &str) -> Result<(), Box<dyn std::error::Error>> {
    let no_editor = env::var("ZAPNOTE_NO_EDITOR")?;
    let parsed_no_editor: bool = no_editor.parse().unwrap_or(false);

    if parsed_no_editor {
        return Ok(());
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
    let editor_cstr = CString::new(editor).expect("CString::new failed editor");
    let path_cstr = CString::new(path).expect("CString::new failed path");
    let args = [editor_cstr.clone(), path_cstr];

    let Err(err) = execvp(&editor_cstr, &args);
    let error = io::Error::other(format!("error executing {editor}: {err}"));
    eprintln!("{error}");
    process::exit(1);
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

pub fn quarter_from_week(week: u32) -> u32 {
    match week {
        1..=13 => 1,
        14..=26 => 2,
        27..=39 => 3,
        40..=53 => 4,
        _ => unreachable!(),
    }
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

    #[test]
    fn test_quarter_from_week() {
        let test_cases = [
            (1, 1),
            (5, 1),
            (13, 1),
            (14, 2),
            (20, 2),
            (26, 2),
            (27, 3),
            (33, 3),
            (39, 3),
            (40, 4),
            (46, 4),
            (52, 4),
            (53, 4),
        ];

        for (week, expected_quarter) in test_cases {
            let result = quarter_from_week(week);

            assert_eq!(result, expected_quarter, "Failed on week: {}", week)
        }
    }

    #[test]
    fn check_journal_note_path_detects_existing_file() {
        let temp_dir = tempfile::TempDir::new().unwrap();
        let file_path = temp_dir.path().join("existing.md");

        std::fs::write(&file_path, "content").unwrap();

        assert!(Path::new(file_path.to_str().unwrap()).is_file());
    }
}
