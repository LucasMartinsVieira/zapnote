use std::env;

use chrono::{Datelike, Local};
use regex::Regex;

use crate::utils::quarter_from_week;

pub struct Placeholder;

impl Placeholder {
    pub fn parse(mut template: String) -> String {
        let time_regex_pattern = r"\{\{time(:([^{}]+))?\}\}";
        let date_regex_pattern = r"\{\{date(:([^{}]+))?\}\}";

        Self::parse_title(&mut template);

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

    fn parse_title(template: &mut String) {
        let note_title =
            env::var("ZAPNOTE_NOTE_TITLE").unwrap_or("Note title not defined".to_string());

        *template = template.replace("{{title}}", &note_title)
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
