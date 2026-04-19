use crate::{
    config::Sub,
    utils::{
        date::resolve_reference_date,
        template::{check_template, insert_template_journal, specific_template_info},
    },
};
use std::process;

pub fn handle_journal_command(name: &str, date: Option<&str>, offset: Option<&str>) {
    let template_hashmap = specific_template_info(Sub::Journal, name).unwrap();

    if let Some(template) = template_hashmap.get("template") {
        check_template(template).unwrap();
    }

    let reference_date = match resolve_reference_date(date, offset) {
        Ok(reference_date) => reference_date,
        Err(err) => {
            eprintln!("{err}");
            process::exit(1);
        }
    };

    insert_template_journal(template_hashmap, reference_date).unwrap();
}
