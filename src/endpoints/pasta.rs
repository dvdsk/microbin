use crate::args::{Args, ARGS};
use crate::endpoints::errors::ErrorTemplate;
use crate::error_handling::AppError;
use crate::pasta::Pasta;
use crate::util::animalnumbers::to_u64;
use crate::util::auth;
use crate::util::db::update;
use crate::util::hashids::to_u64 as hashid_to_u64;
use crate::util::misc::remove_expired;
use crate::AppState;
use askama::Template;
use axum::extract::{Multipart, Path, State};
use axum::http::{header, HeaderMap, HeaderName, StatusCode};
use axum::response::IntoResponse;
use axum::routing::{get, post};
use axum::Router;
use magic_crypt::{new_magic_crypt, MagicCryptTrait};
use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Template)]
#[template(path = "upload.html", escape = "none")]
struct PastaTemplate<'a> {
    pasta: &'a Pasta,
    args: &'a Args,
}

fn pastaresponse(
    data: AppState,
    id: String,
    password: String,
) -> (StatusCode, [(HeaderName, String); 1], String) {
    // get access to the pasta collection
    let mut pastas = data.pastas.lock().unwrap();

    let id = if ARGS.hash_ids {
        hashid_to_u64(&id).unwrap_or(0)
    } else {
        to_u64(&id).unwrap_or(0)
    };

    // remove expired pastas (including this one if needed)
    remove_expired(&mut pastas);

    // find the index of the pasta in the collection based on u64 id
    let mut index: usize = 0;
    let mut found: bool = false;
    for (i, pasta) in pastas.iter().enumerate() {
        if pasta.id == id {
            index = i;
            found = true;
            break;
        }
    }

    if found {
        if pastas[index].encrypt_server && password == *"" {
            return (
                StatusCode::FOUND,
                [(
                    header::LOCATION,
                    format!(
                        "{}/auth/{}",
                        ARGS.public_path_as_str(),
                        pastas[index].id_as_animals()
                    ),
                )],
                "".to_string(),
            );
        }

        // increment read count
        pastas[index].read_count += 1;

        // save the updated read count
        update(Some(&pastas), Some(&pastas[index]));

        let original_content = pastas[index].content.to_owned();

        // decrypt content temporarily
        if password != *"" && !original_content.is_empty() {
            let res = decrypt(&original_content, &password);
            if let Ok(rs) = res {
                pastas[index].content.replace_range(.., rs.as_str());
            } else {
                return (
                    StatusCode::FOUND,
                    [(
                        header::LOCATION,
                        format!(
                            "{}/auth/{}/incorrect",
                            ARGS.public_path_as_str(),
                            pastas[index].id_as_animals()
                        ),
                    )],
                    "".to_string(),
                );
            }
        }

        // serve pasta in template
        let pasta_template = PastaTemplate {
            pasta: &pastas[index],
            args: &ARGS,
        }
        .render()
        .unwrap();
        let response = (
            StatusCode::OK,
            [(header::CONTENT_TYPE, "text/html charset=utf-8".to_string())],
            pasta_template,
        );

        if pastas[index].content != original_content {
            pastas[index].content = original_content;
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
        pastas[index].last_read = timenow;

        // save the updated read count
        update(Some(&pastas), Some(&pastas[index]));

        return response;
    }

    // otherwise, send pasta not found error
    (
        StatusCode::OK,
        [(header::CONTENT_TYPE, "text/html; charset=utf-8".to_string())],
        ErrorTemplate { args: &ARGS }.render().unwrap(),
    )
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
) -> (StatusCode, [(HeaderName, String); 1], String) {
    pastaresponse(data, id, String::from(""))
}

fn urlresponse(data: AppState, id: String) -> (StatusCode, [(HeaderName, String); 1], String) {
    // get access to the pasta collection
    let mut pastas = data.pastas.lock().unwrap();

    let id = if ARGS.hash_ids {
        hashid_to_u64(&id).unwrap_or(0)
    } else {
        to_u64(&id).unwrap_or(0)
    };

    // remove expired pastas (including this one if needed)
    remove_expired(&mut pastas);

    // find the index of the pasta in the collection based on u64 id
    let mut index: usize = 0;
    let mut found: bool = false;

    for (i, pasta) in pastas.iter().enumerate() {
        if pasta.id == id {
            index = i;
            found = true;
            break;
        }
    }

    if found {
        // increment read count
        pastas[index].read_count += 1;

        // save the updated read count
        update(Some(&pastas), Some(&pastas[index]));

        // send redirect if it's a url pasta
        if pastas[index].pasta_type == "url" {
            let response = (
                StatusCode::FOUND,
                [(header::LOCATION, pastas[index].content.to_string())],
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
            pastas[index].last_read = timenow;

            // save the updated read count
            update(Some(&pastas), Some(&pastas[index]));

            return response;
        // send error if we're trying to open a non-url pasta as a redirect
        } else {
            let response = (
                StatusCode::OK,
                [(header::CONTENT_TYPE, "text/html; charset=utf-8".to_string())],
                ErrorTemplate { args: &ARGS }.render().unwrap(),
            );
            return response;
        }
    }

    (
        StatusCode::OK,
        [(header::CONTENT_TYPE, "text/html; charset=utf-8".to_string())],
        ErrorTemplate { args: &ARGS }.render().unwrap(),
    )
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
    State(data): State<AppState>,
    Path(id): Path<String>,
) -> Result<(StatusCode, [(HeaderName, String); 1], String), AppError> {
    // get access to the pasta collection
    let mut pastas = data.pastas.lock().unwrap();

    let id = if ARGS.hash_ids {
        hashid_to_u64(&id).unwrap_or(0)
    } else {
        to_u64(&id).unwrap_or(0)
    };

    // remove expired pastas (including this one if needed)
    remove_expired(&mut pastas);

    // find the index of the pasta in the collection based on u64 id
    let mut index: usize = 0;
    let mut found: bool = false;
    for (i, pasta) in pastas.iter().enumerate() {
        if pasta.id == id {
            index = i;
            found = true;
            break;
        }
    }

    if found {
        if pastas[index].encrypt_server {
            return Ok((
                StatusCode::FOUND,
                [(
                    header::LOCATION,
                    format!(
                        "{}/auth_raw/{}",
                        ARGS.public_path_as_str(),
                        pastas[index].id_as_animals()
                    ),
                )],
                "".to_string(),
            ));
        }

        // increment read count
        pastas[index].read_count += 1;

        // save the updated read count
        update(Some(&pastas), Some(&pastas[index]));

        // get current unix time in seconds
        let timenow: i64 = match SystemTime::now().duration_since(UNIX_EPOCH) {
            Ok(n) => n.as_secs(),
            Err(_) => {
                log::error!("SystemTime before UNIX EPOCH!");
                0
            }
        } as i64;

        // update last read time
        pastas[index].last_read = timenow;

        // send raw content of pasta
        let selected_pasta = pastas[index].content.to_owned();

        let response = (
            StatusCode::NOT_FOUND,
            [(
                header::CONTENT_TYPE,
                "text/plain;  \
        charset=utf-8"
                    .to_string(),
            )],
            selected_pasta,
        );

        return Ok(response);
    }

    // otherwise send pasta not found error as raw text
    log::warn!("Pasta with id {} not found!", id);
    Ok((
        StatusCode::NOT_FOUND,
        [(header::CONTENT_TYPE, "text/html; charset=utf-8".to_string())],
        "Upload not\
     found! :-("
            .to_string(),
    ))
}

pub async fn postrawpasta(
    data: State<AppState>,
    Path(id): Path<String>,
    payload: Multipart,
) -> Result<(StatusCode, [(HeaderName, String); 1], String), AppError> {
    let password = auth::password_from_multipart(payload).await?;

    // get access to the pasta collection
    let mut pastas = data.pastas.lock().unwrap();

    let id = if ARGS.hash_ids {
        hashid_to_u64(&id).unwrap_or(0)
    } else {
        to_u64(&id).unwrap_or(0)
    };

    // remove expired pastas (including this one if needed)
    remove_expired(&mut pastas);

    // find the index of the pasta in the collection based on u64 id
    let mut index: usize = 0;
    let mut found: bool = false;
    for (i, pasta) in pastas.iter().enumerate() {
        if pasta.id == id {
            index = i;
            found = true;
            break;
        }
    }

    if found {
        if pastas[index].encrypt_server && password == *"" {
            let mut headers = HeaderMap::new();
            headers.insert(
                "Location",
                format!(
                    "{}/auth/{}",
                    ARGS.public_path_as_str(),
                    pastas[index].id_as_animals()
                )
                .parse()
                .unwrap(),
            );
            return Ok((
                StatusCode::FOUND,
                [(
                    header::LOCATION,
                    format!(
                        "{}/auth/{}",
                        ARGS.public_path_as_str(),
                        pastas[index].id_as_animals()
                    ),
                )],
                "".to_string(),
            ));
        }

        // increment read count
        pastas[index].read_count += 1;

        // save the updated read count
        update(Some(&pastas), Some(&pastas[index]));

        let original_content = pastas[index].content.to_owned();

        // decrypt content temporarily
        if password != *"" {
            let res = decrypt(&original_content, &password);
            if let Ok(rs) = res {
                pastas[index].content.replace_range(.., rs.as_str());
            } else {
                return Ok((
                    StatusCode::FOUND,
                    [(
                        header::LOCATION,
                        format!(
                            "{}/auth/{}/incorrect",
                            ARGS.public_path_as_str(),
                            pastas[index].id_as_animals()
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
        pastas[index].last_read = timenow;

        // save the updated read count
        update(Some(&pastas), Some(&pastas[index]));

        // send raw content of pasta

        let mut headers = HeaderMap::new();
        headers.insert("content-type", "text/html; charset=utf-8".parse().unwrap());
        let response = (
            StatusCode::NOT_FOUND,
            [(header::CONTENT_TYPE, "text/html; charset=utf-8".to_string())],
            pastas[index].content.to_owned(),
        );

        if pastas[index].content != original_content {
            pastas[index].content = original_content;
        }
        return Ok(response);
    }

    // otherwise send pasta not found error as raw text
    Ok((
        StatusCode::NOT_FOUND,
        [(header::CONTENT_TYPE, "text/html; charset=utf-8".to_string())],
        "Upload not found! :-(".to_string(),
    ))
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
