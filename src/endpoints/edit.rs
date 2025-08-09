use crate::args::Args;
use crate::endpoints::errors::ErrorTemplate;
use crate::error_handling::AppError;
use crate::util::animalnumbers::to_u64;
use crate::util::hashids::to_u64 as hashid_to_u64;
use crate::util::misc::{decrypt, encrypt, remove_expired};
use crate::{AppState, Pasta};
use askama::Template;
use axum::Router;
use axum::extract::{Multipart, Path, State};
use axum::http::{HeaderMap, StatusCode, header};
use axum::response::IntoResponse;
use axum::routing::{get, post};
use db::entities::pasta::PastaEntity;
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
    State(AppState { args, db }): State<AppState>,
    Path(id): Path<String>,
) -> Result<axum::response::Response, AppError> {
    let id = if args.hash_ids {
        hashid_to_u64(&id).unwrap_or(0)
    } else {
        to_u64(&id).unwrap_or(0)
    };

    let found_pasta = db.get_pasta(&id)?;
    match found_pasta {
        Some(pasta) => {
            let pasta: Pasta = pasta.into();
            if !pasta.editable {
                return Ok((
                    StatusCode::FOUND,
                    [(header::LOCATION, format!("{}/", args.public_path_as_str()))],
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
                            args.public_path_as_str(),
                            pasta.id_as_animals(&args.hash_ids)
                        ),
                    )],
                )
                    .into_response());
            }

            Ok((
                StatusCode::OK,
                [(header::CONTENT_TYPE, "text/html; charset=utf-8".to_string())],
                EditTemplate {
                    pasta: &pasta,
                    args: &args,
                    path: &String::from("edit"),
                    status: &String::from(""),
                }
                .render()
                .map_err(AppError::from)?,
            )
                .into_response())
        }
        None => Ok((
            StatusCode::OK,
            [(header::CONTENT_TYPE, "text/html; charset=utf-8".to_string())],
            ErrorTemplate { args: &args }.render()?,
        )
            .into_response()),
    }
}

pub async fn get_edit_with_status(
    State(AppState { args, db }): State<AppState>,
    Path((id, status)): Path<(String, String)>,
) -> Result<axum::response::Response, AppError> {
    let intern_id = if args.hash_ids {
        hashid_to_u64(&id).unwrap_or(0)
    } else {
        to_u64(&id).unwrap_or(0)
    };

    let opt_pasta = db.get_pasta(&intern_id)?;

    let pasta = match opt_pasta {
        Some(pasta) => Ok::<PastaEntity, AppError>(pasta),
        None => {
            return Ok((
                StatusCode::OK,
                [(header::CONTENT_TYPE, "text/html; charset=utf-8".to_string())],
                ErrorTemplate { args: &args }.render()?,
            )
                .into_response());
        }
    }?;
    let pasta: Pasta = pasta.into();

    if !pasta.editable {
        return Ok((
            StatusCode::FOUND,
            [(header::LOCATION, format!("{}/", args.public_path_as_str()))],
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
                    args.public_path_as_str(),
                    pasta.id_as_animals(&args.hash_ids)
                ),
            )],
        )
            .into_response());
    }

    Ok((
        StatusCode::OK,
        [(header::CONTENT_TYPE, "text/html; charset=utf-8".to_string())],
        EditTemplate {
            pasta: &pasta,
            args: &args,
            status: &status,
            path: &String::from("edit"),
        }
        .render()?,
    )
        .into_response())
}

