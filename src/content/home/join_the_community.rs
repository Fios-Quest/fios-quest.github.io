use crate::components::*;
use dioxus::prelude::*;

pub fn JoinTheCommunity() -> Element {
    rsx! {
        section {
            class: "join-the-community",
            H2 { title: "Join the community" }
            div {
                class: "logos",
                a {
                    class: "logo discord",
                    img { src: "logos/discord-logo-blue.svg", alt: "Discord Logo" }
                }
                a {
                    class: "logo patreon",
                    img { src: "logos/PATREON_SYMBOL_1_BLACK_RGB.svg", alt: "Patreon logo", class: "patreon-logo" }
                    img { src: "logos/PATREON_WORDMARK_1_BLACK_RGB.svg" , alt: "Patreon wordmark", class: "patreon-wordmark" }
                }
                a {
                    class: "logo youtube",
                    img { src: "logos/yt_logo_rgb_light.png", alt: "YouTube logo", class: "youtube-logo" }
                }
                a {
                    class: "logo github",
                    img { src: "logos/GitHub_Logo.png", alt: "GitHub logo", class: "github-logo" }
                }
            }
        }
    }
}
