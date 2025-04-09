mod join_the_community;
mod what_we_do;
mod who_we_are;

use dioxus::prelude::*;
use join_the_community::*;
use what_we_do::*;
use who_we_are::*;

const FAVICON: Asset = asset!("/assets/favicon.ico");
const SITE_CSS: Asset = asset!("/assets/site.css");
const RESET_CSS: Asset = asset!("/assets/reset.css");
// const SOCIAL_PREVIEW: Asset = asset!("/assets/social-preview.png");

#[component]
pub fn Home() -> Element {
    rsx! {
        document::Link { rel: "icon", href: FAVICON }
        document::Link { rel: "stylesheet", href: RESET_CSS }
        document::Link { rel: "stylesheet", href: SITE_CSS }

        // Link {
        //     to: Route::Blog {
        //         id: count()
        //     },
        //     "Go to blog"
        // }
        div { class: "title",
            img { src: asset!("assets/images/fio.svg"), alt: "Fio" }
            h1 { "Fio's Quest" }
            h2 { "Demystifying Software Engineering" }
        }

        WhatWeDo {}
        WhoWeAre {}
        JoinTheCommunity {}
    }
}
