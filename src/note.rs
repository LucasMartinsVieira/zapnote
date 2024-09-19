use crate::{
    config::Sub,
    utils::{check_note_name, check_template, insert_template_into_file},
};

pub fn handle_note_command(template: &str, name: &str) {
    // If the template specified by the user doesn't exist on the path established by the user or
    // there is already a note with the same name specified by the user, the program exits with status code 1
    // TODO: Do a better error handling?
    check_template(template).unwrap();

    check_note_name(name, Sub::Note).unwrap();

    insert_template_into_file(template.to_owned(), name.to_owned(), Sub::Note).unwrap();
}
