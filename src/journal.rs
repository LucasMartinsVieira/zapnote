use crate::{
    config::Sub,
    utils::{
        check_note_name,
        template::{check_template, insert_template_journal, specific_template_info},
    },
};

pub fn handle_journal_command(name: &str) {
    let template_hashmap = specific_template_info(Sub::Journal, name).unwrap();

    if let Some(template) = template_hashmap.get("template") {
        check_template(template).unwrap();
    }

    check_note_name(name, Sub::Journal).unwrap();

    insert_template_journal(template_hashmap).unwrap();
}
