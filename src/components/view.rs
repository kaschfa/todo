use dioxus::prelude::*;

#[component]
pub fn TodoView() -> Element {
    rsx! {
        div {
            class: "flex-1 min-h-0 grid grid-cols-[auto_1fr] gap-1",
            div {
                class: "p-5 rounded-xl border border-slate-500",
                p {"Sidebar Menu"}
            }
            div {
                class: "rounded-xl border border-slate-500",
                h3 {"Main Part"}
            }
        }
    }
}
