use dioxus::prelude::*;
use dioxus_free_icons::icons::ld_icons::{LdHome, LdCompass, LdUser, LdUsers, LdPlus};
use dioxus_free_icons::Icon;

#[derive(Clone, PartialEq, Copy)]
pub enum ActiveTab {
    Home,
    Explore,
    Plus,
    Clubs,
    Profile,
}

#[component]
pub fn Navbar(
    active_tab: ActiveTab,
    on_change: EventHandler<ActiveTab>
) -> Element {
    let icon_size = 28;
    
    rsx! {
        nav {
            class: "glass-navbar",
            style: "
                position: fixed; 
                bottom: 0; 
                left: 0; 
                right: 0; 
                height: 70px; 
                display: flex; 
                justify-content: space-around; 
                align-items: center; 
                background: rgba(20, 20, 20, 0.9); 
                backdrop-filter: blur(12px); 
                -webkit-backdrop-filter: blur(12px);
                border-top: 1px solid rgba(255, 255, 255, 0.1);
                z-index: 1000;
                padding-bottom: env(safe-area-inset-bottom);
            ",
            
            // Home
            div {
                class: "nav-item",
                style: "display: flex; flex-direction: column; align-items: center; cursor: pointer; transition: transform 0.2s;",
                onclick: move |_| on_change.call(ActiveTab::Home),
                Icon {
                    width: icon_size,
                    height: icon_size,
                    icon: LdHome,
                    fill: if active_tab == ActiveTab::Home { "var(--primary)" } else { "var(--text-secondary)" },
                }
            }

            // Explore
            div {
                class: "nav-item",
                style: "display: flex; flex-direction: column; align-items: center; cursor: pointer; transition: transform 0.2s;",
                onclick: move |_| on_change.call(ActiveTab::Explore),
                Icon {
                    width: icon_size,
                    height: icon_size,
                    icon: LdCompass,
                    fill: if active_tab == ActiveTab::Explore { "var(--primary)" } else { "var(--text-secondary)" },
                }
            }
            
            // Plus (Middle)
            div {
                class: "nav-item-plus",
                style: "margin-top: -30px; cursor: pointer; transition: transform 0.2s;",
                onclick: move |_| on_change.call(ActiveTab::Plus),
                div {
                    style: "background: var(--background); border-radius: 50%; padding: 5px; box-shadow: 0 0 10px var(--primary);",
                    Icon {
                        width: 40,
                        height: 40,
                        icon: LdPlus,
                        fill: "var(--primary)", // For Lucide, fill controls stroke color
                    }
                }
            }

            // Clubs
            div {
                class: "nav-item",
                style: "display: flex; flex-direction: column; align-items: center; cursor: pointer; transition: transform 0.2s;",
                onclick: move |_| on_change.call(ActiveTab::Clubs),
                Icon {
                    width: icon_size,
                    height: icon_size,
                    icon: LdUsers,
                    fill: if active_tab == ActiveTab::Clubs { "var(--primary)" } else { "var(--text-secondary)" },
                }
            }

            // Profile
            div {
                class: "nav-item",
                style: "display: flex; flex-direction: column; align-items: center; cursor: pointer; transition: transform 0.2s;",
                onclick: move |_| on_change.call(ActiveTab::Profile),
                Icon {
                    width: icon_size,
                    height: icon_size,
                    icon: LdUser,
                    fill: if active_tab == ActiveTab::Profile { "var(--primary)" } else { "var(--text-secondary)" },
                }
            }
        }
    }
}
