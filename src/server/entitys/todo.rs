#![cfg(feature = "server")]
use crate::shared::dto::*;
use sea_orm::entity::prelude::*;
use sea_orm::IntoActiveModel;
use sea_orm::Set;
use time::macros::format_description;
use time::{Date, PrimitiveDateTime, Time};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "todo")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i64,
    pub title: String,
    pub note: Option<String>,
    pub due_date: Date,
    pub due_time: Time,
    pub created_at: PrimitiveDateTime,
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
        let format_dt =
            format_description!("[year]-[month]-[day] [hour]:[minute]:[second].[subsecond]");
        let format_t = format_description!("[hour]:[minute]");
        let format_d = format_description!("[year]-[month]-[day]");
        dbg!(&self.due_time);
        ActiveModel {
            id: Set(self.id),
            title: Set(self.title),
            note: Set(self.note),
            due_date: Set(Date::parse(&self.due_date.trim(), &format_d).expect("invalid date")),
            due_time: Set(Time::parse(&self.due_time.trim(), &format_t).expect("invalid time")),
            created_at: Set(
                PrimitiveDateTime::parse(&self.created_at.trim(), &format_dt)
                    .expect("invalid datetime"),
            ),
        }
    }
}

impl NewTodoDto {
    pub fn into_active_model(self) -> ActiveModel {
        let format_t = format_description!("[hour]:[minute]");
        let format_d = format_description!("[year]-[month]-[day]");
        dbg!(&self.due_time);
        ActiveModel {
            title: Set(self.title),
            note: Set(self.note),
            due_date: Set(Date::parse(&self.due_date.trim(), &format_d).expect("invalid date")),
            due_time: Set(Time::parse(&self.due_time.trim(), &format_t).expect("invalid time")),
            created_at: Default::default(), // Let DB handle it
            id: Default::default(),         // Auto-increment
        }
    }
}
