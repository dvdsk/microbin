use crate::args::{Args, ARGS};
use crate::error_handling::AppError;
use crate::pasta::Pasta;
use crate::util::misc::remove_expired;
use crate::util::version::{fetch_latest_version, Version, CURRENT_VERSION};
use crate::AppState;
use askama::Template;
use axum::extract::{Multipart, State};
use axum::response::{IntoResponse, Response};
use axum::routing::{get, post};
use axum::Router;
use futures::TryStreamExt;
use reqwest::{header, StatusCode};

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

pub async fn get_admin() -> Result<impl IntoResponse, AppError> {
    Ok((
        StatusCode::FOUND,
        [(
            header::LOCATION,
            format!("{}/auth_admin", ARGS.public_path_as_str()),
        )],
        "".to_string(),
    ))
}

pub async fn post_admin(
    State(data): State<AppState>,
    mut payload: Multipart,
) -> Result<Response, AppError> {
    let mut username = String::from("");
    let mut password = String::from("");

    while let Some(mut field) = payload.next_field().await? {
        if field.name() == Some("username") {
            while let Some(chunk) = field.try_next().await? {
                username.push_str(std::str::from_utf8(&chunk).unwrap().to_string().as_str());
            }
        } else if field.name() == Some("password") {
            while let Some(chunk) = field.try_next().await? {
                password.push_str(std::str::from_utf8(&chunk).unwrap().to_string().as_str());
            }
        }
    }

    if username != ARGS.auth_admin_username || password != ARGS.auth_admin_password {
        return Ok((
            StatusCode::FOUND,
            [(
                header::LOCATION,
                format!("{}/auth_admin/incorrect", ARGS.public_path_as_str()),
            )],
            "".to_string(),
        )
            .into_response());
    }

    let pastas = {
        let mut pastas = data.pastas.lock().unwrap();

        remove_expired(&mut pastas);

        // sort pastas in reverse-chronological order of creation time
        pastas.sort_by(|a, b| b.created.cmp(&a.created));
        pastas.to_vec()
    };

    // todo status report more sophisticated
    let mut status = "OK";
    let mut message = "";

    if ARGS.public_path.is_none() {
        status = "WARNING";
        message = "Warning: No public URL set with --public-path parameter. QR code and URL Copying functions have been disabled"
    }

    if ARGS.auth_admin_username == "admin" && ARGS.auth_admin_password == "m1cr0b1n" {
        status = "WARNING";
        message = "Warning: You are using the default admin login details. This is a security risk, please change them."
    }

    let update;

    if !ARGS.disable_update_checking {
        let latest_version_res = fetch_latest_version().await;
        if latest_version_res.is_ok() {
            let latest_version = latest_version_res.unwrap();
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
            args: &ARGS,
            status: &String::from(status),
            version_string: &format!("{}", CURRENT_VERSION.long_title),
            message: &String::from(message),
            update: &update,
        }
        .render()
        .unwrap(),
    )
        .into_response())
}

pub fn admin_router() -> Router<AppState> {
    Router::new()
        .route("/admin/", post(post_admin))
        .route("/admin", get(get_admin))
}
