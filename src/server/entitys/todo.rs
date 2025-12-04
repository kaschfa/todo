#![cfg(feature = "server")]
use crate::shared::dto::*;
use sea_orm::entity::prelude::*;
use sea_orm::IntoActiveModel;
use sea_orm::Set;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "todo")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i64,
    pub title: String,
    pub note: Option<String>,
    pub due_date: Date,
    pub due_time: Time,
    pub created_at: DateTime,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}

impl From<Model> for TodoDto {
    fn from(m: Model) -> Self {
        dbg!(&m);
        TodoDto {
            id: m.id,
            title: m.title,
            note: m.note,
            due_date: m.due_date.to_string(),
            due_time: m.due_time.to_string(),
            created_at: m.created_at.to_string(),
        }
    }
}

// Optional reverse transform if needed
impl IntoActiveModel<ActiveModel> for TodoDto {
    fn into_active_model(self) -> ActiveModel {
        ActiveModel {
            id: Set(self.id),
            title: Set(self.title),
            note: Set(self.note),
            due_date: Set(self.due_date.parse().expect("invalid date")),
            due_time: Set(self.due_time.parse().expect("invalid time")),
            created_at: Set(self.created_at.parse().expect("invalid datetime")),
        }
    }
}

impl NewTodoDto {
    pub fn into_active_model(self) -> ActiveModel {
        ActiveModel {
            title: Set(self.title),
            note: Set(self.note),
            due_date: Set(self.due_date.parse().expect("invalid date")),
            due_time: Set(self.due_time.parse().expect("invalid time")),
            created_at: Default::default(), // Let DB handle it
            id: Default::default(),         // Auto-increment
        }
    }
}
