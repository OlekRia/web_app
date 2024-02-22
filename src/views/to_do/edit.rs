use crate::{
    json_serialization::{to_do_item::ToDoItem, to_do_items::ToDoItems},
    processes::process_input,
    state::read_file,
    to_do::{data::FILE_NAME, enums::TaskStatus, to_do_factory},
};
use actix_web::{web, HttpResponse};

pub async fn edit(to_do_item: web::Json<ToDoItem>) -> HttpResponse {
    let state = read_file(FILE_NAME);
    let status: TaskStatus;

    match &state.get(&to_do_item.title) {
        Some(t) => {
            status = TaskStatus::from_string(t.as_str().unwrap().to_string());
        }
        None => {
            return HttpResponse::NotFound().json(format!("{} not in state", &to_do_item.title));
        }
    }

    let existing_item = to_do_factory(to_do_item.title.as_str(), status.clone());
    if &status.to_string()
        == &TaskStatus::from_string(to_do_item.status.as_str().to_string()).to_string()
    {
        return HttpResponse::Ok().json(ToDoItems::get_state());
    }

    process_input(existing_item, "edit".to_owned(), &state);
    return HttpResponse::Ok().json(ToDoItems::get_state());
}
