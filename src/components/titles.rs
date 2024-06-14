use dioxus::prelude::*;

#[component]
pub fn ToSpans(title: String) -> Element {
    rsx! {
        for word in title.split(' ') {
            " " span { "{word}" } " "
        }
    }
}

#[component]
pub fn H1(title: String) -> Element {
    rsx! {
        h1 {
            ToSpans { title }
        }
    }
}

#[component]
pub fn H2(title: String) -> Element {
    rsx! {
        h2 {
            ToSpans { title }
        }
    }
}

#[component]
pub fn H3(title: String) -> Element {
    rsx! {
        h3 {
            ToSpans { title }
        }
    }
}
