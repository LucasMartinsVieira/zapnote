use crate::{
    config::Sub,
    utils::{
        check_note_name,
        template::{check_template, insert_template_to_file},
    },
};

pub fn handle_note_command(template: &str, name: &[String]) {
    // If the template specified by the user doesn't exist on the path established by the user or
    // there is already a note with the same name specified by the user, the program exits with status code 1
    // TODO: Do a better error handling?
    check_template(template).unwrap();

    // Combine the &[String] into a single String, with each word separated by spaces
    let note_name = &name.join(" ");

    check_note_name(note_name, Sub::Note).unwrap();

    insert_template_to_file(template.to_owned(), note_name.to_owned(), Sub::Note).unwrap();
}
