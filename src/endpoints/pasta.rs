use crate::AppState;
use crate::args::Args;
use crate::endpoints::errors::ErrorTemplate;
use crate::error_handling::AppError;
use crate::pasta::Pasta;
use crate::util::animalnumbers::to_u64;
use crate::util::auth;
use crate::util::hashids::to_u64 as hashid_to_u64;
use askama::Template;
use axum::Router;
use axum::extract::{Multipart, Path, State};
use axum::http::{HeaderMap, StatusCode, header};
use axum::response::IntoResponse;
use axum::routing::{get, post};
use magic_crypt::{MagicCryptTrait, new_magic_crypt};
use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Template)]
#[template(path = "upload.html", escape = "none")]
struct PastaTemplate<'a> {
    pasta: &'a Pasta,
    args: &'a Args,
}

fn pastaresponse(
    AppState { args, db }: AppState,
    id: String,
    password: String,
) -> Result<impl IntoResponse, AppError> {
    // get access to the pasta collection

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
            ));
        }
    }?;

    if pasta.encrypt_server && password == *"" {
        return Ok((
            StatusCode::FOUND,
            [(
                header::LOCATION,
                format!(
                    "{}/auth/{}",
                    args.public_path_as_str(),
                    pasta.id_as_animals(&args.hash_ids)
                ),
            )],
            "".to_string(),
        ));
    }

    // increment read count
    pasta.read_count += 1;

    // save the updated read count
    db.update_pasta(&id, pasta.clone().into())?;
    let original_content = pasta.content.to_owned();

    // decrypt content temporarily
    if password != *"" && !original_content.is_empty() {
        let res = decrypt(&original_content, &password);
        if let Ok(rs) = res {
            pasta.content.replace_range(.., rs.as_str());
        } else {
            return Ok((
                StatusCode::FOUND,
                [(
                    header::LOCATION,
                    format!(
                        "{}/auth/{}/incorrect",
                        args.public_path_as_str(),
                        pasta.id_as_animals(&args.hash_ids)
                    ),
                )],
                "".to_string(),
            ));
        }
    }

    // serve pasta in template
    let pasta_template = PastaTemplate {
        pasta: &pasta,
        args: &args,
    }
    .render()?;
    let response = (
        StatusCode::OK,
        [(header::CONTENT_TYPE, "text/html charset=utf-8".to_string())],
        pasta_template,
    );

    if pasta.content != original_content {
        pasta.content = original_content;
    }

    // get current unix time in seconds
    let timenow: i64 = match SystemTime::now().duration_since(UNIX_EPOCH) {
        Ok(n) => n.as_secs(),
        Err(_) => {
            log::error!("SystemTime before UNIX EPOCH!");
            0
        }
    } as i64;

    // update last read time
    pasta.last_read = timenow;

    // save the updated read count
    db.update_pasta(&id, pasta.clone().into())?;
    Ok(response)
}

pub async fn postpasta(
    State(data): State<AppState>,
    Path(id): Path<String>,
    payload: Multipart,
) -> Result<impl IntoResponse, AppError> {
    let password = auth::password_from_multipart(payload).await?;
    Ok(pastaresponse(data, id, password))
}

pub async fn postshortpasta(
    State(data): State<AppState>,
    Path(id): Path<String>,
    payload: Multipart,
) -> Result<impl IntoResponse, AppError> {
    let password = auth::password_from_multipart(payload).await?;
    Ok(pastaresponse(data, id, password))
}

pub async fn getpasta(State(data): State<AppState>, Path(id): Path<String>) -> impl IntoResponse {
    pastaresponse(data, id, String::from(""))
}

pub async fn getshortpasta(
    State(data): State<AppState>,
    Path(id): Path<String>,
) -> impl IntoResponse {
    pastaresponse(data, id, String::from(""))
}

fn urlresponse(AppState { args, db }: AppState, id: String) -> Result<impl IntoResponse, AppError> {
    // get access to the pasta collection

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
            ));
        }
    }?;
    // increment read count
    pasta.read_count += 1;

    // save the updated read count
    db.update_pasta(&id, pasta.clone().into())?;
    // send redirect if it's a url pasta
    if pasta.pasta_type == "url" {
        let response = (
            StatusCode::FOUND,
            [(header::LOCATION, pasta.content.to_string())],
            "".to_string(),
        );

        // get current unix time in seconds
        let timenow: i64 = match SystemTime::now().duration_since(UNIX_EPOCH) {
            Ok(n) => n.as_secs(),
            Err(_) => {
                log::error!("SystemTime before UNIX EPOCH!");
                0
            }
        } as i64;

        // update last read time
        pasta.last_read = timenow;

        // save the updated read count
        db.update_pasta(&id, pasta.clone().into())?;
        Ok(response)
    // send error if we're trying to open a non-url pasta as a redirect
    } else {
        let response = (
            StatusCode::OK,
            [(header::CONTENT_TYPE, "text/html; charset=utf-8".to_string())],
            ErrorTemplate { args: &args }.render()?,
        );
        Ok(response)
    }
}

