use crate::args::{Args, ARGS};
use crate::endpoints::errors::ErrorTemplate;
use crate::util::animalnumbers::to_u64;
use crate::util::hashids::to_u64 as hashid_to_u64;
use crate::util::misc::remove_expired;
use crate::AppState;
use askama::Template;
use axum::extract::{Path, State};
use axum::http::HeaderMap;
use axum::response::IntoResponse;
use axum::routing::get;
use axum::Router;

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
    State(data): State<AppState>,
    Path(id): Path<String>,
) -> impl IntoResponse {
    // get access to the pasta collection
    let mut pastas = data.pastas.lock().unwrap();

    remove_expired(&mut pastas);

    let intern_id = if ARGS.hash_ids {
        hashid_to_u64(&id).unwrap_or(0)
    } else {
        to_u64(&id).unwrap_or(0)
    };

    for pasta in pastas.iter() {
        if pasta.id == intern_id {
            let mut headers = HeaderMap::new();
            headers.insert("Content-Type", "text/html; charset=utf-8".parse().unwrap());
            let body = AuthPasta {
                args: &ARGS,
                id,
                status: String::from(""),
                encrypted_key: pasta.encrypted_key.to_owned().unwrap_or_default(),
                encrypt_client: pasta.encrypt_client,
                path: String::from("upload"),
            }
            .render()
            .unwrap();
            return (headers, body);
        }
    }

    let mut headers = HeaderMap::new();
    headers.insert("Content-Type", "text/html; charset=utf-8".parse().unwrap());
    let body = ErrorTemplate { args: &ARGS }.render().unwrap();
    (headers, body)
}

pub async fn auth_upload_with_status(
    State(data): State<AppState>,
    Path((id, status)): Path<(String, String)>,
) -> impl IntoResponse {
    // get access to the pasta collection
    let mut pastas = data.pastas.lock().unwrap();

    remove_expired(&mut pastas);

    let intern_id = if ARGS.hash_ids {
        hashid_to_u64(&id).unwrap_or(0)
    } else {
        to_u64(&id).unwrap_or(0)
    };

    for pasta in pastas.iter() {
        if pasta.id == intern_id {
            let mut headers = HeaderMap::new();
            headers.insert("Content-Type", "text/html; charset=utf-8".parse().unwrap());
            let body = AuthPasta {
                args: &ARGS,
                id,
                status,
                encrypted_key: pasta.encrypted_key.to_owned().unwrap_or_default(),
                encrypt_client: pasta.encrypt_client,
                path: String::from("upload"),
            }
            .render()
            .unwrap();
            return (headers, body);
        }
    }

    let mut headers = HeaderMap::new();
    headers.insert("Content-Type", "text/html; charset=utf-8".parse().unwrap());
    let body = ErrorTemplate { args: &ARGS }.render().unwrap();
    (headers, body)
}

pub async fn auth_raw_pasta(
    State(data): State<AppState>,
    Path(id): Path<String>,
) -> impl IntoResponse {
    // get access to the pasta collection
    let mut pastas = data.pastas.lock().unwrap();

    remove_expired(&mut pastas);

    let intern_id = if ARGS.hash_ids {
        hashid_to_u64(&id).unwrap_or(0)
    } else {
        to_u64(&id).unwrap_or(0)
    };

    for pasta in pastas.iter() {
        if pasta.id == intern_id {
            let mut headers = HeaderMap::new();
            headers.insert("Content-Type", "text/html; charset=utf-8".parse().unwrap());
            let body = AuthPasta {
                args: &ARGS,
                id,
                status: String::from(""),
                encrypted_key: pasta.encrypted_key.to_owned().unwrap_or_default(),
                encrypt_client: pasta.encrypt_client,
                path: String::from("raw"),
            }
            .render()
            .unwrap();
            return (headers, body);
        }
    }

    let mut headers = HeaderMap::new();
    headers.insert("Content-Type", "text/html; charset=utf-8".parse().unwrap());
    let body = ErrorTemplate { args: &ARGS }.render().unwrap();
    (headers, body)
}

