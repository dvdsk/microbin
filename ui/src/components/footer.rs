use crate::components::list::ListProps;
use dioxus::dioxus_core::Element;
use dioxus::html::completions::CompleteWithBraces::a;
use dioxus::prelude::*;
use models::args::Args;

#[derive(PartialEq, Props, Clone)]
pub struct FooterProps {
    pub hide_footer: bool,
    pub footer_text: Option<String>,
}

impl From<ListProps> for FooterProps {
    fn from(list_props: ListProps) -> Self {
        FooterProps {
            hide_footer: list_props.hide_logo,
            footer_text: list_props.footer_text,
        }
    }
}

impl From<Args> for FooterProps {
    fn from(args: Args) -> Self {
        FooterProps {
            hide_footer: args.hide_footer,
            footer_text: args.footer_text,
        }
    }
}

pub fn Footer(hide_footer: FooterProps) -> Element {
    if !hide_footer.hide_footer {
        rsx! {
            footer {
                id: "footer",
                {hide_footer.footer_text.map(|e|{
                    rsx!{
                        p { "{e}"}
                    }
                }).or_else(||{
                     rsx! {
                        div {
                            class: "footer",
                                                    a {
                            href: "https://microbin.eu",
                            target: "_blank",
                            rel: "noopener noreferrer",
                            class: "footer-link",
                            "MicroBin "
                        }
                        span {
                            "by Dániel Szabó and the FOSS Community. Let's keep the Web "
                            b {
                                "compact"
                            }
                            b {
                                " accessible "
                            }
                            span {
                                b {
                                    "humane"
                                }
                            }
                        }
                        }
                    }.into()
                })}
            }
        }
    } else {
        rsx! {}
    }
}

pub fn ShowableFooter() -> Element {
    rsx! {
        Footer { hide_footer:  false }
    }
}
