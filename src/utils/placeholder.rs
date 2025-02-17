use std::env;

use chrono::Local;
use regex::Regex;

pub struct Placeholder;

impl Placeholder {
    pub fn parse(mut template: String) -> String {
        Self::parse_title(&mut template);
        Self::parse_time(&mut template);
        Self::parse_date(&mut template);

        template
    }

    fn parse_title(template: &mut String) {
        let note_title =
            env::var("ZAPNOTE_NOTE_TITLE").unwrap_or("Note title not defined".to_string());

        *template = template.replace("{{title}}", &note_title)
    }

    fn parse_time(template: &mut String) {
        let regex = Regex::new(r"\{\{time(:([^{}]+))?\}\}").unwrap();

        *template = regex
            .replace_all(template, |caps: &regex::Captures| {
                let format = caps.get(2).map_or("%H:%M", |m| m.as_str());
                let local_time = Local::now().format(format).to_string();
                local_time
            })
            .to_string();
    }

    fn parse_date(template: &mut String) {
        let regex = Regex::new(r"\{\{date(:([^{}]+))?\}\}").unwrap();

        *template = regex
            .replace_all(template, |caps: &regex::Captures| {
                let format = caps.get(2).map_or("%Y-%m-%d", |m| m.as_str());
                let local_date = Local::now().format(format).to_string();
                local_date
            })
            .to_string();
    }
}
