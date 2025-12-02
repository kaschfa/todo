use dioxus::prelude::*;

#[component]
pub fn Todo_Edit(id: String) -> Element {
    rsx! {
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
                class: "relative flex-1 rounded-xl border border-slate-500 p-5 flex flex-col items-stretch",
                textarea {
                    class: "flex-1 outline-none focus:outline-none focus:ring-0 border-0",
                    placeholder: "Notes !",
                }
                button {
                    class: "absolute bottom-4 right-4 bg-slate-500 text-white py-2 px-4 rounded-2xl shadow-lg z-10 hover:bg-slate-600",
                    "Save"
                }
            }
        }
    }
}
