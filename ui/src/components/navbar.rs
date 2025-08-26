use crate::components::error::ErrorProps;
use crate::components::list::ListProps;
use dioxus::prelude::*;

#[derive(PartialEq, Props, Clone)]
pub struct NavbarProps {
    hide_logo: bool,
    title: Option<String>,
    no_listing: bool,
}

impl From<ListProps> for NavbarProps {
    fn from(list_props: ListProps) -> Self {
        NavbarProps {
            hide_logo: list_props.hide_logo,
            title: list_props.title,
            no_listing: list_props.no_listing,
        }
    }
}

impl From<ErrorProps> for NavbarProps {
    fn from(error_props: ErrorProps) -> Self {
        NavbarProps {
            hide_logo: error_props.hide_logo,
            title: error_props.title,
            no_listing: error_props.no_listing,
        }
    }
}

#[allow(non_snake_case)]
pub fn Navbar(props: NavbarProps) -> Element {
    rsx! {
    nav {
        class: "navbar",
        {if !props.hide_logo {
            rsx!{img
        {
            width: 100,
            src: "/assets/logo.png",
            alt: "Logo",
            class: "logo",
        }}} else {
            rsx!{}
        }},
        a {
            href: "/",
            "New"
        },
        {if !props.no_listing {
            rsx!{
                a
         {
            href: "/list",
            "List"
        }}} else {
            rsx!{}
            }
        },
        a {
            href: "/guide",
            "Guide"
        }
    }
    }
}
