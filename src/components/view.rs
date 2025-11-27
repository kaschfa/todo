use dioxus::prelude::*;

#[component]
pub fn TodoView() -> Element {
    rsx! {
        div {
            class: "flex-1 min-h-0 grid grid-cols-[auto_1fr] gap-1 bg-slate-50",
            div {
                class: "p-5 rounded-xl border border-slate-500",
                p {"Sidebar Menu"}
            }
            div {
                class: "flex flex-col gap-0.5",
                div {
                    class: "rounded-xl border border-slate-500 px-1 py-2 flex justify-center gap-0.5",
                    input {
                        class: "flex-1 rounded-xl border border-slate-500 p-1 text-center",
                        type: "text",
                        placeholder: "Titel",
                    }
                    input {
                        class: "rounded-xl border border-slate-500 p-1",
                        type: "time",
                    }
                    input {
                        class: "rounded-xl border border-slate-500 p-1",
                        type: "date",
                    }
                }
                div {
                    class: "flex-1 rounded-xl border border-slate-500 p-5 flex flex-col items-stretch",
                    textarea {
                        class: "flex-1 outline-none focus:outline-none focus:ring-0 border-0 caret-slate-500",
                        placeholder: "Notes !"
                    }
                }
            }
        }
    }
}
