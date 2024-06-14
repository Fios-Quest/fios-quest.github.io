#![allow(non_snake_case)]

mod components;
mod content;

use content::*;
use dioxus::prelude::*;
use tracing::Level;

#[derive(Clone, Routable, Debug, PartialEq)]
enum Route {
    #[route("/")]
    Home {},
    // #[route("/blog/:id")]
    // Blog { id: i32 },
    // #[route("/shop")]
    // Shop {},
}

fn main() {
    // Init logger
    if cfg!(debug_assertions) {
        dioxus_logger::init(Level::INFO).expect("failed to init logger");
    }
    launch(App);
}

fn App() -> Element {
    rsx! {
        Router::<Route> {}
    }
}

// #[component]
// fn Blog(id: i32) -> Element {
//     rsx! {
//         Link { to: Route::Home {}, "Go to counter" }
//         "Blog post {id}"
//     }
// }
