use crate::components::footer::{Footer, FooterProps};
use crate::components::navbar::Navbar;
use dioxus::prelude::*;
use models::args::Args;

#[derive(PartialEq, Props, Clone)]
pub struct GuideProps {
    pub footer_props: FooterProps,
    pub hide_logo: bool,
    pub title: Option<String>,
    pub enable_burn_after: bool,
    pub highlightsyntax: bool,
    pub no_listing: bool,
}

impl From<Args> for GuideProps {
    fn from(args: Args) -> Self {
        GuideProps {
            footer_props: FooterProps {
                hide_footer: args.hide_footer,
                footer_text: args.footer_text,
            },
            hide_logo: args.hide_logo,
            title: args.title,
            enable_burn_after: args.enable_burn_after,
            highlightsyntax: args.highlightsyntax,
            no_listing: args.no_listing,
        }
    }
}

#[allow(non_snake_case)]
pub fn Guide(props: GuideProps) -> Element {
    rsx! {
            head{
                link { rel: "icon", href: "/assets/favicon.ico" },
                link { rel: "stylesheet", href: "/assets/styling/water.css" }
            }
            body {
                        Navbar{
                    hide_logo: props.hide_logo,
                    title: props.title,
                    no_listing: props.no_listing,
            },
            h1 {
                "Options"
            }

            h2 {
                "Expiration"
            }
            section{
                "Use the expiration dropdown to choose how long you want your upload to exist. When the selected time has expired, it will be removed from the server."
            }
            { if props.enable_burn_after {
                rsx!{
                    h2
                {
                "Burn After"
            },
                            section{
                "Use the burn after dropdown to set a limit on how many times your data can be accessed before it will be removed from the server."
            }
                }
    } else {rsx!{}}}
            {if props.highlightsyntax{rsx!{h2 {
                "Syntax Highlighting"
            }
            section{
                "Use the syntax highlighting dropdown to enable syntax highlighting for your upload, making it easier to read. You may choose to have the syntax highlighting done by your browser, which will also recognise the language automatically. You can select server-side highlighting, where you need to select the language yourself, but the code will get highlighting without javascript."
            }}} else {rsx!{}}}
                }

            h2 {
                "Password"
            }
            section{
                "Use the password field to set a password for your upload. This will encrypt your data while stored on our server with your password, and you will need to enter the password to access (in case of private and secret uploads) or to modify (in case of read-only uploads). Your password is encrypted, and in case of secret uploads, we never even see it."
            }
            h2 {
                "Privacy"
            }
            section{
                "Use this dropdown to select the level of protection your upload needs. Use lower privacy levels if you or your organisation host MicroBin, and higher privacy levels if you are using a public MicroBin service."
            }
            h3 {
                "Level 1: Public"
            }
            section {
                "This privacy level allows everyone to find, see, modify and remove your upload."
            }
            h3 {
                "Level 2: Unlisted (recommended)"
            }
            section {
                "Unlisted uploads cannot be found unless someone knows its unique, random identifier. If someone knows this identifier, they can see, modify and remove the upload."
            }
            h3 {
                "Level 3: Read-only"
            }
            section {
                "With this privacy setting, the upload cannot be found unless someone knows its unique, random identifier. If someone knows this identifier, they can see the contents but cannot modify or remove it without entering the password of the upload."
            }
            h3 {
                "Level 4: Private"
            }
            section {
                "With this privacy setting, the upload cannot be found unless someone knows its unique, random identifier. If someone knows this identifier, they cannot see, modify or remove it without entering the password of the upload. Your upload and its attachments are encrypted, so they are stored safely."
            }
            h3 {
                "Level 5: Secret"
            }
            section {
                "With this privacy setting, the upload cannot be found unless someone knows its unique, random identifier. If someone knows this identifier, they cannot see, modify or remove it without entering the password of the upload. Your browser sends us an already encrypted version, so the unencrypted data and password never even leave your device. This option requires you to enter your password many times when accessing your data, but is extremely safe."
            }
            div {
                style: "margin-top: 2rem;"

            }
            Footer  {
                hide_footer: props.footer_props.hide_footer,
            }
            }
}