pub async fn auth_raw_pasta_with_status(
    State(data): State<AppState>,
    Path((id, status)): Path<(String, String)>,
) -> impl IntoResponse {
    // get access to the pasta collection
    let mut pastas = data.pastas.lock().unwrap();

    remove_expired(&mut pastas);

    let intern_id = if ARGS.hash_ids {
        hashid_to_u64(&id).unwrap_or(0)
    } else {
        to_u64(&id).unwrap_or(0)
    };

    for pasta in pastas.iter() {
        if pasta.id == intern_id {
            let mut headers = HeaderMap::new();
            headers.insert("Content-Type", "text/html; charset=utf-8".parse().unwrap());
            let body = AuthPasta {
                args: &ARGS,
                id,
                status,
                encrypted_key: pasta.encrypted_key.to_owned().unwrap_or_default(),
                encrypt_client: pasta.encrypt_client,
                path: String::from("raw"),
            }
            .render()
            .unwrap();
            return (headers, body);
        }
    }
    let mut headers = HeaderMap::new();
    headers.insert("Content-Type", "text/html; charset=utf-8".parse().unwrap());
    let body = ErrorTemplate { args: &ARGS }.render().unwrap();
    (headers, body)
}

pub async fn auth_edit_private(
    State(data): State<AppState>,
    Path(id): Path<String>,
) -> impl IntoResponse {
    // get access to the pasta collection
    let mut pastas = data.pastas.lock().unwrap();

    remove_expired(&mut pastas);

    let intern_id = if ARGS.hash_ids {
        hashid_to_u64(&id).unwrap_or(0)
    } else {
        to_u64(&id).unwrap_or(0)
    };

    for pasta in pastas.iter() {
        if pasta.id == intern_id {
            let mut headers = HeaderMap::new();
            headers.insert("Content-Type", "text/html; charset=utf-8".parse().unwrap());
            let body = AuthPasta {
                args: &ARGS,
                id,
                status: String::from(""),
                encrypted_key: pasta.encrypted_key.to_owned().unwrap_or_default(),
                encrypt_client: pasta.encrypt_client,
                path: String::from("edit_private"),
            }
            .render()
            .unwrap();
            return (headers, body);
        }
    }

    let mut headers = HeaderMap::new();
    headers.insert("Content-Type", "text/html; charset=utf-8".parse().unwrap());
    let body = ErrorTemplate { args: &ARGS }.render().unwrap();
    (headers, body)
}

pub async fn auth_edit_private_with_status(
    State(data): State<AppState>,
    Path((id, status)): Path<(String, String)>,
) -> impl IntoResponse {
    // get access to the pasta collection
    let mut pastas = data.pastas.lock().unwrap();

    remove_expired(&mut pastas);

    let intern_id = if ARGS.hash_ids {
        hashid_to_u64(&id).unwrap_or(0)
    } else {
        to_u64(&id).unwrap_or(0)
    };

    for pasta in pastas.iter() {
        if pasta.id == intern_id {
            let mut headers = HeaderMap::new();
            headers.insert("Content-Type", "text/html; charset=utf-8".parse().unwrap());
            let body = AuthPasta {
                args: &ARGS,
                id,
                status,
                encrypted_key: pasta.encrypted_key.to_owned().unwrap_or_default(),
                encrypt_client: pasta.encrypt_client,
                path: String::from("edit_private"),
            }
            .render()
            .unwrap();
            return (headers, body);
        }
    }
    let mut headers = HeaderMap::new();
    headers.insert("Content-Type", "text/html; charset=utf-8".parse().unwrap());
    let body = ErrorTemplate { args: &ARGS }.render().unwrap();
    (headers, body)
}

pub async fn auth_file(data: State<AppState>, Path(id): Path<String>) -> impl IntoResponse {
    // get access to the pasta collection
    let mut pastas = data.pastas.lock().unwrap();

    remove_expired(&mut pastas);

    let intern_id = if ARGS.hash_ids {
        hashid_to_u64(&id).unwrap_or(0)
    } else {
        to_u64(&id).unwrap_or(0)
    };

    for pasta in pastas.iter() {
        if pasta.id == intern_id {
            let mut headers = HeaderMap::new();
            headers.insert("Content-Type", "text/html; charset=utf-8".parse().unwrap());
            let body = AuthPasta {
                args: &ARGS,
                id,
                status: String::from(""),
                encrypted_key: pasta.encrypted_key.to_owned().unwrap_or_default(),
                encrypt_client: pasta.encrypt_client,
                path: String::from("secure_file"),
            }
            .render()
            .unwrap();
            return (headers, body);
        }
    }

    let mut headers = HeaderMap::new();
    headers.insert("Content-Type", "text/html; charset=utf-8".parse().unwrap());
    let body = ErrorTemplate { args: &ARGS }.render().unwrap();
    (headers, body)
}

