use dioxus::core_macro::Props;
use dioxus::prelude::*;
use crate::dioxus_core::prelude::*;

#[derive(PartialEq, Props, Clone)]
pub struct CreatePastaProps {

}


pub fn CreatePasta(props: CreatePastaProps) -> Element {
    rsx!{
        div{

        }
    }
}