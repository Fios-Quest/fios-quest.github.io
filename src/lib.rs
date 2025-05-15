#![allow(non_snake_case)]
mod components;
mod content;
use content::*;
use dioxus::prelude::*;
#[derive(Clone, Routable, Debug, PartialEq)]
pub enum Route {
    #[route("/")]
    Home {},
}
pub fn App() -> Element {
    rsx! {
        Router::<Route> {}
    }
}
