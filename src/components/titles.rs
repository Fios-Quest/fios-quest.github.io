use dioxus::prelude::*;
fn to_id(input: &str) -> String {
    input
        .chars()
        .filter_map(|c| match c {
            c if c.is_alphanumeric() && c.is_ascii() => Some(c.to_ascii_lowercase()),
            ' ' => Some('-'),
            _ => None,
        })
        .collect()
}
#[component]
pub fn ToSpans(title: String) -> Element {
    rsx! {
        for word in title.split(' ') {
            " "
            span { "{word}" }
            " "
        }
    }
}
#[component]
pub fn H1(title: String) -> Element {
    let id = to_id(&title);
    rsx! {
        h1 { id,
            ToSpans { title }
        }
    }
}
#[component]
pub fn H2(title: String) -> Element {
    let id = to_id(&title);
    rsx! {
        h2 { id,
            ToSpans { title }
        }
    }
}
#[component]
pub fn H3(title: String) -> Element {
    let id = to_id(&title);
    rsx! {
        h3 { id,
            ToSpans { title }
        }
    }
}
