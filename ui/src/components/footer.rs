use dioxus::dioxus_core::Element;
use dioxus::html::completions::CompleteWithBraces::a;
use dioxus::prelude::*;
static CSS: Asset = asset!("/assets/styling/footer.css");

#[derive(PartialEq, Props, Clone)]
pub struct FooterProps {
    pub hide_footer: bool,
    pub footer_text: Option<String>,
}


pub fn Footer(hide_footer: FooterProps) -> Element {
    if !hide_footer.hide_footer {
        rsx! {
            document::Link { rel: "stylesheet", href: CSS }
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
                                                    Link {
                            to: "https://microbin.eu",
                            new_tab: true,
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