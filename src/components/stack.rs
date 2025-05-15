use dioxus::prelude::*;
#[component]
pub fn Stack(children: Element) -> Element {
    rsx! {
        div { class: "stack", {children} }
    }
}
