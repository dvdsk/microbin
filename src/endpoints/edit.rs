use crate::args::Args;
use crate::endpoints::errors::ErrorTemplate;
use crate::error_handling::AppError;
use crate::util::animalnumbers::to_u64;
use crate::util::db::update;
use crate::util::hashids::to_u64 as hashid_to_u64;
use crate::util::misc::{decrypt, encrypt, remove_expired};
use crate::{AppState, Pasta, ARGS};
use askama::Template;
use axum::extract::{Multipart, Path, State};
use axum::http::{header, StatusCode};
use axum::response::IntoResponse;
use axum::routing::{get, post};
use axum::Router;
use futures::TryStreamExt;

#[derive(Template)]
#[template(path = "edit.html", escape = "none")]
struct EditTemplate<'a> {
    pasta: &'a Pasta,
    args: &'a Args,
    path: &'a String,
    status: &'a String,
}

pub async fn get_edit(
    State(data): State<AppState>,
    Path(id): Path<String>,
) -> Result<axum::response::Response, AppError> {
    let mut pastas = data.pastas.lock().unwrap();

    let id = if ARGS.hash_ids {
        hashid_to_u64(&id).unwrap_or(0)
    } else {
        to_u64(&id).unwrap_or(0)
    };

    remove_expired(&mut pastas);

    for pasta in pastas.iter() {
        if pasta.id == id {
            if !pasta.editable {
                return Ok((
                    StatusCode::FOUND,
                    [(header::LOCATION, format!("{}/", ARGS.public_path_as_str()))],
                    "".to_string(),
                )
                    .into_response());
            }

            if pasta.encrypt_server {
                return Ok((
                    StatusCode::FOUND,
                    [(
                        header::LOCATION,
                        format!(
                            "{}/auth_edit_private/{}",
                            ARGS.public_path_as_str(),
                            pasta.id_as_animals()
                        ),
                    )],
                )
                    .into_response());
            }

            return Ok((
                StatusCode::OK,
                [(header::CONTENT_TYPE, "text/html; charset=utf-8".to_string())],
                EditTemplate {
                    pasta,
                    args: &ARGS,
                    path: &String::from("edit"),
                    status: &String::from(""),
                }
                .render()
                .unwrap(),
            )
                .into_response());
        }
    }

    Ok((
        StatusCode::OK,
        [(header::CONTENT_TYPE, "text/html; charset=utf-8".to_string())],
        ErrorTemplate { args: &ARGS }.render().unwrap(),
    )
        .into_response())
}

pub async fn get_edit_with_status(
    State(data): State<AppState>,
    Path((id, status)): Path<(String, String)>,
) -> Result<axum::response::Response, AppError> {
    let mut pastas = data.pastas.lock().unwrap();

    let intern_id = if ARGS.hash_ids {
        hashid_to_u64(&id).unwrap_or(0)
    } else {
        to_u64(&id).unwrap_or(0)
    };

    remove_expired(&mut pastas);

    for pasta in pastas.iter() {
        if pasta.id == intern_id {
            if !pasta.editable {
                return Ok((
                    StatusCode::FOUND,
                    [(header::LOCATION, format!("{}/", ARGS.public_path_as_str()))],
                )
                    .into_response());
            }

            if pasta.encrypt_server {
                return Ok((
                    StatusCode::FOUND,
                    [(
                        header::LOCATION,
                        format!(
                            "{}/auth_edit_private/{}",
                            ARGS.public_path_as_str(),
                            pasta.id_as_animals()
                        ),
                    )],
                )
                    .into_response());
            }

            return Ok((
                StatusCode::OK,
                [(header::CONTENT_TYPE, "text/html; charset=utf-8".to_string())],
                EditTemplate {
                    pasta,
                    args: &ARGS,
                    status: &status,
                    path: &String::from("edit"),
                }
                .render()
                .unwrap(),
            )
                .into_response());
        }
    }

    Ok((
        StatusCode::OK,
        [(header::CONTENT_TYPE, "text/html; charset=utf-8".to_string())],
        ErrorTemplate { args: &ARGS }.render().unwrap(),
    )
        .into_response())
}

