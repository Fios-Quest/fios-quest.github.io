mod join_the_community;
mod what_we_do;
mod who_we_are;

use dioxus::prelude::*;
use join_the_community::*;
use what_we_do::*;
use who_we_are::*;

#[component]
pub fn Home() -> Element {
    rsx! {
        // Link {
        //     to: Route::Blog {
        //         id: count()
        //     },
        //     "Go to blog"
        // }
        div { class: "title",
            img { src: "images/fio.svg", alt: "Fio" }
            h1 { "Fio's Quest" }
            h2 { "Demystifying Software Engineering" }
        }

        WhatWeDo {}
        WhoWeAre {}
        JoinTheCommunity {}
    }
}
