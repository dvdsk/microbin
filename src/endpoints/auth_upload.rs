use crate::{db, AppState};
use crate::args::{Args};
use crate::endpoints::errors::ErrorTemplate;
use crate::error_handling::AppError;
use crate::util::animalnumbers::to_u64;
use crate::util::hashids::to_u64 as hashid_to_u64;
use askama::Template;
use axum::Router;
use axum::extract::{Path, State};
use axum::http::HeaderMap;
use axum::response::IntoResponse;
use axum::routing::get;

#[derive(Template)]
#[template(path = "auth_upload.html")]
struct AuthPasta<'a> {
    args: &'a Args,
    id: String,
    status: String,
    encrypted_key: String,
    encrypt_client: bool,
    path: String,
}

pub async fn auth_upload(
    State(AppState{args, db}): State<AppState>,
    Path(id): Path<String>,
) -> Result<impl IntoResponse, AppError> {
    // get access to the pasta collection
    let mut pastas = pastas.lock().expect("no microbin thread should panic");

    let intern_id = if args.hash_ids {
        hashid_to_u64(&id).unwrap_or(0)
    } else {
        to_u64(&id).unwrap_or(0)
    };

    for pasta in pastas.iter() {
        if pasta.id == intern_id {
            let mut headers = HeaderMap::new();
            headers.insert(
                "Content-Type",
                "text/html; charset=utf-8".parse().map_err(AppError::from)?,
            );
            let body = AuthPasta {
                args: &args,
                id,
                status: String::from(""),
                encrypted_key: pasta.encrypted_key.to_owned().unwrap_or_default(),
                encrypt_client: pasta.encrypt_client,
                path: String::from("upload"),
            }
            .render()?;
            return Ok((headers, body));
        }
    }

    let mut headers = HeaderMap::new();
    headers.insert("Content-Type", "text/html; charset=utf-8".parse()?);
    let body = ErrorTemplate { args: &args }.render()?;
    Ok((headers, body))
}

pub async fn auth_upload_with_status(
    State(AppState{args, db}): State<AppState>,
    Path((id, status)): Path<(String, String)>,
) -> Result<impl IntoResponse, AppError> {
    // get access to the pasta collection
    let mut pastas = pastas.lock().expect("no microbin thread should panic");

    let intern_id = if args.hash_ids {
        hashid_to_u64(&id).unwrap_or(0)
    } else {
        to_u64(&id).unwrap_or(0)
    };

    for pasta in pastas.iter() {
        if pasta.id == intern_id {
            let mut headers = HeaderMap::new();
            headers.insert("Content-Type", "text/html; charset=utf-8".parse()?);
            let body = AuthPasta {
                args: &args,
                id,
                status,
                encrypted_key: pasta.encrypted_key.to_owned().unwrap_or_default(),
                encrypt_client: pasta.encrypt_client,
                path: String::from("upload"),
            }
            .render()?;
            return Ok((headers, body));
        }
    }

    let mut headers = HeaderMap::new();
    headers.insert("Content-Type", "text/html; charset=utf-8".parse()?);
    let body = ErrorTemplate { args: &args }.render()?;
    Ok((headers, body))
}

pub async fn auth_raw_pasta(
    State(AppState{args, db}): State<AppState>,
    Path(id): Path<String>,
) -> Result<impl IntoResponse, AppError> {
    // get access to the pasta collection
    let pasta = db.get_pasta(id);

    let intern_id = if args.hash_ids {
        hashid_to_u64(&id).unwrap_or(0)
    } else {
        to_u64(&id).unwrap_or(0)
    };

    for pasta in pastas.iter() {
        if pasta.id == intern_id {
            let mut headers = HeaderMap::new();
            headers.insert("Content-Type", "text/html; charset=utf-8".parse()?);
            let body = AuthPasta {
                args: &args,
                id,
                status: String::from(""),
                encrypted_key: pasta.encrypted_key.to_owned().unwrap_or_default(),
                encrypt_client: pasta.encrypt_client,
                path: String::from("raw"),
            }
            .render()?;
            return Ok((headers, body));
        }
    }

    let mut headers = HeaderMap::new();
    headers.insert("Content-Type", "text/html; charset=utf-8".parse()?);
    let body = ErrorTemplate { args: &args }.render()?;
    Ok((headers, body))
}

