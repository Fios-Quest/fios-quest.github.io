use crate::components::*;
use dioxus::prelude::*;

pub fn JoinTheCommunity() -> Element {
    rsx! {
        section {
            class: "join-the-community",
            H2 { title: "Join the community" }
            div {
                class: "logos",
                div {
                    class: "discord",
                    img { src: "logos/discord-logo-blue.svg", alt: "Discord Logo" }
                }
                div {
                    class: "patreon",
                    img { src: "logos/PATREON_SYMBOL_1_BLACK_RGB.svg", alt: "Patreon logo", class: "patreon-logo" }
                    img { src: "logos/PATREON_WORDMARK_1_BLACK_RGB.svg" , alt: "Patreon wordmark", class: "patreon-wordmark" }
                }
            }
        }
    }
}
