mod components;
mod server;
mod shared;

use crate::components::*;
use dioxus::prelude::*;

#[derive(Routable, Clone, PartialEq)]
pub enum Route {
    #[layout(Navbar)]
    #[layout(Sidebar)]
    #[route("/")]
    Todo_Overview,
    #[route("/todo/new")]
    Todo_Create,
    #[route("/todo/edit:id")]
    Todo_Edit { id: i64 },
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
