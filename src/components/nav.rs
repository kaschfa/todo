use crate::Route;
use dioxus::prelude::*;

#[component]
pub fn Navbar() -> Element {
    rsx! {
       header {
            class: "flex shadow-lg items-center p-1 font-mono bg-slate-50 rounded-xl border border-slate-400 ",
            nav {
                class: "flex-1 flex justify-start items-center divide-x divide-gray-500",
                a {
                    class: "p-2 hover:shadow-md",
                    "Home"
                }
                a {
                    class: "p-2 hover:shadow-md",
                    "Blog"
                }
                a {
                    class: "p-2 hover:shadow-md",
                    "About"
                }
            }
            div {
                class: "flex justify-center",
                h1 {
                    class: "p-2",
                    "TO-DO"
                }
            }
            nav {
                class: "flex-1 flex justify-end items-center",
                a {
                    class: "p-2",
                    ""
                }
            }
        }
        Outlet::<Route> {}
    }
}
