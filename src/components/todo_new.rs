use crate::server::api;
use crate::shared::dto;
use dioxus::prelude::*;
use time::macros::format_description;
use time::{Date, PrimitiveDateTime, Time};

#[component]
pub fn Todo_Create() -> Element {
    let mut title = use_signal(|| "".to_string());
    let mut due_time = use_signal(|| "".to_string());
    let mut due_date = use_signal(|| "".to_string());
    let mut description = use_signal(|| "".to_string());
    // those need to be an enum
    let format_dt =
        format_description!("[year]-[month]-[day] [hour]:[minute]:[second].[subsecond]");
    let format_t = format_description!("[hour]:[minute]");
    let format_d = format_description!("[year]-[month]-[day]");
    rsx! {
        div {
            class: "flex flex-col gap-0.5",
            div {
                class: "rounded-xl border border-slate-500 px-1 py-2 flex justify-center gap-0.5",
                input {
                    class: "flex-1 rounded-xl border border-slate-500 p-1 text-center",
                    type: "text",
                    placeholder: "Titel",
                    value: "{title}",
                    oninput: move |e| title.set(e.value()),
                }
                input {
                    class: "rounded-xl border border-slate-500 p-1",
                    type: "time",
                    value: "{due_time}",
                    oninput: move |e| due_time.set(e.value()),
                }
                input {
                    class: "rounded-xl border border-slate-500 p-1",
                    type: "date",
                    value: "{due_date}",
                    oninput: move |e| due_date.set(e.value()),
                }
            }
            div {
                class: "relative flex-1 rounded-xl border border-slate-500 p-5 flex flex-col items-stretch",
                textarea {
                    class: "flex-1 outline-none focus:outline-none focus:ring-0 border-0",
                    placeholder: "Notes !",
                    value: "{description}",
                    oninput: move |e| description.set(e.value()),
                }
                button {
                    class: "absolute bottom-4 right-4 bg-slate-500 text-white py-2 px-4 rounded-2xl shadow-lg z-10 hover:bg-slate-600",
                    onclick: move |_| {
                        let title = title();
                        let due_time = Time::parse(&due_time(), &format_t).expect("invalid time");
                        let due_date = Date::parse(&due_date(), &format_d).expect("invalid date");
                        let note = description();
                        spawn(async move {
                            let _ = create_todo(title, due_time, due_date, Some(note)).await;
                        });
                    },
                    "Save"
                }
            }
        }
    }
}

async fn create_todo(
    title: String,
    due_time: Time,
    due_date: Date,
    note: Option<String>,
) -> Result<()> {
    let todo = dto::NewTodoDto::new(title, due_date, due_time, note);
    println!("Client create");
    api::todo::new_todo(todo).await;

    Ok(())
}
