use std::{
    fs, io,
    path::{Path, PathBuf},
};

use crate::{
    config::{Config, JournalConfig, Sub},
    utils::{
        alternate_path,
        date::format_date,
        open_path_in_editor,
        placeholder::{Placeholder, TemplateContext},
    },
};

use chrono::NaiveDate;
use serde::Serialize;

use super::{check_journal_note_path, command_folder_path};

#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
pub struct TemplateEntry {
    pub name: String,
    pub path: String,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
pub struct JournalEntry {
    pub name: String,
    pub format: String,
    pub template: String,
    pub folder_path: String,
}

pub fn template_folder_path() -> Result<String, Box<dyn std::error::Error>> {
    let config = Config::read()?;
    let template_path = alternate_path(config.general.template_folder_path);
    Ok(template_path)
}

pub fn template_entries() -> Result<Vec<TemplateEntry>, Box<dyn std::error::Error>> {
    let folder_path = template_folder_path()?;
    let templates = templates_in_folder(folder_path.clone())
        .ok_or_else(|| io::Error::other("no templates found in template folder"))?;

    Ok(templates
        .into_iter()
        .map(|name| TemplateEntry {
            path: PathBuf::from(&folder_path)
                .join(format!("{name}.md"))
                .to_string_lossy()
                .into_owned(),
            name,
        })
        .collect())
}

pub fn journal_entries() -> Result<Vec<JournalEntry>, Box<dyn std::error::Error>> {
    let config = Config::read()?;
    Ok(config
        .journal
        .unwrap_or_default()
        .into_iter()
        .map(|entry| JournalEntry {
            name: entry.name,
            format: entry.format,
            template: entry.template,
            folder_path: entry.folder_path,
        })
        .collect())
}

pub fn specific_template_info(name: &str) -> Result<JournalConfig, Box<dyn std::error::Error>> {
    let config = Config::read()?;
    config
        .journal
        .unwrap_or_default()
        .into_iter()
        .find(|entry| entry.name == name)
        .ok_or_else(|| io::Error::other(format!("no journal entry found for '{name}'")).into())
}

pub fn templates_in_folder(path: String) -> Option<Vec<String>> {
    let mut dir_contents: Vec<String> = fs::read_dir(path)
        .ok()?
        .filter_map(|entry| entry.ok())
        .filter_map(|entry| entry.file_name().into_string().ok())
        .filter(|name| name.ends_with(".md"))
        .map(|name| name.trim_end_matches(".md").to_string())
        .collect();

    dir_contents.sort();
    Some(dir_contents)
}

pub fn check_template(template: &str) -> Result<(), Box<dyn std::error::Error>> {
    let template_names: Vec<String> = template_entries()?
        .into_iter()
        .map(|entry| entry.name)
        .collect();

    if !template_names.contains(&template.to_owned()) {
        return Err(io::Error::other(format!(
            "template '{template}' doesn't exist in template folder. available templates: {}",
            template_names.join(", ")
        ))
        .into());
    }

    Ok(())
}

pub fn template_file_contents(template: String, context: &TemplateContext) -> Option<String> {
    let template_folder_path = template_folder_path().ok();

    if let Some(path) = template_folder_path {
        let mut template_file_path = PathBuf::from(path);
        let template_name_with_extension = format!("{template}.md");

        template_file_path.push(&template_name_with_extension);

        let template_file_contents = fs::read_to_string(template_file_path).ok()?;
        let parsed_template_file_contents = Placeholder::parse(template_file_contents, context);

        Some(parsed_template_file_contents)
    } else {
        None
    }
}

pub fn write_template_to_file(
    full_path: String,
    template: String,
    context: &TemplateContext,
) -> Result<(), Box<dyn std::error::Error>> {
    let template_file_contents = template_file_contents(template.clone(), context)
        .ok_or_else(|| io::Error::other(format!("failed to load template '{template}'")))?;

    if let Some(parent) = Path::new(&full_path).parent() {
        fs::create_dir_all(parent)?;
    }

    fs::write(&full_path, template_file_contents)?;
    open_path_in_editor(&full_path)?;

    Ok(())
}

pub fn insert_template_to_file(
    template: String,
    name: String,
    command: Sub,
) -> Result<String, Box<dyn std::error::Error>> {
    let command_path_str = command_folder_path(command)?;
    let full_path = PathBuf::from(command_path_str)
        .join(format!("{name}.md"))
        .to_string_lossy()
        .into_owned();

    let context = TemplateContext::new(name, chrono::Local::now().date_naive());

    write_template_to_file(full_path.clone(), template, &context)?;
    Ok(full_path)
}

pub fn insert_template_journal(
    journal: &JournalConfig,
    reference_date: NaiveDate,
) -> Result<String, Box<dyn std::error::Error>> {
    let date_formatted = format_date(reference_date, &journal.format);
    let command_path_str = command_folder_path(Sub::Journal)?;
    let full_path = PathBuf::from(command_path_str)
        .join(&journal.folder_path)
        .join(format!("{date_formatted}.md"))
        .to_string_lossy()
        .into_owned();

    if let Some(existing_path) = check_journal_note_path(&full_path) {
        open_path_in_editor(&existing_path)?;
        return Ok(existing_path);
    }

    let context = TemplateContext::new(date_formatted, reference_date);

    write_template_to_file(full_path.clone(), journal.template.clone(), &context)?;
    Ok(full_path)
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

    #[test]
    fn template_entries_include_template_paths() {
        let template_dir = TempDir::new().unwrap();
        let template_path = template_dir.path().join("daily.md");
        fs::write(&template_path, "content").unwrap();

        let entries = templates_in_folder(template_dir.path().to_string_lossy().into_owned())
            .unwrap()
            .into_iter()
            .map(|name| TemplateEntry {
                path: template_dir
                    .path()
                    .join(format!("{name}.md"))
                    .to_string_lossy()
                    .into_owned(),
                name,
            })
            .collect::<Vec<_>>();

        assert_eq!(
            entries,
            vec![TemplateEntry {
                name: "daily".to_string(),
                path: template_path.to_string_lossy().into_owned(),
            }]
        );
    }
}
