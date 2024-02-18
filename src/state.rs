use serde_json::{json, value::Value, Map};
use std::io::Read;
use std::{fs, fs::File};

pub type Json = Map<String, Value>;

pub fn read_file(file_name: &str) -> Json {
    let mut file = File::open(file_name.to_string()).unwrap();
    let mut data = String::new();

    file.read_to_string(&mut data).unwrap();
    let json: Value = serde_json::from_str(&data).unwrap();
    let state: Json = json.as_object().unwrap().clone();

    state
}

pub fn write_to_file(file_name: &str, state: &mut Json) {
    let new_data = json!(state);

    fs::write(file_name.to_string(), new_data.to_string()).expect("Unable to write file");
}
