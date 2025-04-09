use crate::components::*;
use dioxus::prelude::*;
use indoc::indoc;

pub fn WhatWeDo() -> Element {
    rsx! {
        section { class: "what-we-do",
            H2 { title: "What we do" }
            p { "Programming can be hard, that doesn't mean you can't do it." }
            p {
                "We provide free and paid training covering Rust, TypeScript, JavaScript and general programming practices."
            }

            H3 { title: "Idiomatic Rust in Simple Steps" }

            p {
                { indoc! { "
                    Idiomatic Rust in Simple Steps (IRISS) is an online book and YouTube series designed
                    to help people learn how to program Rust, with a focus on learning the idioms of the
                    language immediately, with explanations as to why we follow these patterns.
                " }}
            }

            HorizontalCollapse { 
                div {
                    a { href: "https://www.youtube.com/playlist?list=PLW2L8KbM0O7aRi_Bt4YE1JuW9EdMs0ztR",
                        Stack { 
                            img {
                                src: asset!("assets/images/iriss-youtube.jpeg"),
                                alt: "IRISS YouTube Playlist"
                            }
                            img {
                                class: "play-icon",
                                src: asset!("assets/logos/yt_icon_rgb.png")
                            }
                        }
                    }
                }
                div {
                    a { href: "/idiomatic-rust-in-simple-steps", Stack { 
                        img { src: asset!("assets/images/iriss-book.png"), alt: "IRISS Book" }
                        div { class: "label", "Online Book" }
                    } }
                }
            }
        }
    }
}
