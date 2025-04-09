use crate::components::*;
use dioxus::prelude::*;

pub fn JoinTheCommunity() -> Element {
    rsx! {
        section { class: "join-the-community",
            H2 { title: "Join the community" }
            div { class: "logos",
                a {
                    class: "logo discord",
                    href: "https://discord.gg/MDHcrNf4k8",
                    img {
                        src: asset!("assets/logos/discord-logo-blue.svg"),
                        alt: "Discord Logo"
                    }
                }
                a {
                    class: "logo patreon",
                    href: "https://patreon.com/fios_quest",
                    img {
                        src: asset!("assets/logos/PATREON_SYMBOL_1_BLACK_RGB.svg"),
                        alt: "Patreon logo",
                        class: "patreon-logo"
                    }
                    img {
                        src: asset!("assets/logos/PATREON_WORDMARK_1_BLACK_RGB.svg"),
                        alt: "Patreon wordmark",
                        class: "patreon-wordmark"
                    }
                }
                a {
                    class: "logo youtube",
                    href: "https://www.youtube.com/@FiosQuest",
                    img {
                        src: asset!("assets/logos/yt_logo_rgb_light.png"),
                        alt: "YouTube logo",
                        class: "youtube-logo"
                    }
                }
                a {
                    class: "logo github",
                    href: "https://github.com/Fios-Quest/",
                    img {
                        src: asset!("assets/logos/GitHub_Logo.png"),
                        alt: "GitHub logo",
                        class: "github-logo"
                    }
                }
            }
        }
    }
}
