use crate::AppState;
use crate::endpoints::errors::ErrorTemplate;
use crate::error_handling::AppError;
use crate::pasta::{Pasta, PastaFile};
use crate::util::animalnumbers::to_u64;
use crate::util::auth;
use crate::util::hashids::to_u64 as hashid_to_u64;
use crate::util::misc::{decrypt, remove_expired};
use askama::Template;
use axum::Router;
use axum::extract::{Multipart, Path, State};
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::routing::{get, post};
use reqwest::header;
use std::fs;

pub async fn remove(
    State(AppState { args, db }): State<AppState>,
    Path(id): Path<String>,
) -> Result<axum::response::Response, AppError> {
    let id = if args.hash_ids {
        hashid_to_u64(&id).unwrap_or(0)
    } else {
        to_u64(&id).unwrap_or(0)
    };

    let opt_pasta = db.get_pasta(&id)?;

    let mut pasta: Pasta = match opt_pasta {
        Some(pasta) => Ok::<Pasta, AppError>(pasta.into()),
        None => {
            // otherwise, send pasta not found error
            return Ok((
                StatusCode::OK,
                [(header::CONTENT_TYPE, "text/html; charset=utf-8".to_string())],
                ErrorTemplate { args: &args }.render()?,
            )
                .into_response());
        }
    }?;

    // if it's encrypted or read-only, it needs a password to be deleted
    if pasta.encrypt_server || pasta.readonly {
        return Ok((
            StatusCode::FOUND,
            [(
                header::LOCATION,
                format!(
                    "{}/auth_remove_private/{}",
                    args.public_path_as_str(),
                    pasta.id_as_animals(&args.hash_ids)
                ),
            )],
            "".to_string(),
        )
            .into_response());
    }

    // remove the file itself
    if let Some(PastaFile { name, .. }) = &pasta.file {
        if fs::remove_file(format!(
            "{}/attachments/{}/{}",
            args.data_dir,
            pasta.id_as_animals(&args.hash_ids),
            name
        ))
        .is_err()
        {
            log::error!("Failed to delete file {}!", name)
        }

        // and remove the containing directory
        if fs::remove_dir(format!(
            "{}/attachments/{}/",
            args.data_dir,
            pasta.id_as_animals(&args.hash_ids)
        ))
        .is_err()
        {
            log::error!("Failed to delete directory {}!", name)
        }
    }

    db.delete_pasta(&id)?;

    Ok((
        StatusCode::FOUND,
        [(
            header::LOCATION,
            format!("{}/list", args.public_path_as_str()),
        )],
        "".to_string(),
    )
        .into_response())
}

pub async fn post_remove(
    State(AppState { args, db }): State<AppState>,
    Path(id): Path<String>,
    payload: Multipart,
) -> Result<axum::response::Response, AppError> {
    let id = if args.hash_ids {
        hashid_to_u64(&id).unwrap_or(0)
    } else {
        to_u64(&id).unwrap_or(0)
    };

    let password = auth::password_from_multipart(payload).await;
    if password.is_err() {
        return Ok((
            StatusCode::OK,
            [(header::CONTENT_TYPE, "text/html; charset=utf-8".to_string())],
            ErrorTemplate { args: &args }.render()?,
        )
            .into_response());
    }

    let password_unwrapped = password?;

    let opt_pasta = db.get_pasta(&id)?;

    let mut pasta: Pasta = match opt_pasta {
        Some(pasta) => Ok::<Pasta, AppError>(pasta.into()),
        None => {
            // otherwise, send pasta not found error
            return Ok((
                StatusCode::OK,
                [(header::CONTENT_TYPE, "text/html; charset=utf-8".to_string())],
                ErrorTemplate { args: &args }.render()?,
            )
                .into_response());
        }
    }?;
    if pasta.readonly || pasta.encrypt_server {
        if password_unwrapped != *"" {
            let res = decrypt(pasta.content.to_owned().as_str(), &password_unwrapped);
            if res.is_ok() {
                // remove the file itself
                if let Some(PastaFile { name, .. }) = &pasta.file {
                    if fs::remove_file(format!(
                        "{}/attachments/{}/{}",
                        args.data_dir,
                        pasta.id_as_animals(&args.hash_ids),
                        name
                    ))
                    .is_err()
                    {
                        log::error!("Failed to delete file {}!", name)
                    }

                    // and remove the containing directory
                    if fs::remove_dir(format!(
                        "{}/attachments/{}/",
                        args.data_dir,
                        pasta.id_as_animals(&args.hash_ids)
                    ))
                    .is_err()
                    {
                        log::error!("Failed to delete directory {}!", name)
                    }
                }

                db.delete_pasta(&pasta.id)?;

                let res = (
                    StatusCode::FOUND,
                    [(
                        header::LOCATION,
                        format!("{}/list", args.public_path_as_str()),
                    )],
                    "".to_string(),
                )
                    .into_response();
                return Ok(res);
            } else {
                let res = (
                    StatusCode::FOUND,
                    [(
                        header::LOCATION,
                        format!(
                            "{}/auth_remove_private/{}/incorrect",
                            args.public_path_as_str(),
                            pasta.id_as_animals(&args.hash_ids)
                        ),
                    )],
                    "".to_string(),
                )
                    .into_response();
                return Ok(res);
            }
        } else {
            let res = (
                StatusCode::FOUND,
                [(
                    header::LOCATION,
                    format!(
                        "{}/auth_remove_private/{}",
                        args.public_path_as_str(),
                        pasta.id_as_animals(&args.hash_ids)
                    ),
                )],
                "".to_string(),
            )
                .into_response();
            return Ok(res);
        }
    }

    let res = (
        StatusCode::FOUND,
        [(
            header::LOCATION,
            format!(
                "{}/upload/{}",
                args.public_path_as_str(),
                pasta.id_as_animals(&args.hash_ids)
            ),
        )],
        "".to_string(),
    )
        .into_response();
    Ok(res)
}

pub fn remove_router() -> Router<AppState> {
    Router::new()
        .route("/remove/{id}", post(post_remove))
        .route("/remove/{id}", get(remove))
}
