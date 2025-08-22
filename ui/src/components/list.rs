use dioxus::core_macro::Props;
use dioxus::html::KeyCode::F;
use dioxus::prelude::*;
use crate::components::footer::FooterProps;
use models::args::Args;
use crate::components::navbar::NavbarProps;
use models::pasta::Pasta;
use crate::components::footer::Footer;
use crate::components::navbar::Navbar;

#[derive(PartialEq, Props, Clone)]
pub struct ListProps {
    pub footer_text: Option<String>,
    pub pastas: Vec<Pasta>,
    pub public_path: String,
    pub hide_logo: bool,
    pub title: Option<String>,
    pub no_listing: bool,
}

impl From<(Args, Vec<Pasta>)> for ListProps {
    fn from(args: (Args, Vec<Pasta>)) -> Self {
        ListProps {
            pastas: args.1,
            public_path: args.0.public_path.unwrap_or_default().into(),
            hide_logo: args.0.hide_logo,
            title: args.0.title.clone(),
            no_listing: args.0.no_listing,
            footer_text: args.0.footer_text,
        }
    }
}

pub fn List(list_props: ListProps) -> Element {
    if list_props.pastas.is_empty() {
        rsx! {
        head
    {
        link { rel: "icon",
        href: "/assets/favicon.ico"
    },
    link { rel: "stylesheet", href: "/assets/styling/water.css" }
    }
    body {
    Navbar{
                    ..<ListProps as Into<NavbarProps>>::into(list_props.clone().into()),
                },
                p {
                    span{"No uploads yet. 😔 Create one "}
                    a{
                        href: list_props.public_path,
                        "here"
                    }
                    },
                Footer{
                    ..<ListProps as Into<FooterProps>>::into(list_props.clone())
                }
                }

    }
    } else {
             rsx! {
        head
    {
        link { rel: "icon",
        href: "/assets/favicon.ico"
    },
    link { rel: "stylesheet", href: "/assets/styling/water.css" }
    }
    body {
            Navbar{
                    ..<ListProps as Into<NavbarProps>>::into(list_props.clone().into()),
                },
    }
        }
    }
}
