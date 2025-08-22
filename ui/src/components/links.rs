use crate::Route;
use dioxus::prelude::*;

static CSS: Asset = asset!("/assets/styling/links.css");

pub fn Links() -> Element {
    rsx! {
        document::Link { rel: "stylesheet", href: CSS }
        div {
             id: "links",
           Link{
            to: Route::Home {},
            "Home"
        },
        Link{
            to: Route::ShowableFooter {},
            "Footer"
        },
        Link{
            to: Route::Navbar {},
            "Navbar"
        },
        Link {
                to: Route::CreatePasta {},
                "Create pasta"
        },
            Link {
                to: Route::ShowableGuide {},
                "Guide"
            }
        }
    }
}
