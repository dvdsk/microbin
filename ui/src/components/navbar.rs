use dioxus::prelude::*;

const ASSET: Asset = asset!("/assets/logo.png");


pub fn Navbar() -> Element {
    rsx! {
        nav {
            class: "navbar",
            img {
                width: 100,
                src: ASSET,
                alt: "Logo",
                class: "logo",
            },
            Link {
                to: "/",
                "New"
            },
            Link {
                to: "/list",
                "List"
            },
            Link {
                to: "/guide",
                "Guide"
            }
        }
    }
}