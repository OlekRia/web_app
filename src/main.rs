mod state;
mod to_do;

use serde_json::json;
use state::{read_file, write_to_file, Json};
use std::env;
// use to_do::enums::TaskStatus::DONE;
// use to_do::traits::{delete::Delete, edit::Edit, get::Get};
// use to_do::{to_do_factory, ItemTypes};

const FILE_NAME: &str = "./state.json";

fn main() {
    let args: Vec<String> = env::args().collect();
    let status: &String = &args[1];
    let title: &String = &args[2];

    let mut state: Json = read_file(FILE_NAME);

    println!("Before operation: {:?}", state);
    state.insert(title.to_string(), json!(status));
    println!("Before operation: {:?}", state);
    write_to_file(FILE_NAME, &mut state);

    // let to_do_item = to_do_factory("washing", DONE);

    // match to_do_item {
    //     ItemTypes::Done(done) => {
    //         done.get(&done.super_struct.title);
    //         done.delete(&done.super_struct.title);
    //     }
    //     ItemTypes::Pending(pending) => {
    //         pending.get(&pending.super_struct.title);
    //         pending.set_to_done(&pending.super_struct.title);
    //     }
    // }
}