pub async fn post_edit_private(
    State(data): State<AppState>,
    Path(id): Path<String>,
    mut payload: Multipart,
) -> Result<axum::response::Response, AppError> {
    // get access to the pasta collection

    let id = if ARGS.hash_ids {
        hashid_to_u64(&id).unwrap_or(0)
    } else {
        to_u64(&id).unwrap_or(0)
    };

    let mut password = String::from("");

    while let Some(mut field) = payload.next_field().await? {
        if field.name() == Some("password") {
            while let Some(chunk) = field.try_next().await? {
                password.push_str(std::str::from_utf8(&chunk).unwrap().to_string().as_str());
            }
        }
    }

    {
        let mut pastas = data.pastas.lock().unwrap();
        // remove expired pastas (including this one if needed)
        remove_expired(&mut pastas);
    }

    // find the index of the pasta in the collection based on u64 id
    let mut index: usize = 0;
    let mut found: bool = false;
    {
        let pastas = data.pastas.lock().unwrap();
        for (i, pasta) in pastas.iter().enumerate() {
            if pasta.id == id {
                index = i;
                found = true;
                break;
            }
        }
    }

    {
        let mut pastas = data.pastas.lock().unwrap();
        if found && !pastas[index].encrypt_client {
            let original_content = pastas[index].content.to_owned();

            // decrypt content temporarily
            if password != *"" {
                let res = decrypt(&original_content, &password);
                if let Ok(rs) = res {
                    pastas[index].content.replace_range(.., rs.as_str());
                    // save pasta in database
                    update(Some(&pastas), Some(&pastas[index]));
                } else {
                    return Ok((
                        StatusCode::FOUND,
                        [(
                            header::LOCATION,
                            format!(
                                "{}/auth_edit_private/{}/incorrect",
                                ARGS.public_path_as_str(),
                                pastas[index].id_as_animals()
                            ),
                        )],
                    )
                        .into_response());
                }
            }

            // serve pasta in template
            let response = (
                StatusCode::OK,
                [(header::CONTENT_TYPE, "text/html; charset=utf-8".to_string())],
                EditTemplate {
                    pasta: &pastas[index],
                    args: &ARGS,
                    path: &String::from("submit_edit_private"),
                    status: &String::from(""),
                }
                .render()
                .unwrap(),
            );

            if pastas[index].content != original_content {
                pastas[index].content = original_content;
            }

            return Ok(response.into_response());
        }
    }

    Ok((
        StatusCode::OK,
        [(header::CONTENT_TYPE, "text/html; charset=utf-8".to_string())],
        ErrorTemplate { args: &ARGS }.render().unwrap(),
    )
        .into_response())
}

pub async fn post_submit_edit_private(
    State(data): State<AppState>,
    Path(id): Path<String>,
    mut payload: Multipart,
) -> Result<impl IntoResponse, AppError> {
    let id = if ARGS.hash_ids {
        hashid_to_u64(&id).unwrap_or(0)
    } else {
        to_u64(&id).unwrap_or(0)
    };

    let mut password = String::from("");
    let mut new_content = String::from("");

    while let Some(mut field) = payload.next_field().await? {
        if field.name() == Some("content") {
            while let Some(chunk) = field.try_next().await? {
                new_content.push_str(std::str::from_utf8(&chunk).unwrap().to_string().as_str());
            }
        }
        if field.name() == Some("password") {
            while let Some(chunk) = field.try_next().await? {
                password = std::str::from_utf8(&chunk).unwrap().to_string();
            }
        }
    }

    {
        // get access to the pasta collection
        let mut pastas = data.pastas.lock().unwrap();
        // remove expired pastas (including this one if needed)
        remove_expired(&mut pastas);
    }

    // find the index of the pasta in the collection based on u64 id
    let mut index: usize = 0;
    let mut found: bool = false;
    {
        let pastas = data.pastas.lock().unwrap();

        for (i, pasta) in pastas.iter().enumerate() {
            if pasta.id == id {
                index = i;
                found = true;
                break;
            }
        }
    }

    {
        let mut pastas = data.pastas.lock().unwrap();

        if found && pastas[index].editable && !pastas[index].encrypt_client {
            if pastas[index].readonly {
                let res = decrypt(pastas[index].encrypted_key.as_ref().unwrap(), &password);
                if res.is_ok() {
                    pastas[index]
                        .content
                        .replace_range(.., &encrypt(&new_content, &password));
                } else {
                    return Ok((
                        StatusCode::FOUND,
                        [(
                            header::LOCATION,
                            format!(
                                "{}/edit/{}/incorrect",
                                ARGS.public_path_as_str(),
                                pastas[index].id_as_animals()
                            ),
                        )],
                    )
                        .into_response());
                }
            } else if pastas[index].private {
                let res = decrypt(&pastas[index].content, &password);
                if res.is_ok() {
                    pastas[index]
                        .content
                        .replace_range(.., &encrypt(&new_content, &password));
                    // save pasta in database
                    update(Some(&pastas), Some(&pastas[index]));
                } else {
                    return Ok((
                        StatusCode::FOUND,
                        [(
                            header::LOCATION,
                            format!(
                                "{}/auth_edit_private/{}/incorrect",
                                ARGS.public_path_as_str(),
                                pastas[index].id_as_animals()
                            ),
                        )],
                    )
                        .into_response());
                }
            }

            return Ok((
                StatusCode::FOUND,
                [(
                    header::LOCATION,
                    format!(
                        "{}/auth/{}/success",
                        ARGS.public_path_as_str(),
                        pastas[index].id_as_animals()
                    ),
                )],
            )
                .into_response());
        }
    }

    Ok((
        StatusCode::OK,
        [(header::CONTENT_TYPE, "text/html; charset=utf-8".to_string())],
    )
        .into_response())
}

