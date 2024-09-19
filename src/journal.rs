use crate::{config::Sub, utils::template::specific_template_info};

pub fn handle_journal_commmand(name: &str) {
    let a = specific_template_info(Sub::Journal, name).unwrap();

    println!("{:#?}", a)
}
