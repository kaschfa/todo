use dioxus::prelude::*;

const CSS: Asset = asset!("/assets/main.css");

fn main() {
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    let mut sidebar_open = use_signal(|| false);

    rsx! {
        document::Stylesheet { href: CSS }
        Navbar{
            menu_click: move || sidebar_open.set(!sidebar_open()),
        }
        Content{
            sidebar_open: sidebar_open(),
        }
    }
}

#[component]
fn Navbar(menu_click: Callback<()>) -> Element {
    rsx! {
        div {
            class: "navbar",
            div {
                class: "navbar-menu",
                button {
                    class: "navbar-menu-button",
                    onclick: move |_| menu_click.call(()),
                    "󰍜"
                }
            }
            h1 {"Navbar"}
        }
    }
}

#[component]
fn Content(sidebar_open: bool) -> Element {
    let sidebar_elements: Vec<_> = (0..=5)
        .map(|i| {
            rsx! {
                div {
                    class: "sidebar-element",
                    "Item {i}"
                }
            }
        })
        .collect();

    rsx! {
        div {
            class: "layout",
            if sidebar_open {
                div {
                    class: "sidebar",
                    {sidebar_elements.into_iter()}
                }
            }
            Todo {  }
        }
    }
}

#[component]
fn Todo() -> Element {
    rsx! {
        div {
            class: "todo-container",
            Todo_Head {  }
            Todo_Body {  }
        }
    }
}

#[component]
fn Todo_Head() -> Element {
    rsx! {
        div {
            class: "todo-head",
            div {
                class: "todo-title",
                input {
                class: "todo-title-text",
                placeholder: "Title here"
            }
            }
            div {
                class: "todo-options",
                button {
                    class: "todo-edit",
                    ""
                }
            }
        }
    }
}

#[component]
fn Todo_Body() -> Element {
    rsx! {
        div {
            class: "todo-body",
            textarea {
                class: "todo-text",
                placeholder: "Description here"
            }
        }
    }
}
