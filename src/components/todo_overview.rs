use crate::server::api;
use crate::shared::dto::{self, TodoDto};
use crate::Route;
use dioxus::prelude::*;

#[component]
pub fn Todo_Overview() -> Element {
    let todos = use_resource(move || async move { api::todo::get_all_todos().await });

    let todos = todos.read();

    match todos.as_ref() {
        Some(Ok(t)) => rsx! {
            div { /* grid rows might need changing */
                class: "grid grid-cols-5 grid-rows-3 rounded-xl border border-slate-500 p-5 gap-2",
                for todo in t {
                    todo_card { todo: todo.clone() }
                }
            }
        },
        Some(Err(e)) => rsx! { p { "Error: {e}" } },
        None => rsx! { p { "Loading..." } },
    }
}

#[component]
pub fn todo_card(todo: TodoDto) -> Element {
    let nav = use_navigator();

    rsx! {
        div {
            class: "rounded-xl border border-slate-500 p-2 text-center shadow-lg hover:shadow-2xl",
            onclick: move |_| {
                nav.push(Route::Todo_Edit { id: todo.id });
            },
            p {"{todo.id}"}
            p {"{todo.title}"}
            p { "{todo.due_time}"}
            p {"{todo.due_date}"}
            p {"{todo.created_at}"}
            div { "{todo.note.clone().unwrap_or_default()}"}
        }
    }
}
