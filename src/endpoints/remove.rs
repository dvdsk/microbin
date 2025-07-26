use crate::args::ARGS;
use crate::endpoints::errors::ErrorTemplate;
use crate::error_handling::AppError;
use crate::pasta::PastaFile;
use crate::util::animalnumbers::to_u64;
use crate::util::auth;
use crate::util::db::delete;
use crate::util::hashids::to_u64 as hashid_to_u64;
use crate::util::misc::{decrypt, remove_expired};
use crate::AppState;
use askama::Template;
use axum::extract::{Multipart, Path, State};
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::routing::{get, post};
use axum::Router;
use reqwest::header;
use std::fs;

pub async fn remove(State(data): State<AppState>, Path(id): Path<String>) -> impl IntoResponse {
    let mut pastas = data.pastas.lock().unwrap();

    let id = if ARGS.hash_ids {
        hashid_to_u64(&id).unwrap_or(0)
    } else {
        to_u64(&id).unwrap_or(0)
    };

    for (i, pasta) in pastas.iter().enumerate() {
        if pasta.id == id {
            // if it's encrypted or read-only, it needs password to be deleted
            if pasta.encrypt_server || pasta.readonly {
                return (
                    StatusCode::FOUND,
                    [(
                        header::LOCATION,
                        format!(
                            "{}/auth_remove_private/{}",
                            ARGS.public_path_as_str(),
                            pasta.id_as_animals()
                        ),
                    )],
                    "".to_string(),
                );
            }

            // remove the file itself
            if let Some(PastaFile { name, .. }) = &pasta.file {
                if fs::remove_file(format!(
                    "{}/attachments/{}/{}",
                    ARGS.data_dir,
                    pasta.id_as_animals(),
                    name
                ))
                .is_err()
                {
                    log::error!("Failed to delete file {}!", name)
                }

                // and remove the containing directory
                if fs::remove_dir(format!(
                    "{}/attachments/{}/",
                    ARGS.data_dir,
                    pasta.id_as_animals()
                ))
                .is_err()
                {
                    log::error!("Failed to delete directory {}!", name)
                }
            }

            // remove it from in-memory pasta list
            pastas.remove(i);

            delete(Some(&pastas), Some(id));

            return (
                StatusCode::FOUND,
                [(
                    header::LOCATION,
                    format!("{}/list", ARGS.public_path_as_str()),
                )],
                "".to_string(),
            );
        }
    }

    remove_expired(&mut pastas);

    (
        StatusCode::OK,
        [(header::CONTENT_TYPE, "text/html; charset=utf-8".to_string())],
        ErrorTemplate { args: &ARGS }.render().unwrap(),
    )
}

pub async fn post_remove(
    State(data): State<AppState>,
    Path(id): Path<String>,
    payload: Multipart,
) -> Result<impl IntoResponse, AppError> {
    let id = if ARGS.hash_ids {
        hashid_to_u64(&id).unwrap_or(0)
    } else {
        to_u64(&id).unwrap_or(0)
    };

    {
        let mut pastas = data.pastas.lock().unwrap();
        remove_expired(&mut pastas);
    }

    let password = auth::password_from_multipart(payload).await;
    if password.is_err() {
        return Ok((
            StatusCode::OK,
            [(header::CONTENT_TYPE, "text/html; charset=utf-8".to_string())],
            ErrorTemplate { args: &ARGS }.render().unwrap(),
        ));
    }

    let password_unwrapped = password.unwrap();

    {
        let mut pastas = data.pastas.lock().unwrap();
        for (i, pasta) in pastas.iter().enumerate() {
            if pasta.id == id {
                if pastas[i].readonly || pastas[i].encrypt_server {
                    if password_unwrapped != *"" {
                        let res =
                            decrypt(pastas[i].content.to_owned().as_str(), &password_unwrapped);
                        if res.is_ok() {
                            // remove the file itself
                            if let Some(PastaFile { name, .. }) = &pasta.file {
                                if fs::remove_file(format!(
                                    "{}/attachments/{}/{}",
                                    ARGS.data_dir,
                                    pasta.id_as_animals(),
                                    name
                                ))
                                .is_err()
                                {
                                    log::error!("Failed to delete file {}!", name)
                                }

                                // and remove the containing directory
                                if fs::remove_dir(format!(
                                    "{}/attachments/{}/",
                                    ARGS.data_dir,
                                    pasta.id_as_animals()
                                ))
                                .is_err()
                                {
                                    log::error!("Failed to delete directory {}!", name)
                                }
                            }

                            // remove it from in-memory pasta list
                            pastas.remove(i);

                            delete(Some(&pastas), Some(id));

                            let res = (
                                StatusCode::FOUND,
                                [(
                                    header::LOCATION,
                                    format!("{}/list", ARGS.public_path_as_str()),
                                )],
                                "".to_string(),
                            );
                            return Ok(res);
                        } else {
                            let res = (
                                StatusCode::FOUND,
                                [(
                                    header::LOCATION,
                                    format!(
                                        "{}/auth_remove_private/{}/incorrect",
                                        ARGS.public_path_as_str(),
                                        pasta.id_as_animals()
                                    ),
                                )],
                                "".to_string(),
                            );
                            return Ok(res);
                        }
                    } else {
                        let res = (
                            StatusCode::FOUND,
                            [(
                                header::LOCATION,
                                format!(
                                    "{}/auth_remove_private/{}",
                                    ARGS.public_path_as_str(),
                                    pasta.id_as_animals()
                                ),
                            )],
                            "".to_string(),
                        );
                        return Ok(res);
                    }
                }

                let res = (
                    StatusCode::FOUND,
                    [(
                        header::LOCATION,
                        format!(
                            "{}/upload/{}",
                            ARGS.public_path_as_str(),
                            pastas[i].id_as_animals()
                        ),
                    )],
                    "".to_string(),
                );
                return Ok(res);
            }
        }
    }

    Ok((
        StatusCode::OK,
        [(header::CONTENT_TYPE, "text/html; charset=utf-8".to_string())],
        ErrorTemplate { args: &ARGS }.render().unwrap(),
    ))
}

pub fn remove_router() -> Router<AppState> {
    Router::new()
        .route("/remove/{id}", post(post_remove))
        .route("/remove/{id}", get(remove))
}
