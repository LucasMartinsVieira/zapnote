use std::env;

use chrono::{Datelike, Local};
use regex::Regex;

use crate::utils::quarter_from_week;

pub struct Placeholder;

impl Placeholder {
    pub fn parse(mut template: String) -> String {
        let time_regex_pattern = r"\{\{time(:([^{}]+))?\}\}";
        let date_regex_pattern = r"\{\{date(:([^{}]+))?\}\}";

        let note_title =
            env::var("ZAPNOTE_NOTE_TITLE").unwrap_or("Note title not defined".to_string());

        Self::parse_title(&mut template, &note_title);

        Self::parse_placeholder(&mut template, time_regex_pattern, |caps| {
            let format = caps.get(2).map_or("%H:%M", |m| m.as_str());
            Self::format_with_quarter(format)
        });

        Self::parse_placeholder(&mut template, date_regex_pattern, |caps| {
            let format = caps.get(2).map_or("%Y-%m-%d", |m| m.as_str());
            Self::format_with_quarter(format)
        });

        template
    }

    fn parse_title(template: &mut String, note_title: &str) {
        *template = template.replace("{{title}}", note_title)
    }

    fn parse_placeholder<F>(template: &mut String, regex_pattern: &str, replacement_fn: F)
    where
        F: Fn(&regex::Captures) -> String,
    {
        let regex = Regex::new(regex_pattern).unwrap();

        *template = regex
            .replace_all(template, |caps: &regex::Captures| replacement_fn(caps))
            .to_string();
    }

    fn format_with_quarter(format: &str) -> String {
        let now = Local::now();
        let quarter = quarter_from_week(now.iso_week().week());

        let formatted_date = format.replace("%Q", &quarter.to_string());

        now.format(&formatted_date).to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_title() {
        let note_title = "Some kind of Wonderfull";
        let mut template = "{{title}}".to_string();

        Placeholder::parse_title(&mut template, note_title);

        assert_eq!(template, note_title);
    }

    #[test]
    fn test_parse_title_wrong_placeholder() {
        let note_title = "Young Turks";
        let mut template = "{{titl}}".to_string();

        Placeholder::parse_title(&mut template, note_title);

        assert_ne!(template, note_title);
    }

    #[test]
    fn test_format_with_quarter() {
        let quarter_format = "%Q";

        let now = Local::now();
        let quarter = quarter_from_week(now.iso_week().week()).to_string();

        let formatted = Placeholder::format_with_quarter(quarter_format);

        assert_eq!(formatted, quarter);
    }

    #[test]
    fn test_parse_placeholder_time() {
        let mut template = "{{time}}".to_string();
        let time_regex_pattern = r"\{\{time(:([^{}]+))?\}\}";

        let current_time = Local::now().format("%H:%M").to_string();

        Placeholder::parse_placeholder(&mut template, time_regex_pattern, |caps| {
            let format = caps.get(2).map_or("%H:%M", |m| m.as_str());
            Placeholder::format_with_quarter(format)
        });

        assert_eq!(template, current_time);
    }

    #[test]
    fn test_parse_placeholder_date() {
        let mut template = "{{date}}".to_string();
        let date_regex_pattern = r"\{\{date(:([^{}]+))?\}\}";

        let current_date = Local::now().format("%Y-%m-%d").to_string();

        Placeholder::parse_placeholder(&mut template, date_regex_pattern, |caps| {
            let format = caps.get(2).map_or("%Y-%m-%d", |m| m.as_str());
            Placeholder::format_with_quarter(format)
        });

        assert_eq!(template, current_date);
    }

    #[test]
    fn test_parse_placeholder_with_quarter_template() {
        let mut template = "{{date:%Y-Q%Q}}".to_string();
        let date_regex_pattern = r"\{\{date(:([^{}]+))?\}\}";

        let current_year = Local::now().format("%Y").to_string();
        let current_week = Local::now().iso_week().week();

        let current_quarter = quarter_from_week(current_week);

        let asserted = format!("{}-Q{}", current_year, current_quarter);

        Placeholder::parse_placeholder(&mut template, date_regex_pattern, |caps| {
            let format = caps.get(2).map_or("%Y-%m-%d", |m| m.as_str());
            Placeholder::format_with_quarter(format)
        });

        assert_eq!(template, asserted);
    }
}
