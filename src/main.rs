mod components;
mod server;

use crate::components::*;
use crate::server::*;
use dioxus::prelude::*;
use uuid::Uuid;

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
    Todo_Overview,
    #[route("/todo/new")]
    Create_Todo,
    #[route("/todo/edit/:id")]
    Todo_Edit { id: Uuid },
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
