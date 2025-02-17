use std::env;

use chrono::Local;

pub struct Placeholder;

// TODO: Make this functions work with more than just "{{something}}"
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
        let local_time = Local::now().format("%H:%M").to_string();

        *template = template.replace("{{time}}", &local_time)
    }

    fn parse_date(template: &mut String) {
        let local_date = Local::now().format("%Y-%m-%d").to_string();

        *template = template.replace("{{date}}", &local_date)
    }
}
