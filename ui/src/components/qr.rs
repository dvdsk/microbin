use crate::components::footer::Footer;
use crate::components::navbar::Navbar;
use dioxus::core_macro::Props;
use dioxus::prelude::*;
use models::args::Args;
use models::pasta::Pasta;

#[derive(PartialEq, Props, Clone)]
pub struct QRProps {
    pub hash_ids: bool,
    pub hide_logo: bool,
    pub title: Option<String>,
    pub no_listing: bool,
    pub public_path: String,
    pub hide_footer: bool,
    pub footer_text: Option<String>,
    pub qr_code: String,
    pub pasta: Pasta,
}

impl From<(Args, String, Pasta)> for QRProps {
    fn from(args: (Args, String, Pasta)) -> Self {
        QRProps {
            hide_logo: args.0.hide_logo,
            title: args.0.title.clone(),
            no_listing: args.0.no_listing,
            public_path: args.0.public_path.unwrap_or_default().into(),
            hide_footer: args.0.hide_footer,
            footer_text: args.0.footer_text,
            qr_code: args.1,
            pasta: args.2,
            hash_ids: args.0.hash_ids,
        }
    }
}

#[allow(non_snake_case)]
pub fn QRCode(props: QRProps) -> Element {
    let url_to_pasta = match props.pasta.pasta_type == "url" {
        true => {
            format!(
                "{}url/{}",
                props.public_path,
                props.pasta.id_as_animals(&props.hash_ids)
            )
        }
        false => {
            format!(
                "{}upload/{}",
                props.public_path,
                props.pasta.id_as_animals(&props.hash_ids)
            )
        }
    };
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
            hide_logo: props.hide_logo,
            title: props.title.clone(),
            no_listing: props.no_listing,
        },
        a {
            class: "qr-code",
            href: url_to_pasta,
            dangerous_inner_html: props.qr_code,
        }
        Footer {
            hide_footer: props.hide_footer,
            footer_text: props.footer_text.clone(),
        }
        }
    }
}
