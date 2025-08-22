use dioxus::logger::tracing::log;
use crate::components::list::ListProps;
use dioxus::prelude::*;

pub fn ListTable(list_props: ListProps) -> Element {
    let public_js_path = format!("{}assets/js/list.js", list_props.public_path);
    rsx! {
            script{
                src: public_js_path
            }
            h2 {
                "Uploads"
            }
            table {
                class: "list-table",
                thead {
                    th {
                        style: "width: 25%;",
                        "Key"
                    },
                    th {
                        style: "width: 10%;",
                    },
                    th {
                        style: "width: 15%;",
                        "Created"
                    },
                    th {
                        style: "width: 15%;",
                        "Expiration"
                    },
                    th {
                        style: "width: 15%;",
                        "Contents"
                    },
                    th {
                        style: "width: 10%;",
                    },
                    th {
                        style: "width: 10%;",
                    }
                },
                tbody{
                    {
                        list_props.pastas.iter().map(|item|{
                        let item = item.clone();
                        let content_url: String;
                        let content_text: String;
                        let edit_url = format!("{}edit/{}", list_props.public_path, item
                            .id_as_animals(&list_props.hash_ids));

                        if !item.content.is_empty() {
                            content_url = format!("{}raw/{}", list_props.public_path, item
                                .id_as_animals(&list_props.hash_ids));
                            content_text = "Text".to_string();
                        } else if let Some(file) = &item.file {
                            if file.is_image() {
                                content_text = "Image".to_string();
                                content_url = format!("{}file/{}", list_props.public_path, item
                                    .id_as_animals(&list_props.hash_ids));
                            } else if file.is_video() {
                                content_text = "Video".to_string();
                                content_url = format!("{}file/{}", list_props.public_path, item
                                    .id_as_animals(&list_props.hash_ids));
                            }  else {
                                content_text = "File".to_string();
                                content_url = format!("{}file/{}", list_props.public_path, item
                                    .id_as_animals(&list_props.hash_ids));
                            }
                        } else {
                            content_text = "None".to_string();
                            content_url = "".to_string();
                        }



                        return if item.pasta_type == "text" && !item.private {
                            let base_url = format!("{}upload/{}", list_props.public_path, item
                                .id_as_animals(&list_props.hash_ids));
                            rsx!{
                                tr {
                                    td {
                                        a {
                                           href: base_url.clone(),
                                            "{item.id_as_animals(&list_props.hash_ids)}"
                                        }
                                    }
                                    td {
        {
            if !list_props.public_path.is_empty() {
                if list_props.short_path.is_empty() {
                let value = list_props.public_path.clone();
                    rsx! {
                        a {
                            style: "margin-right:1rem; cursor: pointer;",
                            class: "copy-button",
                            data: format!("{}upload/{}", value, item.id_as_animals(&list_props
                            .hash_ids)),
                            "Copy"
                        }
                    }
                } else {
                let value = list_props.short_path.clone();
                    rsx! {
                        a {
                            style: "margin-right:1rem; cursor: pointer;",
                            class: "copy-button",
                            data: format!("{}upload/{}", value, item.id_as_animals(&list_props
                            .hash_ids)),
                            "Copy"
                        }
                    }
                }
            } else {
            rsx!{}
        }
        }
    }, td {
                                        "{item.created}"
                                    },
                                    td {
                                        "{item.expiration}"
                                    },
                                    td {
                                        a {
                                            href: "{content_url}",
                                            "{content_text}"
                                        }
                                    },
                                    {if item.editable {
                                        rsx!{
                                            td {a{
                                                href: edit_url,
                                                "Edit"
                                            }}
                                        }
                                    } else {
                                        rsx!{
                                            td{}
                                        }
                                    }
                                        }
                                    td {
                                        a {
                                            href: format!("{}/remove/{}", list_props.public_path, item.id_as_animals(&list_props.hash_ids)),
                                            "Remove"
                                        }
                                    }
                                }
                        }
                        } else {
                            rsx!{

                            }
                        }
                    })
                    }
                }
            }
        }
}
