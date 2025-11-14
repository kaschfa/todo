use dioxus::prelude::*;

const FAVICON: Asset = asset!("/assets/favicon.ico");
const CSS: Asset = asset!("/assets/main.css");
const HEADER_SVG: Asset = asset!("/assets/header.svg");
const TAILWIND_CSS: Asset = asset!("/assets/tailwind.css");

fn main() {
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    rsx! {
        document::Stylesheet { href: CSS }
        div { id: "container",
            div { id: "sidebar",

            }
            div { id: "note",
                div { id: "head",
                    h1 { "TitleBar" }
                }
                div { id: "todo",
                    h1 { "Body" }
                }
            }
        }
    }
}
