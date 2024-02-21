use crate::{state::read_file, to_do::data::FILE_NAME};
use actix_web::{web, Responder};

pub async fn get() -> impl Responder {
    let state = read_file(FILE_NAME);
    web::Json(state)
}
