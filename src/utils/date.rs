use chrono::{DateTime, Datelike, Days, Local, Months, NaiveDate, Weekday};
use regex::Regex;

use crate::utils::quarter_from_week;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum DateOffsetUnit {
    Day,
    Week,
    Quarter,
    Month,
    Year,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct DateOffset {
    amount: i32,
    unit: DateOffsetUnit,
}

pub fn format_date(date: NaiveDate, format: &str) -> String {
    let processed = replace_quarter(format, date);
    date.format(&processed).to_string()
}

pub fn format_datetime(datetime: DateTime<Local>, format: &str) -> String {
    let processed = replace_quarter(format, datetime.date_naive());
    datetime.format(&processed).to_string()
}

pub fn parse_date_offset(input: &str) -> Result<DateOffset, String> {
    let regex = Regex::new(
        r#"^\s*([+-]?\d+)\s*(day|days|week|weeks|quarter|quarters|month|months|year|years)\s*$"#,
    )
    .unwrap();
    let captures = regex
        .captures(input)
        .ok_or_else(|| format!("invalid offset '{input}'"))?;

    let amount = captures[1]
        .parse::<i32>()
        .map_err(|_| format!("invalid offset amount in '{input}'"))?;

    let unit = match &captures[2] {
        "day" | "days" => DateOffsetUnit::Day,
        "week" | "weeks" => DateOffsetUnit::Week,
        "quarter" | "quarters" => DateOffsetUnit::Quarter,
        "month" | "months" => DateOffsetUnit::Month,
        "year" | "years" => DateOffsetUnit::Year,
        _ => unreachable!(),
    };

    Ok(DateOffset { amount, unit })
}

pub fn apply_date_offset(date: NaiveDate, offset: DateOffset) -> NaiveDate {
    match offset.unit {
        DateOffsetUnit::Day => add_days(date, offset.amount),
        DateOffsetUnit::Week => add_days(date, offset.amount.saturating_mul(7)),
        DateOffsetUnit::Quarter => add_months(date, offset.amount.saturating_mul(3)),
        DateOffsetUnit::Month => add_months(date, offset.amount),
        DateOffsetUnit::Year => add_months(date, offset.amount.saturating_mul(12)),
    }
}

pub fn resolve_reference_date(
    date_input: Option<&str>,
    offset_input: Option<&str>,
) -> Result<NaiveDate, String> {
    let base_date = match date_input {
        Some(input) => parse_reference_date_input(input)?,
        None => Local::now().date_naive(),
    };

    match offset_input {
        Some(input) => {
            let offset = parse_date_offset(input)?;
            Ok(apply_date_offset(base_date, offset))
        }
        None => Ok(base_date),
    }
}

pub fn parse_reference_date_input(input: &str) -> Result<NaiveDate, String> {
    NaiveDate::parse_from_str(input, "%Y-%m-%d")
        .or_else(|_| parse_iso_week_date(input))
        .or_else(|_| parse_quarter_date(input))
        .or_else(|_| parse_month_date(input))
        .or_else(|_| parse_year_date(input))
        .map_err(|_| {
            format!(
                "unsupported --date '{input}'. use YYYY-MM-DD, YYYY-W01, YYYY-Q1, YYYY-MM, or YYYY"
            )
        })
}

fn add_days(date: NaiveDate, amount: i32) -> NaiveDate {
    if amount >= 0 {
        date.checked_add_days(Days::new(amount as u64)).unwrap()
    } else {
        date.checked_sub_days(Days::new(amount.unsigned_abs() as u64))
            .unwrap()
    }
}

fn add_months(date: NaiveDate, amount: i32) -> NaiveDate {
    if amount >= 0 {
        date.checked_add_months(Months::new(amount as u32)).unwrap()
    } else {
        date.checked_sub_months(Months::new(amount.unsigned_abs()))
            .unwrap()
    }
}

fn replace_quarter<T: Datelike>(format: &str, date: T) -> String {
    let quarter = quarter_from_week(date.iso_week().week());
    format.replace("%Q", &quarter.to_string())
}

fn parse_iso_week_date(input: &str) -> Result<NaiveDate, String> {
    let regex = Regex::new(r"^(\d{4})-W(\d{1,2})$").unwrap();
    let captures = regex
        .captures(input)
        .ok_or_else(|| "expected --date in format 2026-W01".to_string())?;

    let year = captures[1]
        .parse::<i32>()
        .map_err(|_| "invalid ISO week year".to_string())?;
    let week = captures[2]
        .parse::<u32>()
        .map_err(|_| "invalid ISO week number".to_string())?;

    NaiveDate::from_isoywd_opt(year, week, Weekday::Mon)
        .ok_or_else(|| format!("invalid ISO week '{input}'"))
}

fn parse_month_date(input: &str) -> Result<NaiveDate, String> {
    let regex = Regex::new(r"^(\d{4})-(\d{2})$").unwrap();
    let captures = regex
        .captures(input)
        .ok_or_else(|| "expected --date in format 2026-04".to_string())?;

    let year = captures[1]
        .parse::<i32>()
        .map_err(|_| "invalid month year".to_string())?;
    let month = captures[2]
        .parse::<u32>()
        .map_err(|_| "invalid month value".to_string())?;

    NaiveDate::from_ymd_opt(year, month, 1).ok_or_else(|| format!("invalid month '{input}'"))
}

fn parse_quarter_date(input: &str) -> Result<NaiveDate, String> {
    let regex = Regex::new(r"^(\d{4})-Q([1-4])$").unwrap();
    let captures = regex
        .captures(input)
        .ok_or_else(|| "expected --date in format 2026-Q1".to_string())?;

    let year = captures[1]
        .parse::<i32>()
        .map_err(|_| "invalid quarter year".to_string())?;
    let quarter = captures[2]
        .parse::<u32>()
        .map_err(|_| "invalid quarter value".to_string())?;
    let month = match quarter {
        1 => 1,
        2 => 4,
        3 => 7,
        4 => 10,
        _ => unreachable!(),
    };

    NaiveDate::from_ymd_opt(year, month, 1).ok_or_else(|| format!("invalid quarter '{input}'"))
}

fn parse_year_date(input: &str) -> Result<NaiveDate, String> {
    let regex = Regex::new(r"^(\d{4})$").unwrap();
    let captures = regex
        .captures(input)
        .ok_or_else(|| "expected --date in format 2026".to_string())?;

    let year = captures[1]
        .parse::<i32>()
        .map_err(|_| "invalid year value".to_string())?;

    NaiveDate::from_ymd_opt(year, 1, 1).ok_or_else(|| format!("invalid year '{input}'"))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_iso_week_input_with_single_digit_week() {
        let result = parse_reference_date_input("2026-W1").unwrap();

        assert_eq!(result, NaiveDate::from_ymd_opt(2025, 12, 29).unwrap());
    }

    #[test]
    fn parses_month_input_to_first_day() {
        let result = parse_reference_date_input("2026-04").unwrap();

        assert_eq!(result, NaiveDate::from_ymd_opt(2026, 4, 1).unwrap());
    }

    #[test]
    fn parses_year_input_to_first_day() {
        let result = parse_reference_date_input("2026").unwrap();

        assert_eq!(result, NaiveDate::from_ymd_opt(2026, 1, 1).unwrap());
    }

    #[test]
    fn parses_quarter_input_to_first_day() {
        let result = parse_reference_date_input("2026-Q2").unwrap();

        assert_eq!(result, NaiveDate::from_ymd_opt(2026, 4, 1).unwrap());
    }

    #[test]
    fn applies_offsets_with_calendar_months() {
        let date = NaiveDate::from_ymd_opt(2024, 1, 31).unwrap();
        let offset = parse_date_offset("+1 month").unwrap();

        assert_eq!(
            apply_date_offset(date, offset),
            NaiveDate::from_ymd_opt(2024, 2, 29).unwrap()
        );
    }

    #[test]
    fn formats_quarter_from_reference_date() {
        let date = NaiveDate::from_ymd_opt(2026, 4, 19).unwrap();

        assert_eq!(format_date(date, "%Y-Q%Q"), "2026-Q2");
    }

    #[test]
    fn applies_quarter_offsets_as_three_months() {
        let date = NaiveDate::from_ymd_opt(2024, 1, 31).unwrap();
        let offset = parse_date_offset("+1 quarter").unwrap();

        assert_eq!(
            apply_date_offset(date, offset),
            NaiveDate::from_ymd_opt(2024, 4, 30).unwrap()
        );
    }

    #[test]
    fn resolves_reference_date_with_quarter_offset() {
        let result = resolve_reference_date(Some("2026-04-19"), Some("-1 quarter")).unwrap();

        assert_eq!(result, NaiveDate::from_ymd_opt(2026, 1, 19).unwrap());
    }

    #[test]
    fn rejects_invalid_iso_week_input() {
        let error = parse_reference_date_input("2026-W54").unwrap_err();

        assert_eq!(
            error,
            "unsupported --date '2026-W54'. use YYYY-MM-DD, YYYY-W01, YYYY-Q1, YYYY-MM, or YYYY"
        );
    }

    #[test]
    fn resolves_reference_date_with_offset() {
        let result = resolve_reference_date(Some("2026-04-19"), Some("+1 year")).unwrap();

        assert_eq!(result, NaiveDate::from_ymd_opt(2027, 4, 19).unwrap());
    }
}
