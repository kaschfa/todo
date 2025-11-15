use dioxus::prelude::*;

const CSS: Asset = asset!("/assets/main.css");

fn main() {
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    rsx! {
        document::Stylesheet { href: CSS }
        Navbar{}
        Content{}
    }
}

#[component]
fn Navbar() -> Element {
    rsx! {
        div { id: "navbar",
            h1 {"Navbar"}
        }
    }
}

#[component]
fn Content() -> Element {
    rsx! {
        div { id: "layout",
            div { id: "sidebar",
                h1 { "Sidebar" }
            }
            div { id: "container",
                div { id: "head",
                    h1 { "TitleBar" }
                },
                div { id: "todo",
                    textarea { id: "todo-text",
                    "Type here"
                    }
                }
            }
        }
    }
}
