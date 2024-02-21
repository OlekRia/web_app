use crate::{
    json_serialization::to_do_items::ToDoItems,
    state::read_file,
    to_do::{data::FILE_NAME, enums::TaskStatus, to_do_factory},
};
use actix_web::{web, Responder};

pub async fn get() -> impl Responder {
    let state = read_file(FILE_NAME);
    let mut array_buffer = Vec::new();
    for (key, value) in state {
        let val = value.as_str().unwrap();
        let status = TaskStatus::from_string(val.to_string());
        let item = to_do_factory(&key, status);

        array_buffer.push(item);
    }

    let return_package = ToDoItems::new(array_buffer);

    web::Json(return_package)
}
