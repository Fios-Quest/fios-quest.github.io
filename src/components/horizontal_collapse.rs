use dioxus::prelude::*;

#[component]
pub fn HorizontalCollapse(children: Element) -> Element {
    rsx! {
        div { class: "horizontal-collapse", {children} }
    }
}
