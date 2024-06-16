use std::fmt::{Display, Formatter};
use dioxus::prelude::*;

#[derive(PartialOrd, PartialEq, Copy, Clone)]
pub enum SpeechBubbleType {
    Left,
    Right,
}

impl Display for SpeechBubbleType {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            SpeechBubbleType::Left => write!(f, "left"),
            SpeechBubbleType::Right => write!(f, "right"),

        }
    }
}

#[component]
pub fn SpeechBubble(bubble_type: SpeechBubbleType, children: Element) -> Element {
    rsx! {
        div { class: "speech-bubble {bubble_type}", {children} }
    }
}