pub async fn auth_raw_pasta_with_status(
    State(AppState{pastas,args, db}): State<AppState>,
    Path((id, status)): Path<(String, String)>,
) -> Result<impl IntoResponse, AppError> {
    // get access to the pasta collection
    let mut pastas = pastas.lock().expect("no microbin thread should panic");

    clean_up_expired_pastes(&mut pastas, &args);

    let intern_id = if args.hash_ids {
        hashid_to_u64(&id).unwrap_or(0)
    } else {
        to_u64(&id).unwrap_or(0)
    };

    for pasta in pastas.iter() {
        if pasta.id == intern_id {
            let mut headers = HeaderMap::new();
            headers.insert("Content-Type", "text/html; charset=utf-8".parse()?);
            let body = AuthPasta {
                args: &args,
                id,
                status,
                encrypted_key: pasta.encrypted_key.to_owned().unwrap_or_default(),
                encrypt_client: pasta.encrypt_client,
                path: String::from("raw"),
            }
            .render()?;
            return Ok((headers, body));
        }
    }
    let mut headers = HeaderMap::new();
    headers.insert("Content-Type", "text/html; charset=utf-8".parse()?);
    let body = ErrorTemplate { args: &args }.render()?;
    Ok((headers, body))
}

pub async fn auth_edit_private(
    State(AppState{pastas,args, db}): State<AppState>,
    Path(id): Path<String>,
) -> Result<impl IntoResponse, AppError> {
    // get access to the pasta collection
    let mut pastas = pastas.lock().expect("no microbin thread should panic");


    let intern_id = if args.hash_ids {
        hashid_to_u64(&id).unwrap_or(0)
    } else {
        to_u64(&id).unwrap_or(0)
    };

    for pasta in pastas.iter() {
        if pasta.id == intern_id {
            let mut headers = HeaderMap::new();
            headers.insert("Content-Type", "text/html; charset=utf-8".parse()?);
            let body = AuthPasta {
                args: &args,
                id,
                status: String::from(""),
                encrypted_key: pasta.encrypted_key.to_owned().unwrap_or_default(),
                encrypt_client: pasta.encrypt_client,
                path: String::from("edit_private"),
            }
            .render()?;
            return Ok((headers, body));
        }
    }

    let mut headers = HeaderMap::new();
    headers.insert("Content-Type", "text/html; charset=utf-8".parse()?);
    let body = ErrorTemplate { args: &args }.render()?;
    Ok((headers, body))
}

pub async fn auth_edit_private_with_status(
    State(AppState{pastas,args, db}): State<AppState>,
    Path((id, status)): Path<(String, String)>,
) -> Result<impl IntoResponse, AppError> {
    // get access to the pasta collection
    let mut pastas = pastas.lock().expect("no microbin thread should panic");


    let intern_id = if args.hash_ids {
        hashid_to_u64(&id).unwrap_or(0)
    } else {
        to_u64(&id).unwrap_or(0)
    };

    for pasta in pastas.iter() {
        if pasta.id == intern_id {
            let mut headers = HeaderMap::new();
            headers.insert("Content-Type", "text/html; charset=utf-8".parse()?);
            let body = AuthPasta {
                args: &args,
                id,
                status,
                encrypted_key: pasta.encrypted_key.to_owned().unwrap_or_default(),
                encrypt_client: pasta.encrypt_client,
                path: String::from("edit_private"),
            }
            .render()?;
            return Ok((headers, body));
        }
    }
    let mut headers = HeaderMap::new();
    headers.insert("Content-Type", "text/html; charset=utf-8".parse()?);
    let body = ErrorTemplate { args: &args }.render()?;
    Ok((headers, body))
}

pub async fn auth_file(
    State(AppState{pastas,args, db}): State<AppState>,
    Path(id): Path<String>,
) -> Result<impl IntoResponse, AppError> {
    // get access to the pasta collection
    let mut pastas = pastas.lock().expect("no microbin thread should panic");

    let intern_id = if args.hash_ids {
        hashid_to_u64(&id).unwrap_or(0)
    } else {
        to_u64(&id).unwrap_or(0)
    };

    for pasta in pastas.iter() {
        if pasta.id == intern_id {
            let mut headers = HeaderMap::new();
            headers.insert("Content-Type", "text/html; charset=utf-8".parse()?);
            let body = AuthPasta {
                args: &args,
                id,
                status: String::from(""),
                encrypted_key: pasta.encrypted_key.to_owned().unwrap_or_default(),
                encrypt_client: pasta.encrypt_client,
                path: String::from("secure_file"),
            }
            .render()?;
            return Ok((headers, body));
        }
    }

    let mut headers = HeaderMap::new();
    headers.insert("Content-Type", "text/html; charset=utf-8".parse()?);
    let body = ErrorTemplate { args: &args }.render()?;
    Ok((headers, body))
}

