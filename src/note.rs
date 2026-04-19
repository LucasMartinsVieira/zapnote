use crate::{
    config::Sub,
    utils::{
        check_note_name, open_path_in_editor,
        template::{check_template, insert_template_to_file},
    },
};

pub fn handle_note_command(template: &str, note_name: String) {
    check_template(template).unwrap();

    if let Some(existing_path) = check_note_name(&note_name, Sub::Note).unwrap() {
        println!(
            "note already exists at '{}'; opening existing note",
            existing_path
        );
        open_path_in_editor(&existing_path).unwrap();
        return;
    }

    insert_template_to_file(template.to_owned(), note_name.to_owned(), Sub::Note).unwrap();
}
