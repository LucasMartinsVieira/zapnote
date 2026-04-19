use chrono::{DateTime, Local, NaiveDate};
use regex::Regex;

use crate::utils::date::{apply_date_offset, format_date, format_datetime, parse_date_offset};

pub struct TemplateContext {
    pub title: String,
    pub now: DateTime<Local>,
    pub reference_date: NaiveDate,
}

impl TemplateContext {
    pub fn new(title: String, reference_date: NaiveDate) -> Self {
        Self {
            title,
            now: Local::now(),
            reference_date,
        }
    }
}

pub struct Placeholder;

impl Placeholder {
    pub fn parse(template: String, context: &TemplateContext) -> String {
        let regex = Regex::new(r"\{\{([^{}]+)\}\}").unwrap();

        regex
            .replace_all(&template, |caps: &regex::Captures| {
                let raw = caps.get(1).unwrap().as_str().trim();

                Self::render_placeholder(raw, context)
                    .unwrap_or_else(|| caps.get(0).unwrap().as_str().to_string())
            })
            .to_string()
    }

    fn render_placeholder(raw: &str, context: &TemplateContext) -> Option<String> {
        if raw == "title" {
            return Some(context.title.clone());
        }

        if raw == "date" {
            return Some(format_date(context.reference_date, "%Y-%m-%d"));
        }

        if raw == "time" {
            return Some(format_datetime(context.now, "%H:%M"));
        }

        if let Some(format) = raw.strip_prefix("date:") {
            return Some(format_date(context.reference_date, format));
        }

        if let Some(format) = raw.strip_prefix("time:") {
            return Some(format_datetime(context.now, format));
        }

        if let Some(attributes) = raw.strip_prefix("date ") {
            return Self::render_date_with_attributes(attributes, context);
        }

        if let Some(attributes) = raw.strip_prefix("time ") {
            return Self::render_time_with_attributes(attributes, context);
        }

        None
    }

    fn render_date_with_attributes(attributes: &str, context: &TemplateContext) -> Option<String> {
        let parsed = Self::parse_attributes(attributes);
        let mut date = context.reference_date;

        if let Some(offset) = parsed.get("offset") {
            let parsed_offset = parse_date_offset(offset).ok()?;
            date = apply_date_offset(date, parsed_offset);
        }

        let format = parsed
            .get("format")
            .map(String::as_str)
            .unwrap_or("%Y-%m-%d");

        Some(format_date(date, format))
    }

    fn render_time_with_attributes(attributes: &str, context: &TemplateContext) -> Option<String> {
        let parsed = Self::parse_attributes(attributes);
        let format = parsed.get("format").map(String::as_str).unwrap_or("%H:%M");

        Some(format_datetime(context.now, format))
    }

    fn parse_attributes(attributes: &str) -> std::collections::HashMap<String, String> {
        let regex = Regex::new(r#"([a-zA-Z_]+)\s*=\s*"([^"]*)""#).unwrap();

        regex
            .captures_iter(attributes)
            .map(|captures| (captures[1].to_string(), captures[2].to_string()))
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn context(reference_date: NaiveDate) -> TemplateContext {
        TemplateContext::new("Daily Note".to_string(), reference_date)
    }

    #[test]
    fn parses_title_placeholder() {
        let rendered = Placeholder::parse(
            "{{title}}".to_string(),
            &context(NaiveDate::from_ymd_opt(2026, 4, 19).unwrap()),
        );

        assert_eq!(rendered, "Daily Note");
    }

    #[test]
    fn keeps_legacy_date_format_placeholder() {
        let rendered = Placeholder::parse(
            "{{date:%Y-Q%Q}}".to_string(),
            &context(NaiveDate::from_ymd_opt(2026, 4, 19).unwrap()),
        );

        assert_eq!(rendered, "2026-Q2");
    }

    #[test]
    fn renders_date_with_offset_attributes() {
        let rendered = Placeholder::parse(
            "{{date offset=\"-1 day\" format=\"%Y-%m-%d\"}}".to_string(),
            &context(NaiveDate::from_ymd_opt(2026, 4, 19).unwrap()),
        );

        assert_eq!(rendered, "2026-04-18");
    }

    #[test]
    fn renders_iso_week_from_reference_date() {
        let rendered = Placeholder::parse(
            "{{date format=\"%G-W%V\"}}".to_string(),
            &context(NaiveDate::from_ymd_opt(2026, 4, 19).unwrap()),
        );

        assert_eq!(rendered, "2026-W16");
    }

    #[test]
    fn leaves_unknown_placeholders_unchanged() {
        let rendered = Placeholder::parse(
            "{{unknown}}".to_string(),
            &context(NaiveDate::from_ymd_opt(2026, 4, 19).unwrap()),
        );

        assert_eq!(rendered, "{{unknown}}");
    }
}
