use std::{
    fs,
    path::{Path, PathBuf},
    process::Command,
};

use tempfile::TempDir;

fn test_env() -> (TempDir, PathBuf) {
    let temp_dir = TempDir::new().unwrap();
    let root = temp_dir.path();
    let templates = root.join("templates");
    let notes = root.join("notes");
    let journal_root = root.join("journal");

    fs::create_dir_all(&templates).unwrap();
    fs::create_dir_all(&notes).unwrap();
    fs::create_dir_all(&journal_root).unwrap();

    fs::write(templates.join("daily.md"), "# {{title}}").unwrap();
    fs::write(templates.join("meeting.md"), "# {{title}}").unwrap();
    fs::write(templates.join("weekly.md"), "# {{title}}").unwrap();

    let config = format!(
        r#"[general]
template_folder_path = "{}"
editor = ""
note_folder_path = "{}"
journal_folder_path = "{}"
note_case_style = "original"

[[journal]]
name = "day"
format = "%Y-%m-%d"
template = "daily"
folder_path = "daily"

[[journal]]
name = "week"
format = "%G-W%V"
template = "weekly"
folder_path = "weekly"
"#,
        templates.display(),
        notes.display(),
        journal_root.display(),
    );

    let config_path = root.join("zapnote.toml");
    fs::write(&config_path, config).unwrap();

    (temp_dir, config_path)
}

fn run_zn(config_path: &Path, args: &[&str]) -> std::process::Output {
    Command::new(env!("CARGO_BIN_EXE_zn"))
        .arg("--config")
        .arg(config_path)
        .args(args)
        .output()
        .unwrap()
}

fn run_zn_raw(args: &[&str]) -> std::process::Output {
    Command::new(env!("CARGO_BIN_EXE_zn"))
        .args(args)
        .output()
        .unwrap()
}

#[test]
fn list_templates_json_returns_structured_entries() {
    let (_temp_dir, config_path) = test_env();
    let output = run_zn(&config_path, &["list", "templates", "--json"]);

    assert!(output.status.success());

    let stdout = String::from_utf8(output.stdout).unwrap();
    assert!(stdout.contains("\"name\": \"daily\""));
    assert!(stdout.contains("\"path\":"));
}

#[test]
fn list_journals_text_prints_names_only() {
    let (_temp_dir, config_path) = test_env();
    let output = run_zn(&config_path, &["list", "journals"]);

    assert!(output.status.success());

    let stdout = String::from_utf8(output.stdout).unwrap();
    assert_eq!(stdout.lines().collect::<Vec<_>>(), vec!["day", "week"]);
}

#[test]
fn note_no_editor_prints_created_path() {
    let (_temp_dir, config_path) = test_env();
    let output = run_zn(
        &config_path,
        &["--no-editor", "note", "meeting", "Project Kickoff"],
    );

    assert!(output.status.success());

    let stdout = String::from_utf8(output.stdout).unwrap();
    assert!(stdout.trim_end().ends_with("notes/Project Kickoff.md"));
}

#[test]
fn note_no_editor_prints_existing_path_for_duplicates() {
    let (temp_dir, config_path) = test_env();
    let existing_path = temp_dir.path().join("notes").join("Project Kickoff.md");
    fs::write(&existing_path, "# existing").unwrap();

    let output = run_zn(
        &config_path,
        &["--no-editor", "note", "meeting", "Project Kickoff"],
    );

    assert!(output.status.success());

    let stdout = String::from_utf8(output.stdout).unwrap();
    assert_eq!(stdout.trim_end(), existing_path.to_string_lossy());
}

#[test]
fn journal_no_editor_prints_existing_path() {
    let (temp_dir, config_path) = test_env();
    let existing_path = temp_dir
        .path()
        .join("journal")
        .join("daily")
        .join("2026-04-19.md");
    fs::create_dir_all(existing_path.parent().unwrap()).unwrap();
    fs::write(&existing_path, "# daily").unwrap();

    let output = run_zn(
        &config_path,
        &["--no-editor", "journal", "day", "--date", "2026-04-19"],
    );

    assert!(output.status.success());

    let stdout = String::from_utf8(output.stdout).unwrap();
    assert_eq!(stdout.trim_end(), existing_path.to_string_lossy());
}

#[test]
fn help_flag_prints_help_without_debug_error_wrapper() {
    let output = run_zn_raw(&["--help"]);

    assert!(output.status.success());

    let stdout = String::from_utf8(output.stdout).unwrap();
    let stderr = String::from_utf8(output.stderr).unwrap();

    assert!(stdout.contains("Usage:"));
    assert!(stdout.contains("completion"));
    assert!(stderr.is_empty());
}

#[test]
fn version_flag_prints_version_without_debug_error_wrapper() {
    let output = run_zn_raw(&["--version"]);

    assert!(output.status.success());

    let stdout = String::from_utf8(output.stdout).unwrap();
    let stderr = String::from_utf8(output.stderr).unwrap();

    assert!(stdout.contains("zn 0.2.0"));
    assert!(stderr.is_empty());
}

#[test]
fn missing_subcommand_prints_clap_message_instead_of_debug_error() {
    let output = run_zn_raw(&["--no-editor"]);

    assert!(!output.status.success());

    let stderr = String::from_utf8(output.stderr).unwrap();

    assert!(stderr.contains("Usage:"));
    assert!(!stderr.contains("ErrorInner"));
}

#[test]
fn completion_output_uses_zn_binary_name() {
    let output = run_zn_raw(&["completion", "bash"]);

    assert!(output.status.success());

    let stdout = String::from_utf8(output.stdout).unwrap();
    assert!(stdout.contains("_zn()"));
    assert!(stdout.contains("cmd=\"zn\""));
    assert!(!stdout.contains("_zapnote()"));
    assert!(!stdout.contains("cmd=\"zapnote\""));
}

#[test]
#[ignore]
fn build_script_generates_completion_files_for_zn() {
    let completions_dir = Path::new(env!("CARGO_MANIFEST_DIR")).join("target/completions");

    assert!(completions_dir.exists());

    let bash = fs::read_to_string(completions_dir.join("zn.bash")).unwrap();
    let fish = fs::read_to_string(completions_dir.join("zn.fish")).unwrap();
    let zsh = fs::read_to_string(completions_dir.join("_zn")).unwrap();

    assert!(bash.contains("_zn()"));
    assert!(fish.contains("complete -c zn"));
    assert!(zsh.contains("#compdef zn"));
}