pub async fn post_edit_private(
    State(AppState { args, db }): State<AppState>,
    Path(id): Path<String>,
    mut payload: Multipart,
) -> Result<axum::response::Response, AppError> {
    // get access to the pasta collection

    let id = if args.hash_ids {
        hashid_to_u64(&id).unwrap_or(0)
    } else {
        to_u64(&id).unwrap_or(0)
    };

    let pasta_entity = match db.get_pasta(&id)? {
        Some(pasta) => Ok::<PastaEntity, AppError>(pasta),
        None => {
            return Ok((
                StatusCode::OK,
                [(header::CONTENT_TYPE, "text/html; charset=utf-8".to_string())],
                ErrorTemplate { args: &args }.render()?,
            )
                .into_response());
        }
    }?;

    let mut pasta: Pasta = pasta_entity.into();

    let mut password = String::from("");

    while let Some(mut field) = payload.next_field().await? {
        if field.name() == Some("password") {
            while let Some(chunk) = field.try_next().await? {
                password.push_str(std::str::from_utf8(&chunk)?.to_string().as_str());
            }
        }
    }
    if !pasta.encrypt_client {
        let original_content = pasta.content.to_owned();

        // decrypt content temporarily
        if password != *"" {
            let res = decrypt(&original_content, &password);
            if let Ok(rs) = res {
                pasta.content.replace_range(.., rs.as_str());
                // save pasta in database
                db.update_pasta(&id, pasta.clone().into())?;
            } else {
                return Ok((
                    StatusCode::FOUND,
                    [(
                        header::LOCATION,
                        format!(
                            "{}/auth_edit_private/{}/incorrect",
                            args.public_path_as_str(),
                            pasta.id_as_animals(&args.hash_ids)
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
                pasta: &pasta,
                args: &args,
                path: &String::from("submit_edit_private"),
                status: &String::from(""),
            }
            .render()?,
        );

        if pasta.content != original_content {
            pasta.content = original_content;
        }

        return Ok(response.into_response());
    }
    Ok((
        StatusCode::OK,
        [(header::CONTENT_TYPE, "text/html; charset=utf-8".to_string())],
        ErrorTemplate { args: &args }.render()?,
    )
        .into_response())
}

pub async fn post_submit_edit_private(
    State(AppState { args, db }): State<AppState>,
    Path(id): Path<String>,
    mut payload: Multipart,
) -> Result<axum::response::Response, AppError> {
    let id = if args.hash_ids {
        hashid_to_u64(&id).unwrap_or(0)
    } else {
        to_u64(&id).unwrap_or(0)
    };

    let mut pasta = match db.get_pasta(&id)? {
        Some(pasta) => Ok::<Pasta, AppError>(pasta.into()),
        None => {
            let mut headers = HeaderMap::new();
            headers.insert("Content-Type", "text/html; charset=utf-8".parse()?);
            let body = ErrorTemplate { args: &args }.render()?;
            return Ok((headers, body).into_response());
        }
    }?;

    let mut password = String::from("");
    let mut new_content = String::from("");

    while let Some(mut field) = payload.next_field().await? {
        if field.name() == Some("content") {
            while let Some(chunk) = field.try_next().await? {
                new_content.push_str(std::str::from_utf8(&chunk)?.to_string().as_str());
            }
        }
        if field.name() == Some("password") {
            while let Some(chunk) = field.try_next().await? {
                password = std::str::from_utf8(&chunk)?.to_string();
            }
        }
    }
    if pasta.editable && !pasta.encrypt_client {
        if pasta.readonly
            && let Some(encrypted_key) = pasta.encrypted_key.as_ref()
        {
            let res = decrypt(encrypted_key, &password);
            if res.is_ok() {
                pasta
                    .content
                    .replace_range(.., &encrypt(&new_content, &password));
            } else {
                return Ok((
                    StatusCode::FOUND,
                    [(
                        header::LOCATION,
                        format!(
                            "{}/edit/{}/incorrect",
                            args.public_path_as_str(),
                            pasta.id_as_animals(&args.hash_ids)
                        ),
                    )],
                )
                    .into_response());
            }
        } else if pasta.private {
            let res = decrypt(&pasta.content, &password);
            if res.is_ok() {
                pasta
                    .content
                    .replace_range(.., &encrypt(&new_content, &password));
                // save pasta in database
                db.update_pasta(&id, pasta.clone().into())?;
            } else {
                return Ok((
                    StatusCode::FOUND,
                    [(
                        header::LOCATION,
                        format!(
                            "{}/auth_edit_private/{}/incorrect",
                            args.public_path_as_str(),
                            pasta.id_as_animals(&args.hash_ids)
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
                    args.public_path_as_str(),
                    pasta.id_as_animals(&args.hash_ids)
                ),
            )],
        )
            .into_response());
    }

    Ok((
        StatusCode::OK,
        [(header::CONTENT_TYPE, "text/html; charset=utf-8".to_string())],
    )
        .into_response())
}

pub async fn post_edit(
    State(AppState { args, db }): State<AppState>,
    Path(id): Path<String>,
    mut payload: Multipart,
) -> Result<axum::response::Response, AppError> {
    let id = if args.hash_ids {
        hashid_to_u64(&id).unwrap_or(0)
    } else {
        to_u64(&id).unwrap_or(0)
    };

    let pasta_entity = match db.get_pasta(&id)? {
        Some(pasta) => Ok::<PastaEntity, AppError>(pasta),
        None => {
            return Ok((
                StatusCode::OK,
                [(header::CONTENT_TYPE, "text/html; charset=utf-8".to_string())],
                ErrorTemplate { args: &args }.render()?,
            )
                .into_response());
        }
    }?;

    let mut pasta: Pasta = pasta_entity.into();

    let mut new_content = String::from("");
    let mut password = String::from("");

    while let Some(mut field) = payload.next_field().await? {
        if field.name() == Some("content") {
            while let Some(chunk) = field.try_next().await? {
                new_content.push_str(std::str::from_utf8(&chunk)?.to_string().as_str());
            }
        }
        if field.name() == Some("password") {
            while let Some(chunk) = field.try_next().await? {
                password = std::str::from_utf8(&chunk)?.to_string();
            }
        }
    }
    if pasta.editable && !pasta.encrypt_client {
        if pasta.readonly || pasta.encrypt_server {
            if password != *""
                && let Some(encrypted_key) = pasta.encrypted_key.as_ref()
            {
                let res = decrypt(encrypted_key, &password);
                if res.is_ok() {
                    pasta.content.replace_range(.., &new_content);
                    // save pasta in database
                    db.update_pasta(&id, pasta.clone().into())?;
                } else {
                    return Ok((
                        StatusCode::FOUND,
                        [(
                            header::LOCATION,
                            format!(
                                "{}/edit/{}/incorrect",
                                args.public_path_as_str(),
                                pasta.id_as_animals(&args.hash_ids)
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
                            args.public_path_as_str(),
                            pasta.id_as_animals(&args.hash_ids)
                        ),
                    )],
                )
                    .into_response());
            }
        } else {
            pasta.content.replace_range(.., &new_content);
            // save pasta in database
            db.update_pasta(&id, pasta.clone().into())?;
        }

        return Ok((
            StatusCode::FOUND,
            [(
                header::LOCATION,
                format!(
                    "{}/upload/{}",
                    args.public_path_as_str(),
                    pasta.id_as_animals(&args.hash_ids)
                ),
            )],
        )
            .into_response());
    }

    Ok((
        StatusCode::OK,
        [(header::CONTENT_TYPE, "text/html; charset=utf-8".to_string())],
        ErrorTemplate { args: &args }.render()?,
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
