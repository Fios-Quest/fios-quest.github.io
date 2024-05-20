#![allow(non_snake_case)]

use dioxus::prelude::*;
use tracing::Level;

#[derive(Clone, Routable, Debug, PartialEq)]
enum Route {
    #[route("/")]
    Home {},
    // #[route("/blog/:id")]
    // Blog { id: i32 },
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

#[component]
fn Home() -> Element {
    rsx! {
        // Link {
        //     to: Route::Blog {
        //         id: count()
        //     },
        //     "Go to blog"
        // }
        div {
            class: "center",
            img {
                src: "images/fio.svg",
                alt: "Fio"
            }
            h1 { "Coming soon..."}
        }
    }
}
