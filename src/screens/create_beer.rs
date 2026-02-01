use dioxus::prelude::*;

#[component]
pub fn CreateBeerScreen() -> Element {
    rsx! {
        div {
            class: "glass",
            style: "margin: 20px; padding: 20px;",
            h1 { "Nieuw Bier 🍺" }
            p { "Formulier om bier toe te voegen..." }
        }
    }
}
