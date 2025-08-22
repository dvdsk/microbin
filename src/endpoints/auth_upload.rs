use crate::AppState;
use askama::Template;
use axum::Router;
use axum::extract::{Path, State};
use axum::http::HeaderMap;
use axum::response::IntoResponse;
use axum::routing::get;
use db::entities::pasta::PastaEntity;
use microbin_frontend::components::error::Error;
use models::args::Args;
use models::error_handling::AppError;
use models::pasta::Pasta;
use models::util::animalnumbers::to_u64;
use models::util::hashids::to_u64_hash_ids;

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
    State(AppState { args, db }): State<AppState>,
    Path(id): Path<String>,
) -> Result<impl IntoResponse, AppError> {
    // get access to the pasta collection

    let intern_id = if args.hash_ids {
        to_u64_hash_ids(&id).unwrap_or(0)
    } else {
        to_u64(&id).unwrap_or(0)
    };

    let pasta = db.get_pasta(&intern_id)?;

    let pasta: Pasta = match pasta {
        Some(pasta) => Ok::<Pasta, AppError>(pasta.into()),
        None => {
            let mut headers = HeaderMap::new();
            headers.insert("Content-Type", "text/html; charset=utf-8".parse()?);
            let error = dioxus_ssr::render_element(Error(args.into()));
            return Ok((headers, error));
        }
    }?;

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
    Ok((headers, body))
}

pub async fn auth_upload_with_status(
    State(AppState { args, db }): State<AppState>,
    Path((id, status)): Path<(String, String)>,
) -> Result<impl IntoResponse, AppError> {
    let intern_id = if args.hash_ids {
        to_u64_hash_ids(&id).unwrap_or(0)
    } else {
        to_u64(&id).unwrap_or(0)
    };

    let pasta = db.get_pasta(&intern_id)?;

    let pasta: Pasta = match pasta {
        Some(pasta) => Ok::<Pasta, AppError>(pasta.into()),
        None => {
            let mut headers = HeaderMap::new();
            headers.insert("Content-Type", "text/html; charset=utf-8".parse()?);
            let error = dioxus_ssr::render_element(Error(args.into()));
            return Ok((headers, error));
        }
    }?;

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
    Ok((headers, body))
}

pub async fn auth_raw_pasta(
    State(AppState { args, db }): State<AppState>,
    Path(id): Path<String>,
) -> Result<impl IntoResponse, AppError> {
    // get access to the pasta collection

    let intern_id = if args.hash_ids {
        to_u64_hash_ids(&id).unwrap_or(0)
    } else {
        to_u64(&id).unwrap_or(0)
    };

    let pasta_entity = match db.get_pasta(&intern_id)? {
        Some(pasta) => Ok::<PastaEntity, AppError>(pasta),
        None => {
            let mut headers = HeaderMap::new();
            headers.insert("Content-Type", "text/html; charset=utf-8".parse()?);
            let error = dioxus_ssr::render_element(Error(args.into()));
            return Ok((headers, error));
        }
    }?;

    let pasta: Pasta = pasta_entity.into();
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
    Ok((headers, body))
}

pub async fn auth_raw_pasta_with_status(
    State(AppState { args, db }): State<AppState>,
    Path((id, status)): Path<(String, String)>,
) -> Result<impl IntoResponse, AppError> {
    // get access to the pasta collection

    let intern_id = if args.hash_ids {
        to_u64_hash_ids(&id).unwrap_or(0)
    } else {
        to_u64(&id).unwrap_or(0)
    };

    let pasta_entity = match db.get_pasta(&intern_id)? {
        Some(pasta) => Ok::<PastaEntity, AppError>(pasta),
        None => {
            let mut headers = HeaderMap::new();
            headers.insert("Content-Type", "text/html; charset=utf-8".parse()?);
            let error = dioxus_ssr::render_element(Error(args.into()));
            return Ok((headers, error));
        }
    }?;

    let pasta: Pasta = pasta_entity.into();
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
    Ok((headers, body))
}

pub async fn auth_edit_private(
    State(AppState { args, db }): State<AppState>,
    Path(id): Path<String>,
) -> Result<impl IntoResponse, AppError> {
    // get access to the pasta collection

    let intern_id = if args.hash_ids {
        to_u64_hash_ids(&id).unwrap_or(0)
    } else {
        to_u64(&id).unwrap_or(0)
    };

    let pasta = match db.get_pasta(&intern_id)? {
        Some(pasta) => Ok::<Pasta, AppError>(pasta.into()),
        None => {
            let mut headers = HeaderMap::new();
            headers.insert("Content-Type", "text/html; charset=utf-8".parse()?);
            let error = dioxus_ssr::render_element(Error(args.into()));
            return Ok((headers, error));
        }
    }?;

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
    Ok((headers, body))
}

