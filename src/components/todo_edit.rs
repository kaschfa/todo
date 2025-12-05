use crate::server::api;
use crate::shared::dto::{self, TodoDto};
use crate::Route;
use dioxus::prelude::*;
use time::macros::format_description;
use time::{Date, PrimitiveDateTime, Time};

#[component]
pub fn Todo_Edit(id: i64) -> Element {
    let todo = use_resource(move || async move { api::todo::get_todo_by_id(id).await });
    // those need to be an enum
    let format_dt =
        format_description!("[year]-[month]-[day] [hour]:[minute]:[second].[subsecond]");
    let format_t = format_description!("[hour]:[minute]");
    let format_d = format_description!("[year]-[month]-[day]");

    match todo.value().read().clone() {
        Some(Ok(mut t)) => {
            let mut t_s = use_signal(|| t.clone());
            rsx! {
                div {
                    class: "flex flex-col gap-0.5",
                    div {
                        class: "rounded-xl border border-slate-500 px-1 py-2 flex justify-center gap-0.5",
                        input {
                            class: "flex-1 rounded-xl border border-slate-500 p-1 text-center",
                            type: "text",
                            value: "{t.title}",
                            oninput: move |e| t_s.write().title = e.value(),
                        }
                        input {
                            class: "rounded-xl border border-slate-500 p-1",
                            type: "time",
                            value: "{t.due_time}",
                            oninput: move |e| t_s.write().due_time = Time::parse(&e.value(), &format_t).expect("invalid time"),
                        }
                        input {
                            class: "rounded-xl border border-slate-500 p-1",
                            type: "date",
                            value: "{t.due_date}",
                            oninput: move |e| t_s.write().due_date = Date::parse(&e.value(), &format_d).expect("invalid date"),
                        }
                    }
                    div {
                        class: "relative flex-1 rounded-xl border border-slate-500 p-5 flex flex-col items-stretch",
                        textarea {
                            class: "flex-1 outline-none focus:outline-none focus:ring-0 border-0",
                            placeholder: "Notes !",
                            value: "{t.note.clone().unwrap_or_default()}",
                            oninput: move |e| t_s.write().note = Some(e.value()),
                        }
                        button {
                            class: "absolute bottom-4 right-4 bg-slate-500 text-white py-2 px-4 rounded-2xl shadow-lg z-10 hover:bg-slate-600",
                            onclick: move |_| {
                                let value = t_s().clone();
                                spawn(async move {
                                    let _ = api::todo::edit_todo(value).await;
                                }
                            );
                        },
                        "Save"
                    }
                }
            }}
        }
        Some(Err(e)) => rsx! { p { "Error: {e}" } },
        None => rsx! { p { "Loading..." } },
    }
}
