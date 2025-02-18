use convert_case::{Case, Casing};

use crate::config::{CaseStyle, Config};

pub fn convert_case(note_title: String) -> String {
    let config = Config::read().unwrap();

    if let Some(case) = config.general.note_case_style {
        match case {
            CaseStyle::Camel => note_title.to_case(Case::Camel),
            CaseStyle::Kebab => note_title.to_case(Case::Kebab),
            CaseStyle::Pascal => note_title.to_case(Case::Pascal),
            CaseStyle::Snake => note_title.to_case(Case::Snake),
            CaseStyle::Original => note_title.to_string(),
        }
    } else {
        note_title
    }
}
