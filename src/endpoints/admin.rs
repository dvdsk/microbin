use crate::AppState;
use crate::args::{Args};
use crate::error_handling::AppError;
use crate::pasta::Pasta;
use crate::util::misc::remove_expired;
use crate::util::version::{CURRENT_VERSION, Version, fetch_latest_version};
use askama::Template;
use axum::Router;
use axum::extract::{Multipart, State};
use axum::response::{IntoResponse, Response};
use axum::routing::{get, post};
use futures::TryStreamExt;
use reqwest::{StatusCode, header};

#[derive(Template)]
#[template(path = "admin.html")]
struct AdminTemplate<'a> {
    pastas: &'a Vec<Pasta>,
    args: &'a Args,
    status: &'a String,
    version_string: &'a String,
    message: &'a String,
    update: &'a Option<Version>,
}

pub async fn get_admin(State(AppState{args,..}): State<AppState>,) -> Result<impl IntoResponse, AppError> {
    Ok((
        StatusCode::FOUND,
        [(
            header::LOCATION,
            format!("{}/auth_admin", &args.public_path_as_str()),
        )],
        "".to_string(),
    ))
}

pub async fn post_admin(
    State(AppState{pastas,args}): State<AppState>,
    mut payload: Multipart,
) -> Result<Response, AppError> {
    let mut username = String::from("");
    let mut password = String::from("");

    while let Some(mut field) = payload.next_field().await? {
        if field.name() == Some("username") {
            while let Some(chunk) = field.try_next().await? {
                username.push_str(
                    std::str::from_utf8(&chunk)
                        .map_err(AppError::from)?
                        .to_string()
                        .as_str(),
                );
            }
        } else if field.name() == Some("password") {
            while let Some(chunk) = field.try_next().await? {
                password.push_str(
                    std::str::from_utf8(&chunk)
                        .map_err(AppError::from)?
                        .to_string()
                        .as_str(),
                );
            }
        }
    }

    if username != args.auth_admin_username || password != args.auth_admin_password {
        return Ok((
            StatusCode::FOUND,
            [(
                header::LOCATION,
                format!("{}/auth_admin/incorrect", args.public_path_as_str()),
            )],
            "".to_string(),
        )
            .into_response());
    }

    let pastas = {
        let mut pastas = pastas.lock().expect("no microbin thread should panic");

        remove_expired(&mut pastas, &args);

        // sort pastas in reverse-chronological order of creation time
        pastas.sort_by(|a, b| b.created.cmp(&a.created));
        pastas.to_vec()
    };

    // todo status report more sophisticated
    let mut status = "OK";
    let mut message = "";

    if args.public_path.is_none() {
        status = "WARNING";
        message = "Warning: No public URL set with --public-path parameter. QR code and URL Copying functions have been disabled"
    }

    if args.auth_admin_username == "admin" && args.auth_admin_password == "m1cr0b1n" {
        status = "WARNING";
        message = "Warning: You are using the default admin login details. This is a security risk, please change them."
    }

    let update;

    if !args.disable_update_checking {
        let latest_version_res = fetch_latest_version().await;
        if let Ok(latest_version) = latest_version_res {
            if latest_version.newer_than_current() {
                update = Some(latest_version);
            } else {
                update = None;
            }
        } else {
            update = None;
        }
    } else {
        update = None;
    }

    Ok((
        StatusCode::OK,
        [(header::CONTENT_TYPE, "text/html; charset=utf-8".to_string())],
        AdminTemplate {
            pastas: &pastas,
            args: &args,
            status: &String::from(status),
            version_string: &format!("{}", CURRENT_VERSION.long_title),
            message: &String::from(message),
            update: &update,
        }
        .render()?,
    )
        .into_response())
}

pub fn admin_router() -> Router<AppState> {
    Router::new()
        .route("/admin/", post(post_admin))
        .route("/admin", get(get_admin))
}
