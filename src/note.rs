use crate::{
    config::SubcommandType,
    utils::{check_template, get_template_folder_path},
};

pub fn handle_note_command(template: &str, name: &str) {
    let template_path = get_template_folder_path();

    match template_path {
        Ok(path) => println!("Template path: {}", path),
        Err(err) => println!("Error: {}", err),
    }

    // If the template specified by the user doesn't exist on the path established by the user or
    // there is already a note with the same name specified by the user, the program exits with status code 1
    check_template(template.into(), SubcommandType::Note, name.to_owned()).unwrap();
}
