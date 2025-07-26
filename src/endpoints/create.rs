use crate::args::Args;
use crate::error_handling::AppError;
use crate::pasta::PastaFile;
use crate::util::animalnumbers::to_animal_names;
use crate::util::db::insert;
use crate::util::hashids::to_hashids;
use crate::util::misc::{encrypt, encrypt_file, is_valid_url};
use crate::{AppState, Pasta, ARGS};
use askama::Template;
use axum::extract::{Multipart, Path, State};
use axum::http::Response;
use axum::response::IntoResponse;
use axum::routing::{get, post};
use axum::Router;
use bytesize::ByteSize;
use futures::TryStreamExt;
use log::warn;
use std::sync::LazyLock;
use std::time::{SystemTime, UNIX_EPOCH};
use tokio::io::AsyncWriteExt;

#[derive(Template)]
#[template(path = "index.html")]
struct IndexTemplate<'a> {
    args: &'a LazyLock<Args>,
    status: String,
}

pub async fn index() -> impl IntoResponse {
    let index_html = IndexTemplate {
        args: &ARGS,
        status: String::from(""),
    }
    .render()
    .unwrap();

    Response::builder()
        .status(200)
        .header("content-type", "text/html; charset=utf-8")
        .body(index_html)
        .unwrap()
}

pub async fn index_with_status(Path(status): Path<String>) -> impl IntoResponse {
    let index_with_status = IndexTemplate {
        args: &ARGS,
        status,
    }
    .render()
    .unwrap();

    Response::builder()
        .status(200)
        .header("content-type", "text/html; charset=utf-8")
        .body(index_with_status)
        .unwrap()
}

pub fn expiration_to_timestamp(expiration: &str, timenow: i64) -> i64 {
    match expiration {
        "1min" => timenow + 60,
        "10min" => timenow + 60 * 10,
        "1hour" => timenow + 60 * 60,
        "24hour" => timenow + 60 * 60 * 24,
        "3days" => timenow + 60 * 60 * 24 * 3,
        "1week" => timenow + 60 * 60 * 24 * 7,
        "never" => {
            if ARGS.eternal_pasta {
                0
            } else {
                timenow + 60 * 60 * 24 * 7
            }
        }
        _ => {
            log::error!("{}", "Unexpected expiration time!");
            timenow + 60 * 60 * 24 * 7
        }
    }
}

