mod backend;
mod components;

use crate::components::*;
use dioxus::prelude::*;

#[derive(serde::Deserialize)]
struct RandomFact {
    id: String,
    text: String,
}

#[derive(Routable, Clone, PartialEq)]
enum Route {
    #[layout(Navbar)]
    #[route("/")]
    TodoView,
}

fn main() {
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    let mut sidebar_open = use_signal(|| false);

    rsx! {
        document::Stylesheet{ href: asset!("/assets/tailwind.css")}
        Router::<Route> {}
    }
}
