use crate::utils::get_template_folder_path;

pub fn handle_note_command(_template: &str, _name: &str) {
    let template_path = get_template_folder_path();

    match template_path {
        Ok(path) => println!("Template path: {}", path),
        Err(err) => println!("Error: {}", err),
    }
}
