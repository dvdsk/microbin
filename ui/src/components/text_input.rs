use dioxus::logger::tracing::span::Attributes;
use dioxus::prelude::*;

#[derive(PartialEq, Props, Clone)]
pub struct TextInputProps {
    pub id: String,
    pub placeholder: String,
    pub value: String,
    pub oninput: fn(String) -> (),
    pub disabled: bool,
}

pub fn TextInput(props: TextInputProps) -> Element {
    let TextInputProps {
        id,
        placeholder,
        value,
        oninput,
        disabled,
    } = props;

    rsx! {
        input {
            id: id,
            class: "text-input",
            placeholder: placeholder,
            value: value,
            disabled: disabled,
            oninput: move |e| {
                let value = e.value();
                oninput(value)
            },
        }
    }
}
