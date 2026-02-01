use dioxus::prelude::*;
use crate::components::navbar::{Navbar, ActiveTab};
use crate::screens::{HomeScreen, CreateBeerScreen, ClubsScreen, ProfileScreen, ExploreScreen};

#[component]
pub fn Dashboard() -> Element {
    let mut active_tab = use_signal(|| ActiveTab::Home);

    rsx! {
        div {
            class: "dashboard-container",
            style: "height: 100vh; width: 100vw; overflow: hidden; background-color: var(--background); color: var(--text);",
            
            // Main Content Area - Scrollable
            div {
                class: "content-area",
                style: "height: 100%; width: 100%; overflow-y: auto; padding-bottom: 80px;", // padding for navbar
                
                match active_tab() {
                    ActiveTab::Home => rsx! { HomeScreen {} },
                    ActiveTab::Explore => rsx! { ExploreScreen {} },
                    ActiveTab::Plus => rsx! { CreateBeerScreen {} }, // Or Modal? For now screen.
                    ActiveTab::Clubs => rsx! { ClubsScreen {} },
                    ActiveTab::Profile => rsx! { ProfileScreen {} },
                }
            }

            // Fixed Navbar
            Navbar {
                active_tab: active_tab(),
                on_change: move |tab| active_tab.set(tab),
            }
        }
    }
}
