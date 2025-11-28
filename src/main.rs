mod components;
mod server;

use crate::components::*;
use crate::server::*;
use dioxus::prelude::*;

#[derive(serde::Deserialize)]
struct RandomFact {
    id: String,
    text: String,
}

#[derive(Routable, Clone, PartialEq)]
pub enum Route {
    #[layout(Navbar)]
    #[layout(Sidebar)]
    #[route("/")]
    TodoView,
}

fn main() {
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    rsx! {
        document::Stylesheet{ href: asset!("/assets/tailwind.css")}
        Router::<Route> {}
    }
}
