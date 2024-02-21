use to_do::ItemTypes;

use super::to_do::{
    structs::{done::Done, pending::Pending},
    traits::{create::Create, delete::Delete, edit::Edit, get::Get},
};
use serde_json::{value::Value, Map};

fn process_pending(item: Pending, command: String, state: &Map<String, Value>) {
    let mut state = state.clone();
    let t = &item.super_struct.title;
    let s = &item.super_struct.status.to_string();

    match command.as_str() {
        "get" => item.get(t, state),
        "create" => item.create(t, s, &mut state),
        "edit" => item.set_to_done(t, &mut state),
        _ => println!("command {} not supported (get, create, edit)", command),
    }
}

fn process_done(item: Done, command: String, state: &Map<String, Value>) {
    let mut state = state.clone();
    let t = &item.super_struct.title;

    match command.as_str() {
        "get" => item.get(t, state),
        "delete" => item.delete(t, &mut state),
        "edit" => item.set_to_pending(t, &mut state),
        _ => println!("command {} not supported (get, delete, edit)", command),
    }
}

pub fn process_input(item: ItemTypes, command: String, state: &Map<String, Value>) {
    match item {
        ItemTypes::Pending(item) => process_pending(item, command, state),
        ItemTypes::Done(item) => process_done(item, command, state),
    }
}
