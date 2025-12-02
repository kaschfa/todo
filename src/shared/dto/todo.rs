use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct TodoDto {
    pub id: i64,
    pub title: String,
    pub note: Option<String>,
    pub due_date: String,
    pub due_time: String,
    pub created_at: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct NewTodoDto {
    pub title: String,
    pub note: Option<String>,
    pub due_date: String,
    pub due_time: String,
}

impl NewTodoDto {
    pub fn new(title: String, due_date: String, due_time: String, note: Option<String>) -> Self {
        NewTodoDto {
            title: title,
            note: note,
            due_date: due_date,
            due_time: due_time,
        }
    }
}
