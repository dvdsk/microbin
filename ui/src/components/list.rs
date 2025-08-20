use dioxus::core_macro::Props;
use dioxus::prelude::*;
use models::pasta::Pasta;

#[derive(PartialEq, Props, Clone)]
pub struct ListProps {
    pastas: Vec<Pasta>
}