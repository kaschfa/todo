use crate::server::dbpool::*;
use chrono::{DateTime, NaiveDate, NaiveDateTime, NaiveTime};
use dioxus::prelude::*;
#[cfg(feature = "server")]
use sqlx::Row;
use uuid::Uuid;

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "server", derive(sqlx::FromRow))]
pub struct Todo {
    pub id: Option<Uuid>,            // DB generates if None
    pub title: String,               // Required field
    pub due_time: Option<NaiveTime>, // DB default if None
    pub due_date: Option<NaiveDate>, // DB default if None
    pub description: Option<String>,
    pub created: Option<NaiveDateTime>, // DB generates if None
}

impl Todo {
    pub fn new(
        title: String,
        due_time: Option<NaiveTime>,
        due_date: Option<NaiveDate>,
        description: Option<String>,
    ) -> Todo {
        Todo {
            title: title,
            due_time: due_time,
            due_date: due_date,
            description: description,
            id: None,
            created: None,
        }
    }
}

#[server]
pub async fn get_todos() -> Result<Vec<Todo>, ServerFnError> {
    let pool = get_pool().await;

    let todos = sqlx::query_as::<_, Todo>(
        r#"
        SELECT id, title, due_time, due_date, description, created
        FROM todo
        ORDER BY created DESC
        "#,
    )
    .fetch_all(pool)
    .await
    .map_err(|e| ServerFnError::new(format!("DB query error: {e}")))?;

    Ok(todos)
}

#[server]
pub async fn get_todo(id: Uuid) -> Result<Todo, ServerFnError> {
    let pool = get_pool().await;

    let todo = sqlx::query_as::<_, Todo>(
        r#"
        SELECT id, title, due_time, due_date, description, created
        FROM todo
        WHERE id = $1
        "#,
    )
    .bind(id)
    .fetch_one(pool)
    .await
    .map_err(|e| ServerFnError::new(format!("DB query error: {e}")))?;

    Ok(todo)
}

#[server]
pub async fn save_todo(todo: Todo) -> Result<(), ServerFnError> {
    let pool = get_pool().await;

    sqlx::query(
        r#"
            INSERT INTO todo (title, due_time, due_date, description) 
            VALUES ($1, $2, $3, $4)
            "#,
    )
    .bind(&todo.title)
    .bind(todo.due_time)
    .bind(todo.due_date)
    .bind(todo.description)
    .execute(pool)
    .await
    .map_err(|e| ServerFnError::new(format!("DB insert error: {e}")))?;

    Ok(())
}