pub async fn redirecturl(
    State(data): State<AppState>,
    Path(id): Path<String>,
) -> impl IntoResponse {
    urlresponse(data, id)
}

pub async fn shortredirecturl(
    State(data): State<AppState>,
    Path(id): Path<String>,
) -> impl IntoResponse {
    urlresponse(data, id)
}

pub async fn getrawpasta(
    State(AppState { args, db }): State<AppState>,
    Path(id): Path<String>,
) -> Result<impl IntoResponse, AppError> {
    // get access to the pasta collection

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
            ));
        }
    }?;
    if pasta.encrypt_server {
        return Ok((
            StatusCode::FOUND,
            [(
                header::LOCATION,
                format!(
                    "{}/auth_raw/{}",
                    args.public_path_as_str(),
                    pasta.id_as_animals(&args.hash_ids)
                ),
            )],
            "".to_string(),
        ));
    }

    // increment read count
    pasta.read_count += 1;

    // save the updated read count
    db.update_pasta(&id, pasta.clone().into())?;

    // get current unix time in seconds
    let timenow: i64 = match SystemTime::now().duration_since(UNIX_EPOCH) {
        Ok(n) => n.as_secs(),
        Err(_) => {
            log::error!("SystemTime before UNIX EPOCH!");
            0
        }
    } as i64;

    // update last read time
    pasta.last_read = timenow;

    // send raw content of pasta
    let selected_pasta = pasta.content.to_owned();

    let response = (
        StatusCode::OK,
        [(
            header::CONTENT_TYPE,
            "text/plain;  \
        charset=utf-8"
                .to_string(),
        )],
        selected_pasta,
    );

    Ok(response)
}

pub async fn postrawpasta(
    State(AppState { args, db }): State<AppState>,
    Path(id): Path<String>,
    payload: Multipart,
) -> Result<impl IntoResponse, AppError> {
    let password = auth::password_from_multipart(payload).await?;

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
            ));
        }
    }?;

    if pasta.encrypt_server && password == *"" {
        let mut headers = HeaderMap::new();
        headers.insert(
            "Location",
            format!(
                "{}/auth/{}",
                args.public_path_as_str(),
                pasta.id_as_animals(&args.hash_ids)
            )
            .parse()?,
        );
        return Ok((
            StatusCode::FOUND,
            [(
                header::LOCATION,
                format!(
                    "{}/auth/{}",
                    args.public_path_as_str(),
                    pasta.id_as_animals(&args.hash_ids)
                ),
            )],
            "".to_string(),
        ));
    }

    // increment read count
    pasta.read_count += 1;

    // save the updated read count
    db.update_pasta(&id, pasta.clone().into())?;

    let original_content = pasta.content.to_owned();

    // decrypt content temporarily
    if password != *"" {
        let res = decrypt(&original_content, &password);
        if let Ok(rs) = res {
            pasta.content.replace_range(.., rs.as_str());
        } else {
            return Ok((
                StatusCode::FOUND,
                [(
                    header::LOCATION,
                    format!(
                        "{}/auth/{}/incorrect",
                        args.public_path_as_str(),
                        pasta.id_as_animals(&args.hash_ids)
                    ),
                )],
                "".to_string(),
            ));
        }
    }

    // get current unix time in seconds
    let timenow: i64 = match SystemTime::now().duration_since(UNIX_EPOCH) {
        Ok(n) => n.as_secs(),
        Err(_) => {
            log::error!("SystemTime before UNIX EPOCH!");
            0
        }
    } as i64;

    // update last read time
    pasta.last_read = timenow;

    // save the updated read count
    db.update_pasta(&id, pasta.clone().into())?;

    // send raw content of pasta

    let mut headers = HeaderMap::new();
    headers.insert("content-type", "text/html; charset=utf-8".parse()?);
    let response = (
        StatusCode::NOT_FOUND,
        [(header::CONTENT_TYPE, "text/html; charset=utf-8".to_string())],
        pasta.content.to_owned(),
    );

    if pasta.content != original_content {
        pasta.content = original_content;
    }
    Ok(response)
}

fn decrypt(text_str: &str, key_str: &str) -> Result<String, magic_crypt::MagicCryptError> {
    let mc = new_magic_crypt!(key_str, 256);

    mc.decrypt_base64_to_string(text_str)
}

pub fn pasta_routes() -> Router<AppState> {
    Router::new()
        .route("/raw/{id}", post(postrawpasta))
        .route("/raw/{id}", get(getrawpasta))
        .route("/u/{id}", get(shortredirecturl))
        .route("/url/{id}", get(redirecturl))
        .route("/p/{id}", get(getshortpasta))
        .route("/p/{id}", post(postshortpasta))
        .route("/upload/{id}", get(getpasta))
        .route("/upload/{id}", post(postpasta))
}
