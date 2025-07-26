use crate::args::ARGS;
use crate::error_handling::AppError;
use crate::util::auth;
use crate::util::hashids::to_u64 as hashid_to_u64;
use crate::util::misc::remove_expired;
use crate::util::{animalnumbers::to_u64, misc::decrypt_file};
use crate::AppState;
use axum::extract::{Multipart, Path, State};
use axum::response::{IntoResponse, Response};
use reqwest::header;
use reqwest::StatusCode;
use std::fs::File;
use std::path::PathBuf;
use tokio_util::io::ReaderStream;

pub async fn post_secure_file(
    State(data): State<AppState>,
    Path(id): Path<String>,
    payload: Multipart,
) -> Result<Response, AppError> {
    // get access to the pasta collection

    let id = if ARGS.hash_ids {
        hashid_to_u64(&id).unwrap_or(0)
    } else {
        to_u64(&id).unwrap_or(0)
    };

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
        // find the index of the pasta in the collection based on u64 id
        for (i, pasta) in pastas.iter().enumerate() {
            if pasta.id == id {
                index = i;
                found = true;
                break;
            }
        }
    }

    let password = auth::password_from_multipart(payload).await?;

    {
        let pastas = data.pastas.lock().unwrap();
        if found {
            if let Some(ref pasta_file) = pastas[index].file {
                let file = File::open(format!(
                    "{}/attachments/{}/data.enc",
                    ARGS.data_dir,
                    pastas[index].id_as_animals()
                ))?;

                // Not compatible with NamedFile from actix_files (it needs a File
                // to work therefore secure files do not support streaming
                let decrypted_data: Vec<u8> = decrypt_file(&password, &file)?;

                // Set the content type based on the file extension
                let content_type = mime_guess::from_path(&pasta_file.name)
                    .first_or_octet_stream()
                    .to_string();

                // Create a response with the decrypted data
                let response = Response::builder()
                    .status(200)
                    .header("Content-Type", content_type)
                    .header(
                        "Content-Disposition",
                        format!("attachment; filename=\"{}\"", pasta_file.name()),
                    )
                    .body(decrypted_data.into())
                    .unwrap();
                return Ok(response);
            }
        }
    }
    Ok((StatusCode::NOT_FOUND).into_response())
}

pub async fn get_file(
    Path(id): Path<String>,
    State(data): State<AppState>,
) -> Result<axum::response::Response, AppError> {
    let id_intern = if ARGS.hash_ids {
        hashid_to_u64(&id).unwrap_or(0)
    } else {
        to_u64(&id).unwrap_or(0)
    };

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
            if pasta.id == id_intern {
                index = i;
                found = true;
                break;
            }
        }
    }

    let pastas = { data.pastas.lock().unwrap().clone() };
    if found {
        if let Some(ref pasta_file) = pastas[index].file {
            if pastas[index].encrypt_server {
                return Ok((
                    StatusCode::FOUND,
                    [(
                        header::LOCATION,
                        format!("/auth_file/{}", pastas[index].id_as_animals()),
                    )],
                )
                    .into_response());
            }

            // Construct the path to the file
            let file_path = format!(
                "{}/attachments/{}/{}",
                ARGS.data_dir,
                pastas[index].id_as_animals(),
                pasta_file.name()
            );
            let file_path = PathBuf::from(file_path);

            // This will stream the file and set the content type based on the
            // file path
            let file = tokio::fs::File::open(&file_path).await?;
            let stream = ReaderStream::new(file);
            let body = axum::body::Body::from_stream(stream);
            let content_disposition = format!("attachment; filename=\"{}\"", pasta_file.name());
            let response = Response::builder()
                .status(StatusCode::OK)
                .header(
                    header::CONTENT_TYPE,
                    mime_guess::from_path(&file_path)
                        .first_or_octet_stream()
                        .as_ref(),
                )
                .header(header::CONTENT_DISPOSITION, &content_disposition)
                .body(body)
                .unwrap();
            // This takes care of streaming/seeking using the Range
            // header in the request.
            return Ok(response.into_response());
        }
    }

    Ok((StatusCode::NOT_FOUND).into_response())
}

pub fn files_router() -> axum::Router<AppState> {
    axum::Router::new()
        .route("/secure_file/{id}", axum::routing::post(post_secure_file))
        .route("/file/{id}", axum::routing::get(get_file))
}
