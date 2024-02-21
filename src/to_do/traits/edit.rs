use super::super::data::FILE_NAME;
use crate::{state::write_to_file, to_do::enums::TaskStatus};
use serde_json::{json, Map, Value};

pub trait Edit {
    fn set_to_done(&self, title: &str, state: &mut Map<String, Value>) {
        state.insert(title.to_string(), json!(TaskStatus::DONE.to_string()));
        write_to_file(FILE_NAME, state);

        println!("\n\n{} is being set to done\n\n", title);
    }

    fn set_to_pending(&self, title: &str, state: &mut Map<String, Value>) {
        state.insert(title.to_string(), json!(TaskStatus::PENDING.to_string()));
        write_to_file(FILE_NAME, state);

        println!("\n\n{} is being set to pending\n\n", title);
    }
}
