use dioxus::prelude::*;
use crate::components::list::ListProps;

pub fn ListTable(list_props: ListProps) -> Element {
    rsx!{
        table {
            class: "list-table",
            thead {
                th {
                    style: "width: 25%;"
                    "Key"
                },
                th {
                    style: "width: 10%;"
                },
                th {
                    style: "width: 15%;"
                    "Created"
                },
                th {
                    style: "width: 15%;"
                    "Expiration"
                },
                th {
                    style: "width: 15%;"
                    "Contents"
                },
                th {
                    style: "width: 10%;"
                },
                th {
                    style: "width: 10%;"
                }
            }
        }
    }
}