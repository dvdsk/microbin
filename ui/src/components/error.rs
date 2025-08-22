use crate::components::footer::{Footer, FooterProps};
use crate::components::navbar::Navbar;
use crate::components::navbar::NavbarProps;
use dioxus::prelude::*;
use models::args::{Args, PublicUrl};

#[derive(PartialEq, Props, Clone)]
pub struct ErrorProps {
    pub footer_props: FooterProps,
    pub hide_logo: bool,
    pub title: Option<String>,
    pub no_listing: bool,
    pub public_path: String,
}

impl From<Args> for ErrorProps {
    fn from(args: Args) -> Self {
        ErrorProps {
            footer_props: FooterProps {
                hide_footer: args.hide_footer,
                footer_text: args.footer_text,
            },
            no_listing: args.no_listing,
            title: args.title.clone(),
            hide_logo: args.hide_logo,
            public_path: args.public_path.unwrap_or(PublicUrl::default()).into(),
        }
    }
}

pub fn Error(props: ErrorProps) -> Element {
    rsx! {
                  head{
                link { rel: "icon", href: "/assets/favicon.ico" },
                link { rel: "stylesheet", href: "/assets/styling/water.css" }
            }
            body {
                        Navbar{
                ..<ErrorProps as Into<NavbarProps>>::into(props.clone())
            },
            h1 {
                "404 - Not Found"
            },
            a {
                href: props.public_path,
                "Go Home"
            },

                        Footer  {
                hide_footer: props.footer_props.hide_footer,
            }
            }
    }
}
