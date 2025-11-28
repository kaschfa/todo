use crate::server::todo::*;
use chrono::{DateTime, NaiveDate, NaiveDateTime, NaiveTime};
use dioxus::prelude::*;

#[component]
pub fn TodoView() -> Element {
    let mut title = use_signal(|| "".to_string());
    let mut due_time = use_signal(|| "".to_string());
    let mut due_date = use_signal(|| "".to_string());
    let mut description = use_signal(|| "".to_string());
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
                        let due_time = due_time();
                        let due_date = due_date();
                        let description = description();
                        spawn(async move {
                            let _ = write_todo(title, due_time, due_date, description).await;
                        });
                    },
                    "Save"
                }
            }
        }
    }
}

async fn write_todo(
    title: String,
    due_time_str: String,
    due_date_str: String,
    description: String,
) -> Result<()> {
    let todo: Todo = Todo::new(
        title,
        NaiveTime::parse_from_str(&due_time_str, "%H:%M").ok(),
        NaiveDate::parse_from_str(&due_date_str, "%Y-%m-%d").ok(),
        if description.is_empty() {
            None
        } else {
            Some(description)
        },
    );
    save_todo(todo).await?;
    Ok(())
}
