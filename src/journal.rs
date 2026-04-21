use crate::utils::{
    date::resolve_reference_date,
    template::{check_template, insert_template_journal, specific_template_info},
};

pub fn handle_journal_command(
    name: &str,
    date: Option<&str>,
    offset: Option<&str>,
) -> Result<String, Box<dyn std::error::Error>> {
    let journal = specific_template_info(name)?;

    check_template(&journal.template)?;

    let reference_date = resolve_reference_date(date, offset)
        .map_err(|err| std::io::Error::other(err.to_string()))?;

    insert_template_journal(&journal, reference_date)
}
