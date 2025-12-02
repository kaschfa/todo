use crate::shared::dto::*;
use dioxus::prelude::*;

#[server(GetAllTodos, "/api/todo")]
pub async fn get_all_todos() -> Result<Vec<TodoDto>, ServerFnError> {
    use crate::server::database;
    use crate::server::entitys::todo;
    use sea_orm::{ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter};

    let db = database::connect()
        .await
        .map_err(|e| ServerFnError::new(e.to_string()))?;
    let models = todo::Entity::find()
        .all(&db)
        .await
        .expect("todo load error");
    let dtos: Vec<TodoDto> = models.into_iter().map(|m| TodoDto::from(m)).collect();
    Ok(dtos)
}

#[server(NewTodo, "/api/todo")]
pub async fn new_todo(todo: NewTodoDto) -> Result<(), ServerFnError> {
    use crate::server::database;
    use crate::server::entitys::todo;
    use sea_orm::ActiveModelTrait;

    let db = database::connect()
        .await
        .map_err(|e| ServerFnError::new(e.to_string()))?;
    let act_model: todo::ActiveModel = todo.into_active_model();
    println!("Server create");
    act_model
        .insert(&db)
        .await
        .map_err(|e| ServerFnError::new(e.to_string()))?;

    Ok(())
}
