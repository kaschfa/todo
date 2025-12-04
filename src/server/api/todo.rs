use crate::shared::dto::*;
use dioxus::prelude::*;

#[server]
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

#[server]
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

#[server]
pub async fn get_todo_by_id(id: i64) -> Result<TodoDto, ServerFnError> {
    use crate::server::database;
    use crate::server::entitys::todo;
    use sea_orm::EntityTrait;

    println!("{}", id);

    let db = database::connect()
        .await
        .map_err(|e| ServerFnError::new(e.to_string()))?;

    if let Ok(Some(model)) = todo::Entity::find_by_id(id).one(&db).await {
        Ok(TodoDto::from(model))
    } else {
        Err(ServerFnError::Response(
            format!("Failed to load todo with id: {id}").into(),
        ))
    }
}

#[server]
pub async fn edit_todo(todo: TodoDto) -> Result<(), ServerFnError> {
    use crate::server::database;
    use crate::server::entitys::todo;
    use sea_orm::ActiveModelTrait;
    use sea_orm::IntoActiveModel;
    //needs redo  without loading from db again prob
    let db = database::connect()
        .await
        .map_err(|e| ServerFnError::new(e.to_string()))?;

    let todo: todo::ActiveModel = todo.into_active_model();

    if let Ok(model) = todo.update(&db).await {
        Ok(())
    } else {
        panic!();
        Err(ServerFnError::Response("Saving todo failed".to_string()))
    }
}