pub async fn auth_file_with_status(
    State(data): State<AppState>,
    Path((id, status)): Path<(String, String)>,
) -> impl IntoResponse {
    // get access to the pasta collection
    let mut pastas = data.pastas.lock().unwrap();

    remove_expired(&mut pastas);

    let intern_id = if ARGS.hash_ids {
        hashid_to_u64(&id).unwrap_or(0)
    } else {
        to_u64(&id).unwrap_or(0)
    };

    for pasta in pastas.iter() {
        if pasta.id == intern_id {
            let mut headers = HeaderMap::new();
            headers.insert("Content-Type", "text/html; charset=utf-8".parse().unwrap());
            let body = AuthPasta {
                args: &ARGS,
                id,
                status,
                encrypted_key: pasta.encrypted_key.to_owned().unwrap_or_default(),
                encrypt_client: pasta.encrypt_client,
                path: String::from("secure_file"),
            }
            .render()
            .unwrap();
            return (headers, body);
        }
    }

    let mut headers = HeaderMap::new();
    headers.insert("Content-Type", "text/html; charset=utf-8".parse().unwrap());
    let body = ErrorTemplate { args: &ARGS }.render().unwrap();
    (headers, body)
}

pub async fn auth_remove_private(
    State(data): State<AppState>,
    Path(id): Path<String>,
) -> impl IntoResponse {
    // get access to the pasta collection
    let mut pastas = data.pastas.lock().unwrap();

    remove_expired(&mut pastas);

    let intern_id = if ARGS.hash_ids {
        hashid_to_u64(&id).unwrap_or(0)
    } else {
        to_u64(&id).unwrap_or(0)
    };

    for pasta in pastas.iter() {
        if pasta.id == intern_id {
            let mut headers = HeaderMap::new();
            headers.insert("Content-Type", "text/html; charset=utf-8".parse().unwrap());
            let body = AuthPasta {
                args: &ARGS,
                id,
                status: String::from(""),
                encrypted_key: pasta.encrypted_key.to_owned().unwrap_or_default(),
                encrypt_client: pasta.encrypt_client,
                path: String::from("remove"),
            }
            .render()
            .unwrap();
            return (headers, body);
        }
    }

    let mut headers = HeaderMap::new();
    headers.insert("Content-Type", "text/html; charset=utf-8".parse().unwrap());
    let body = ErrorTemplate { args: &ARGS }.render().unwrap();
    (headers, body)
}

pub async fn auth_remove_private_with_status(
    State(state): State<AppState>,
    Path((id, status)): Path<(String, String)>,
) -> impl IntoResponse {
    // get access to the pasta collection
    let mut pastas = state.pastas.lock().unwrap();

    remove_expired(&mut pastas);

    let intern_id = if ARGS.hash_ids {
        hashid_to_u64(&id).unwrap_or(0)
    } else {
        to_u64(&id).unwrap_or(0)
    };

    for pasta in pastas.iter() {
        if pasta.id == intern_id {
            let mut headers = HeaderMap::new();
            let body = AuthPasta {
                args: &ARGS,
                id: id.clone(),
                status: status.clone(),
                encrypted_key: pasta.encrypted_key.to_owned().unwrap_or_default(),
                encrypt_client: pasta.encrypt_client,
                path: String::from("remove"),
            }
            .render()
            .unwrap();
            headers.insert("Content-Type", "text/html; charset=utf-8".parse().unwrap());
            return (headers, body);
        }
    }

    let mut headers = HeaderMap::new();
    let body = ErrorTemplate { args: &ARGS }.render().unwrap();
    headers.insert("Content-Type", "text/html; charset=utf-8".parse().unwrap());
    (headers, body)
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