pub async fn post_edit(
    State(data): State<AppState>,
    Path(id): Path<String>,
    mut payload: Multipart,
) -> Result<axum::response::Response, AppError> {
    let id = if ARGS.hash_ids {
        hashid_to_u64(&id).unwrap_or(0)
    } else {
        to_u64(&id).unwrap_or(0)
    };

    {
        let mut pastas = data.pastas.lock().unwrap();
        remove_expired(&mut pastas);
    }

    let mut new_content = String::from("");
    let mut password = String::from("");

    while let Some(mut field) = payload.next_field().await? {
        if field.name() == Some("content") {
            while let Some(chunk) = field.try_next().await? {
                new_content.push_str(std::str::from_utf8(&chunk).unwrap().to_string().as_str());
            }
        }
        if field.name() == Some("password") {
            while let Some(chunk) = field.try_next().await? {
                password = std::str::from_utf8(&chunk).unwrap().to_string();
            }
        }
    }

    {
        let mut pastas = data.pastas.lock().unwrap();

        for (i, pasta) in pastas.iter().enumerate() {
            if pasta.id == id {
                if pasta.editable && !pasta.encrypt_client {
                    if pastas[i].readonly || pastas[i].encrypt_server {
                        if password != *"" {
                            let res = decrypt(pastas[i].encrypted_key.as_ref().unwrap(), &password);
                            if res.is_ok() {
                                pastas[i].content.replace_range(.., &new_content);
                                // save pasta in database
                                update(Some(&pastas), Some(&pastas[i]));
                            } else {
                                return Ok((
                                    StatusCode::FOUND,
                                    [(
                                        header::LOCATION,
                                        format!(
                                            "{}/edit/{}/incorrect",
                                            ARGS.public_path_as_str(),
                                            pasta.id_as_animals()
                                        ),
                                    )],
                                )
                                    .into_response());
                            }
                        } else {
                            return Ok((
                                StatusCode::FOUND,
                                [(
                                    header::LOCATION,
                                    format!(
                                        "{}/edit/{}/incorrect",
                                        ARGS.public_path_as_str(),
                                        pasta.id_as_animals()
                                    ),
                                )],
                            )
                                .into_response());
                        }
                    } else {
                        pastas[i].content.replace_range(.., &new_content);
                        // save pasta in database
                        update(Some(&pastas), Some(&pastas[i]));
                    }

                    return Ok((
                        StatusCode::FOUND,
                        [(
                            header::LOCATION,
                            format!(
                                "{}/upload/{}",
                                ARGS.public_path_as_str(),
                                pastas[i].id_as_animals()
                            ),
                        )],
                    )
                        .into_response());
                } else {
                    break;
                }
            }
        }
    }

    Ok((
        StatusCode::OK,
        [(header::CONTENT_TYPE, "text/html; charset=utf-8".to_string())],
        ErrorTemplate { args: &ARGS }.render().unwrap(),
    )
        .into_response())
}

pub fn edit_router() -> Router<AppState> {
    Router::new()
        .route("/edit/{id}", get(get_edit))
        .route("/edit/{id}/{status}", get(get_edit_with_status))
        .route("/edit_private/{id}", post(post_edit_private))
        .route("/submit_edit_private/{id}", post(post_submit_edit_private))
        .route("/edit/{id}", post(post_edit))
}
