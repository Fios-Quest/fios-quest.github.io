use crate::components::*;
use dioxus::prelude::*;
use indoc::indoc;

pub fn WhoWeAre() -> Element {
    rsx! {
        section { class: "who-we-are",

            H2 { title: "Who we are" }

            HorizontalCollapse { 
                SpeechBubble { bubble_type: SpeechBubbleType::Left,
                    H3 { title: "Hi, I'm Fio" }
                    p {
                        { indoc! {
                            "I'm on a quest to help people learn the complexities of software
                            engineering by demystifying the hard bits. Learning software engineering
                            can be challenging, but I believe in you!"
                        }}
                    }
                }
                img { src: "images/fio-round.svg", alt: "Fio" }
            }

            HorizontalCollapse { 
                img {
                    src: "images/daniel.png",
                    alt: "Daniel",
                    class: "circle"
                }
                SpeechBubble { bubble_type: SpeechBubbleType::Right,
                    H3 { title: "Hello, I'm Daniel" }
                    p {
                        { indoc! {
                            "Since Fio is... well, a crab, I'm here to support her in her quest to
                            demystify software engineering. I write the book and scripts, film and
                            edits the videos and provide technical support."
                        }}
                    }
                }
            }
        }
    }
}
