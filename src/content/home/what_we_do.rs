use crate::components::*;
use dioxus::prelude::*;

pub fn WhatWeDo() -> Element {
    rsx! {
        section {
            class: "what-we-do",
            H2 { title: "What we do" }
            p { "Programming can be hard, that doesn't mean you can't do it." }
            p { "We provide free and paid training covering Rust, TypeScript, JavaScript and general programming practices." }

            HorizontalCollapse {
                a {
                    href: "https://www.youtube.com/playlist?list=PLW2L8KbM0O7aRi_Bt4YE1JuW9EdMs0ztR",
                    img {
                        src: "./images/iriss-youtube.jpeg"
                    }
                    figcaption { "YouTube" }
                }
                a {
                    href: "/idiomatic-rust-in-simple-steps",
                    img {
                        src: "./images/iriss-book.png"
                    }
                    figcaption {
                        "Book"
                    }
                }
            }
        }
    }
}