/// Receives a file through http Post on url /upload/a-b-c with a, b and c
/// different animals. The client sends the post in response to a form.
// TODO: form field order might need to be changed. In my testing the attachment
// data is nestled between password encryption key etc <21-10-24, dvdsk>
#[axum::debug_handler]
pub async fn create(
    data: State<AppState>,
    mut payload: Multipart,
) -> Result<Response<String>, AppError> {
    let timenow: i64 = match SystemTime::now().duration_since(UNIX_EPOCH) {
        Ok(n) => n.as_secs(),
        Err(_) => {
            log::error!("SystemTime before UNIX EPOCH!");
            0
        }
    } as i64;

    let mut new_pasta = Pasta {
        id: rand::random::<u16>() as u64,
        content: String::from(""),
        file: None,
        extension: String::from(""),
        private: false,
        readonly: false,
        editable: ARGS.editable,
        hide_read_count: false,
        encrypt_server: false,
        encrypted_key: Some(String::from("")),
        encrypt_client: false,
        created: timenow,
        read_count: 0,
        burn_after_reads: 0,
        last_read: timenow,
        pasta_type: String::from(""),
        expiration: expiration_to_timestamp(&ARGS.default_expiry, timenow),
    };

    let mut random_key: String = String::from("");
    let mut plain_key: String = String::from("");
    let mut uploader_password = String::from("");

    while let Some(mut field) = payload.next_field().await? {
        let Some(field_name) = field.name() else {
            continue;
        };
        match field_name {
            "uploader_password" => {
                while let Some(chunk) = field.try_next().await? {
                    uploader_password
                        .push_str(std::str::from_utf8(&chunk).unwrap().to_string().as_str());
                }
                continue;
            }
            "random_key" => {
                while let Some(chunk) = field.try_next().await? {
                    random_key = std::str::from_utf8(&chunk).unwrap().to_string();
                }
                continue;
            }
            "privacy" => {
                while let Some(chunk) = field.try_next().await? {
                    let privacy = std::str::from_utf8(&chunk).unwrap();
                    new_pasta.private = !matches!(privacy, "public");
                    new_pasta.readonly = matches!(privacy, "readonly");
                    new_pasta.encrypt_client = matches!(privacy, "secret");
                    new_pasta.encrypt_server = matches!(privacy, "private" | "secret")
                }
            }
            "plain_key" => {
                while let Some(chunk) = field.try_next().await? {
                    plain_key = std::str::from_utf8(&chunk).unwrap().to_string();
                }
                continue;
            }
            "encrypted_random_key" => {
                while let Some(chunk) = field.try_next().await? {
                    new_pasta.encrypted_key =
                        Some(std::str::from_utf8(&chunk).unwrap().to_string());
                }
                continue;
            }
            "hide_read_count" => {
                new_pasta.hide_read_count = true;
                continue;
            }
            "expiration" => {
                while let Some(chunk) = field.try_next().await? {
                    new_pasta.expiration =
                        expiration_to_timestamp(std::str::from_utf8(&chunk).unwrap(), timenow);
                }

                continue;
            }
            "burn_after" => {
                while let Some(chunk) = field.try_next().await? {
                    new_pasta.burn_after_reads = match std::str::from_utf8(&chunk).unwrap() {
                        // give an extra read because the user will be
                        // redirected to the pasta page automatically
                        "1" => 2,
                        "10" => 10,
                        "100" => 100,
                        "1000" => 1000,
                        "10000" => 10000,
                        "0" => 0,
                        _ => {
                            log::error!("{}", "Unexpected burn after value!");
                            0
                        }
                    };
                }

                continue;
            }
            "content" => {
                let mut content = String::from("");
                while let Some(chunk) = field.try_next().await? {
                    content.push_str(std::str::from_utf8(&chunk).unwrap().to_string().as_str());
                }
                if !content.is_empty() {
                    new_pasta.content = content;

                    new_pasta.pasta_type = if is_valid_url(new_pasta.content.as_str()) {
                        String::from("url")
                    } else {
                        String::from("text")
                    };
                }
                continue;
            }
            "syntax_highlight" => {
                while let Some(chunk) = field.try_next().await? {
                    new_pasta.extension = std::str::from_utf8(&chunk).unwrap().to_string();
                }
                continue;
            }
            "file" => {
                if ARGS.no_file_upload {
                    continue;
                }

                let path = field.file_name();

                let path = match path {
                    Some("") => continue,
                    Some(p) => p,
                    None => continue,
                };

                let mut file = match PastaFile::from_unsanitized(path) {
                    Ok(f) => f,
                    Err(e) => {
                        warn!("Unsafe file name: {e:?}");
                        continue;
                    }
                };

                std::fs::create_dir_all(format!(
                    "{}/attachments/{}",
                    ARGS.data_dir,
                    &new_pasta.id_as_animals()
                ))
                .unwrap();

                let filepath = format!(
                    "{}/attachments/{}/{}",
                    ARGS.data_dir,
                    &new_pasta.id_as_animals(),
                    &file.name()
                );

                let mut f = tokio::fs::File::create(filepath).await?;
                let mut size = 0;
                while let Some(chunk) = field.try_next().await? {
                    size += chunk.len();
                    if (new_pasta.encrypt_server
                        && size > ARGS.max_file_size_encrypted_mb * 1024 * 1024)
                        || size > ARGS.max_file_size_unencrypted_mb * 1024 * 1024
                    {
                        let repsonse = axum::response::Response::builder()
                            .status(400)
                            .body(
                                "File \
                        exceeded \
                        size \
                        limit."
                                    .to_string(),
                            )
                            .unwrap();
                        return Ok(repsonse);
                    }
                    f.write_all(&chunk).await?;
                }

                file.size = ByteSize::b(size as u64);

                new_pasta.file = Some(file);
                new_pasta.pasta_type = String::from("text");
            }
            field => {
                log::error!("Unexpected multipart field:  {}", field);
            }
        }
    }

    let res = axum::response::Response::builder()
        .status(302)
        .header(
            "Location",
            format!("{}/incorrect", ARGS.public_path_as_str()),
        )
        .body("".to_string())
        .unwrap();

    if ARGS.readonly
        && ARGS.uploader_password.is_some()
        && uploader_password != *ARGS.uploader_password.as_ref().unwrap()
    {
        return Ok(res);
    }

    let id = new_pasta.id;

    if plain_key != *"" && new_pasta.readonly {
        new_pasta.encrypted_key = Some(encrypt(id.to_string().as_str(), &plain_key));
    }

    if new_pasta.encrypt_server && !new_pasta.readonly && new_pasta.content != *"" {
        if new_pasta.encrypt_client {
            new_pasta.content = encrypt(&new_pasta.content, &random_key);
        } else {
            new_pasta.content = encrypt(&new_pasta.content, &plain_key);
        }
    }

    if new_pasta.file.is_some() && new_pasta.encrypt_server && !new_pasta.readonly {
        let filepath = format!(
            "{}/attachments/{}/{}",
            ARGS.data_dir,
            &new_pasta.id_as_animals(),
            &new_pasta.file.as_ref().unwrap().name()
        );
        if new_pasta.encrypt_client {
            encrypt_file(&random_key, &filepath).expect("Failed to encrypt file with random key")
        } else {
            encrypt_file(&plain_key, &filepath).expect("Failed to encrypt file with plain key")
        }
    }

    let encrypt_server = new_pasta.encrypt_server;
    {
        let mut pastas = data.pastas.lock().unwrap();

        pastas.push(new_pasta);

        for pasta in pastas.iter() {
            if pasta.id == id {
                insert(Some(&pastas), Some(pasta));
            }
        }
    }

    let slug = if ARGS.hash_ids {
        to_hashids(id)
    } else {
        to_animal_names(id)
    };

    if encrypt_server {
        Ok(Response::builder()
            .status(302)
            .header("Location", format!("/auth/{slug}/success"))
            .body("".to_string())
            .unwrap())
    } else {
        Ok(Response::builder()
            .status(302)
            .header(
                "Location",
                format!("{}/upload/{}", ARGS.public_path_as_str(), slug),
            )
            .body("".to_string())
            .unwrap())
    }
}

pub fn create_routes() -> Router<AppState> {
    Router::new()
        .route("/", get(index))
        .route("/{status}", get(index_with_status))
        .route("/create", post(create))
        .route("/upload", post(create))
        .route("/create/{status}", post(create))
}
