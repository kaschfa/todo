use serde::{Deserialize, Serialize};
use time::macros::format_description;
use time::{Date, PrimitiveDateTime, Time};

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct TodoDto {
    pub id: i64,
    pub title: String,
    pub note: Option<String>,
    pub due_date: Date,
    pub due_time: Time,
    pub created_at: PrimitiveDateTime,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct NewTodoDto {
    pub title: String,
    pub note: Option<String>,
    pub due_date: Date,
    pub due_time: Time,
}

impl NewTodoDto {
    pub fn new(title: String, due_date: Date, due_time: Time, note: Option<String>) -> Self {
        NewTodoDto {
            title: title,
            note: note,
            due_date: due_date,
            due_time: due_time,
        }
    }
}
