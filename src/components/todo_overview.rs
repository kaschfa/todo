use crate::server::todo::*;
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
                    for todo in list {
                        div {
                            class: "rounded-xl border border-slate-500 p-2 text-center",
                            "{todo.title}"
                        }
                    }
                },
                Some(Err(e)) => rsx!{ p { "Error: {e}" } },
                None => rsx!{ p { "Loading..." } },
            }
        }
    }
}
