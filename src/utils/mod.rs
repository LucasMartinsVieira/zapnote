use crate::{
    config::{Config, Sub},
    utils::template::specific_template_info,
};
use chrono::{Datelike, Local};
use directories::BaseDirs;
use nix::unistd::execvp;
use std::{ffi::CString, fs, path::Path, process};

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
pub fn check_note_name(name: &str, command: Sub) -> Result<(), Box<dyn std::error::Error>> {
    // Check if there's already a note with the same name specified by the user on the folder path
    match command {
        Sub::Note => {
            let path = command_folder_path(command)?;

            if let Err(err) = fs::create_dir_all(&path) {
                eprintln!("error creating directories: {:?}", err);
                process::exit(1);
            }

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
        }
        Sub::Journal => {
            let hashmap_info = specific_template_info(Sub::Journal, name).unwrap();

            let folder_path = match hashmap_info.get("folder_path") {
                Some(folder_path_value) => {
                    let command_path = command_folder_path(command)?;
                    format!("{}/{}", command_path, folder_path_value)
                }
                None => todo!(),
            };

            let journal_entry = match hashmap_info.get("format") {
                Some(format_value) => current_date_formatted(format_value),
                None => todo!(),
            };

            let full_path = format!("{}/{}.md", folder_path, journal_entry);

            if Path::new(&full_path).is_file() {
                eprintln!("There is already a note with the path: '{}'", &full_path);
                process::exit(1)
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

fn alternate_path(path: String) -> String {
    if path.starts_with("~/") {
        if let Some(base_dirs) = BaseDirs::new() {
            let home_dir = base_dirs.home_dir().to_str().unwrap();

            return path.replacen('~', home_dir, 1);
        }
    }

    path
}

fn quarter_from_week(week: u32) -> u32 {
    match week {
        1..=13 => 1,
        14..=26 => 2,
        27..=39 => 3,
        40..=53 => 4,
        _ => unreachable!(),
    }
}

// Function to add support for %Q (year quarter) not available on chrono crate.
fn process_format_string(format: &str) -> String {
    let week_number = Local::now().naive_local().iso_week().week();
    let quarter = quarter_from_week(week_number);

    format.replace("%Q", &quarter.to_string())
}

// TODO: Test this function
pub fn current_date_formatted(format: &str) -> String {
    let current_date = Local::now();

    let formatted = process_format_string(format);

    let date_formatted = current_date.format(&formatted);

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
    fn test_process_format_string() {
        let test_cases = [
            (1, "Q1"),
            (13, "Q1"),
            (14, "Q2"),
            (26, "Q2"),
            (27, "Q3"),
            (39, "Q3"),
            (40, "Q4"),
            (53, "Q4"),
        ];

        for (week, expected_quarter) in test_cases {
            // This format outputs the quarter as "Q1", "Q2", "Q3", or "Q4" based on the week number.
            let format = "Q%Q".to_string();

            let result =
                process_format_string(&format.replace("%Q", &quarter_from_week(week).to_string()));

            assert_eq!(result, expected_quarter, "Failed on week: {}", result);
        }
    }
}
