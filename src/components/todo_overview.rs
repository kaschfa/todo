use crate::Route;
use crate::{components::todo_edit, server::todo::*};
use chrono::{DateTime, NaiveDate, NaiveDateTime, NaiveTime};
use dioxus::prelude::*;

#[component]
pub fn Todo_Overview() -> Element {
    let todos = use_resource(get_todos);
    rsx! {
        div { /* grid rows might need changing */
            class: "grid grid-cols-5 grid-rows-3 rounded-xl border border-slate-500 p-5 gap-2",
            match &*todos.read() {
                Some(Ok(list)) => rsx! {
                    {list.iter().map( |todo| {
                        rsx! {
                            div {
                                class: "rounded-xl border border-slate-500 p-2 text-center shadow-lg hover:shadow-2xl",

                                p {"{todo.title}"}
                                p { "{todo.due_time.unwrap_or_default()}"}
                                p {"{todo.due_date.unwrap_or_default()}"}
                                div { "{todo.description.clone().unwrap_or_default()}"}
                            }
                        }
                    })}
                },
                Some(Err(e)) => rsx!{ p { "Error: {e}" } },
                None => rsx!{ p { "Loading..." } },
            }
        }
    }
}

#[component]
pub fn todo_card(todo: Todo) -> Element {
    let nav = use_navigator();

    rsx! {
        div {
            class: "rounded-xl border border-slate-500 p-2 text-center shadow-lg hover:shadow-2xl",
            onclick: move |_| {
                nav.push(Route::Todo_Edit { id: todo.id.expect("ID missing") });
                nav.go_forward();
            },
            p {"{todo.title}"}
            p { "{todo.due_time.unwrap_or_default()}"}
            p {"{todo.due_date.unwrap_or_default()}"}
            div { "{todo.description.clone().unwrap_or_default()}"}
        }
    }
}
