use dioxus::prelude::*;

#[component]
pub fn ClubsScreen() -> Element {
    rsx! {
        div {
            class: "glass",
            style: "margin: 20px; padding: 20px;",
            h1 { "Clubs 🛡️" }
            p { "Jouw clubs en chats..." }
        }
    }
}
