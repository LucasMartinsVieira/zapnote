use crate::{
    config::Subcommand,
    utils::{check_template, insert_template_into_file, template_folder_path},
};

pub fn handle_note_command(template: &str, name: &str) {
    let template_path = template_folder_path();

    match template_path {
        Ok(path) => println!("Template path: {}", path),
        Err(err) => println!("Error: {}", err),
    }

    // If the template specified by the user doesn't exist on the path established by the user or
    // there is already a note with the same name specified by the user, the program exits with status code 1
    // TODO: Do a better error handling?
    check_template(&template, Subcommand::Note, &name).unwrap();

    insert_template_into_file(template.to_owned(), name.to_owned(), Subcommand::Note).unwrap();
}
