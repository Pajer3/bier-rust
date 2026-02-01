use dioxus::prelude::*;

#[component]
pub fn ExploreScreen() -> Element {
    rsx! {
        div {
            class: "glass",
            style: "margin: 20px; padding: 20px;",
            h1 { "Ontdekken 🔍" }
            p { "Zoek naar bieren, brouwerijen en meer..." }
        }
    }
}
