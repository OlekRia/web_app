use serde::ser::{Serialize, Serializer};
use std::fmt;

#[derive(Clone)]
pub enum TaskStatus {
    DONE,
    PENDING,
}

impl Serialize for TaskStatus {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        Ok(serializer.serialize_str(&self.to_string().as_str())?)
    }
}

impl fmt::Display for TaskStatus {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match &self {
            &Self::DONE => write!(f, "DONE"),
            &Self::PENDING => write!(f, "PENDING"),
        }
    }
}

impl TaskStatus {
    pub fn from_string(input_string: String) -> Self {
        match input_string.as_str() {
            "DONE" => TaskStatus::DONE,
            "PENDING" => TaskStatus::PENDING,
            _ => panic!("input {} not supported", input_string),
        }
    }
}
