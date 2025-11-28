use crate::Route;
use dioxus::prelude::*;

#[component]
pub fn Sidebar() -> Element {
    rsx! {
        div {
            class: "flex-1 min-h-0 grid grid-cols-[auto_1fr] gap-1 bg-slate-50",
            div {
                class: "p-4 rounded-xl border border-slate-500 divide-y divide-slate-500",
                p {class:"p-1", "Sidebar Menu"}
                p {class:"p-1", "Sidebar Menu"}
            }
            Outlet::<Route> {}
        }
    }
}
