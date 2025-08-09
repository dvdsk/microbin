use crate::AppState;
use crate::error_handling::AppError;
use crate::pasta::{Pasta, PastaFile};
use crate::util::auth;
use crate::util::hashids::to_u64 as hashid_to_u64;
use crate::util::misc::clean_up_expired_pastes;
use crate::util::{animalnumbers::to_u64, misc::decrypt_file};
use axum::extract::{Multipart, Path, State};
use axum::response::{IntoResponse, Response};
use db::entities::pasta::PastaEntity;
use reqwest::StatusCode;
use reqwest::header;
use std::fs::File;
use std::path::PathBuf;
use tokio_util::io::ReaderStream;

pub async fn post_secure_file(
    State(AppState { args, db }): State<AppState>,
    Path(id): Path<String>,
    payload: Multipart,
) -> Result<Response, AppError> {
    // get access to the pasta collection

    let password = auth::password_from_multipart(payload).await?;

    let id = if args.hash_ids {
        hashid_to_u64(&id).unwrap_or(0)
    } else {
        to_u64(&id).unwrap_or(0)
    };

    let pasta_entity = match db.get_pasta(&id)? {
        Some(pasta) => Ok::<PastaEntity, AppError>(pasta),
        None => {
            return Ok(StatusCode::NOT_FOUND.into_response());
        }
    }?;

    let pasta: Pasta = pasta_entity.into();

    let pasta_file = match pasta.file {
        Some(ref pasta_file) => Ok::<&PastaFile, AppError>(pasta_file),
        None => {
            return Ok((StatusCode::NOT_FOUND).into_response());
        }
    }?;

    let file = File::open(format!(
        "{}/attachments/{}/enc",
        &args.data_dir,
        pasta.id_as_animals(&args.hash_ids)
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
        .status(StatusCode::OK)
        .header("Content-Type", content_type)
        .header(
            "Content-Disposition",
            format!("attachment; filename=\"{}\"", pasta_file.name()),
        )
        .body(decrypted_data.into())?;
    Ok(response)
}

pub async fn get_file(
    Path(id): Path<String>,
    State(AppState { args, db }): State<AppState>,
) -> Result<Response, AppError> {
    let id_intern = if args.hash_ids {
        hashid_to_u64(&id).unwrap_or(0)
    } else {
        to_u64(&id).unwrap_or(0)
    };

    let pasta_entity = match db.get_pasta(&id_intern)? {
        Some(pasta) => Ok::<PastaEntity, AppError>(pasta),
        None => {
            return Ok((StatusCode::NOT_FOUND).into_response());
        }
    }?;

    let pasta: Pasta = pasta_entity.into();

    let pasta_file = match pasta.file {
        Some(ref pasta_file) => Ok::<&PastaFile, AppError>(pasta_file),
        None => {
            return Ok(StatusCode::NOT_FOUND.into_response());
        }
    }?;

    if pasta.encrypt_server {
        return Ok((
            StatusCode::FOUND,
            [(
                header::LOCATION,
                format!("/auth_file/{}", pasta.id_as_animals(&args.hash_ids)),
            )],
        )
            .into_response());
    }

        // Construct the path to the file
        let file_path = format!(
            "{}/attachments/{}/{}",
            &args.data_dir,
            pasta.id_as_animals(&args.hash_ids),
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
            .body(body)?;
        // This takes care of streaming/seeking using the Range
        // header in the request.
        return Ok(response.into_response());
}

pub fn files_router() -> axum::Router<AppState> {
    axum::Router::new()
        .route("/secure_file/{id}", axum::routing::post(post_secure_file))
        .route("/file/{id}", axum::routing::get(get_file))
}