pub async fn auth_edit_private_with_status(
    State(AppState { args, db }): State<AppState>,
    Path((id, status)): Path<(String, String)>,
) -> Result<impl IntoResponse, AppError> {
    // get access to the pasta collection

    let intern_id = if args.hash_ids {
        to_u64_hash_ids(&id).unwrap_or(0)
    } else {
        to_u64(&id).unwrap_or(0)
    };

    let pasta = match db.get_pasta(&intern_id)? {
        Some(pasta) => Ok::<Pasta, AppError>(pasta.into()),
        None => {
            let mut headers = HeaderMap::new();
            headers.insert("Content-Type", "text/html; charset=utf-8".parse()?);
            let error = dioxus_ssr::render_element(Error(args.into()));
            return Ok((headers, error));
        }
    }?;

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
    Ok((headers, body))
}

pub async fn auth_file(
    State(AppState { args, db }): State<AppState>,
    Path(id): Path<String>,
) -> Result<impl IntoResponse, AppError> {
    let intern_id = if args.hash_ids {
        to_u64_hash_ids(&id).unwrap_or(0)
    } else {
        to_u64(&id).unwrap_or(0)
    };
    let pasta = match db.get_pasta(&intern_id)? {
        Some(pasta) => Ok::<Pasta, AppError>(pasta.into()),
        None => {
            let mut headers = HeaderMap::new();
            headers.insert("Content-Type", "text/html; charset=utf-8".parse()?);
            let error = dioxus_ssr::render_element(Error(args.into()));
            return Ok((headers, error));
        }
    }?;

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
    Ok((headers, body))
}

pub async fn auth_file_with_status(
    State(AppState { args, db }): State<AppState>,
    Path((id, status)): Path<(String, String)>,
) -> Result<impl IntoResponse, AppError> {
    let intern_id = if args.hash_ids {
        to_u64_hash_ids(&id).unwrap_or(0)
    } else {
        to_u64(&id).unwrap_or(0)
    };

    let opt_pasta = db.get_pasta(&intern_id)?;

    let pasta: Pasta = match opt_pasta {
        Some(pasta) => Ok::<Pasta, AppError>(pasta.into()),
        None => {
            let mut headers = HeaderMap::new();
            headers.insert("Content-Type", "text/html; charset=utf-8".parse()?);
            let error = dioxus_ssr::render_element(Error(args.into()));
            return Ok((headers, error));
        }
    }?;

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
    Ok((headers, body))
}

pub async fn auth_remove_private(
    State(AppState { args, db }): State<AppState>,
    Path(id): Path<String>,
) -> Result<impl IntoResponse, AppError> {
    let intern_id = if args.hash_ids {
        to_u64_hash_ids(&id).unwrap_or(0)
    } else {
        to_u64(&id).unwrap_or(0)
    };

    let pasta = match db.get_pasta(&intern_id)? {
        Some(pasta) => Ok::<Pasta, AppError>(pasta.into()),
        None => {
            let mut headers = HeaderMap::new();
            headers.insert("Content-Type", "text/html; charset=utf-8".parse()?);
            let error = dioxus_ssr::render_element(Error(args.into()));
            return Ok((headers, error));
        }
    }?;
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
    Ok((headers, body))
}

pub async fn auth_remove_private_with_status(
    State(AppState { args, db }): State<AppState>,
    Path((id, status)): Path<(String, String)>,
) -> Result<impl IntoResponse, AppError> {
    let intern_id = if args.hash_ids {
        to_u64_hash_ids(&id).unwrap_or(0)
    } else {
        to_u64(&id).unwrap_or(0)
    };

    let pasta = match db.get_pasta(&intern_id)? {
        Some(pasta) => Ok::<Pasta, AppError>(pasta.into()),
        None => {
            let mut headers = HeaderMap::new();
            let error = dioxus_ssr::render_element(Error(args.into()));
            headers.insert("Content-Type", "text/html; charset=utf-8".parse()?);
            return Ok((headers, error));
        }
    }?;
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
