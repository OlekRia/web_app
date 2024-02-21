use crate::{
    processes::process_input,
    state::read_file,
    to_do::{data::FILE_NAME, enums::TaskStatus, to_do_factory},
};
use actix_web::HttpRequest;

pub async fn create(req: HttpRequest) -> String {
    let state = read_file(FILE_NAME);
    let title = req.match_info().get("title").unwrap();
    let item = to_do_factory(&title, TaskStatus::PENDING);
    process_input(item, "create".to_string(), &state);

    format!("{} created", title)
}
