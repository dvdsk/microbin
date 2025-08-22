use crate::dioxus_core::prelude::*;
use dioxus::core_macro::Props;
use dioxus::prelude::*;

#[derive(PartialEq, Props, Clone)]
pub struct CreatePastaProps {}

pub fn CreatePasta(props: CreatePastaProps) -> Element {
    rsx! {
        div{

        }
    }
}
