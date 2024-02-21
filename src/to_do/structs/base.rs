use crate::to_do::enums::TaskStatus;
use serde::Serialize;

#[derive(Serialize)]
pub struct Base {
    pub title: String,
    pub status: TaskStatus,
}