pub async fn auth_file_with_status(
    State(AppState{pastas,args, db}): State<AppState>,
    Path((id, status)): Path<(String, String)>,
) -> Result<impl IntoResponse, AppError> {
    // get access to the pasta collection
    let mut pastas = pastas.lock().expect("no microbin thread should panic");

    let intern_id = if args.hash_ids {
        hashid_to_u64(&id).unwrap_or(0)
    } else {
        to_u64(&id).unwrap_or(0)
    };

    for pasta in pastas.iter() {
        if pasta.id == intern_id {
            let mut headers = HeaderMap::new();
            headers.insert("Content-Type", "text/html; charset=utf-8".parse()?);
            let body = AuthPasta {
                args: &args,
                id,
                status,
                encrypted_key: pasta.encrypted_key.to_owned().unwrap_or_default(),
                encrypt_client: pasta.encrypt_client,
                path: String::from("secure_file"),
            }
            .render()?;
            return Ok((headers, body));
        }
    }

    let mut headers = HeaderMap::new();
    headers.insert("Content-Type", "text/html; charset=utf-8".parse()?);
    let body = ErrorTemplate { args: &args }.render()?;
    Ok((headers, body))
}

pub async fn auth_remove_private(
    State(AppState{pastas,args, db}): State<AppState>,
    Path(id): Path<String>,
) -> Result<impl IntoResponse, AppError> {
    // get access to the pasta collection
    let mut pastas = pastas.lock().expect("no microbin thread should panic");


    let intern_id = if args.hash_ids {
        hashid_to_u64(&id).unwrap_or(0)
    } else {
        to_u64(&id).unwrap_or(0)
    };

    for pasta in pastas.iter() {
        if pasta.id == intern_id {
            let mut headers = HeaderMap::new();
            headers.insert("Content-Type", "text/html; charset=utf-8".parse()?);
            let body = AuthPasta {
                args: &args,
                id,
                status: String::from(""),
                encrypted_key: pasta.encrypted_key.to_owned().unwrap_or_default(),
                encrypt_client: pasta.encrypt_client,
                path: String::from("remove"),
            }
            .render()?;
            return Ok((headers, body));
        }
    }

    let mut headers = HeaderMap::new();
    headers.insert("Content-Type", "text/html; charset=utf-8".parse()?);
    let body = ErrorTemplate { args: &args }.render()?;
    Ok((headers, body))
}

pub async fn auth_remove_private_with_status(
    State(AppState{pastas,args, db}): State<AppState>,
    Path((id, status)): Path<(String, String)>,
) -> Result<impl IntoResponse, AppError> {
    // get access to the pasta collection
    let mut pastas = pastas.lock()?;


    let intern_id = if args.hash_ids {
        hashid_to_u64(&id).unwrap_or(0)
    } else {
        to_u64(&id).unwrap_or(0)
    };

    for pasta in pastas.iter() {
        if pasta.id == intern_id {
            let mut headers = HeaderMap::new();
            let body = AuthPasta {
                args: &args,
                id: id.clone(),
                status: status.clone(),
                encrypted_key: pasta.encrypted_key.to_owned().unwrap_or_default(),
                encrypt_client: pasta.encrypt_client,
                path: String::from("remove"),
            }
            .render()?;
            headers.insert("Content-Type", "text/html; charset=utf-8".parse()?);
            return Ok((headers, body));
        }
    }

    let mut headers = HeaderMap::new();
    let body = ErrorTemplate { args: &args }.render()?;
    headers.insert("Content-Type", "text/html; charset=utf-8".parse()?);
    Ok((headers, body))
}

pub fn auth_upload_router() -> Router<AppState> {
    Router::new()
        .route(
            "/auth_remove_private/{id}/{status}",
            get(auth_remove_private_with_status),
        )
        .route("/auth_remove_private/{id}", get(auth_remove_private))
        .route("/auth_file/{id}/{status}", get(auth_file_with_status))
        .route("/auth_file/{id}", get(auth_file))
        .route(
            "/auth_edit_private/{id}/{status}",
            get(auth_edit_private_with_status),
        )
        .route("/auth_edit_private/{id}", get(auth_edit_private))
        .route("/auth_raw/{id}/{status}", get(auth_raw_pasta_with_status))
        .route("/auth_raw/{id}", get(auth_raw_pasta))
        .route("/auth/{id}/{status}", get(auth_upload_with_status))
        .route("/auth/{id}", get(auth_upload))
}
