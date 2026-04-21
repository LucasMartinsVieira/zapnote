use crate::{
    config::Sub,
    utils::{
        check_note_name, open_path_in_editor,
        template::{check_template, insert_template_to_file},
    },
};

pub fn handle_note_command(
    template: &str,
    note_name: String,
) -> Result<String, Box<dyn std::error::Error>> {
    check_template(template)?;

    if let Some(existing_path) = check_note_name(&note_name, Sub::Note)? {
        open_path_in_editor(&existing_path)?;
        return Ok(existing_path);
    }

    insert_template_to_file(template.to_owned(), note_name.to_owned(), Sub::Note)
}
