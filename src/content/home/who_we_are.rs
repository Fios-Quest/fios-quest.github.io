use crate::components::*;
use dioxus::prelude::*;
use indoc::indoc;

pub fn WhoWeAre() -> Element {
    rsx! {
        section {
            H2 { title: "Who we are" }

            HorizontalCollapse {
                div {
                    H3 { title: "This is Fio" }
                    p {{ indoc! {
                        "She's on a quest to help people learn the complexities of software
                         engineering by demystifying the hard bits. Learning software engineering
                         can be challenging, but Fio believes in you!"
                    }}}
                }
                img { src: "images/fio.svg", alt: "Fio" }
            }

            HorizontalCollapse {
                img { src: "images/fio.svg", alt: "Daniel" }
                div {
                    H3 { title: "This is Daniel" }
                    p {{ indoc! {
                        "Since Fio is... well, a crab, Daniel is here to support her in her quest to
                         demystify software engineering. He writes the book and scripts, films and
                         edits the videos and provides technical support."
                    }}}
                }
            }
        }

    }
}
