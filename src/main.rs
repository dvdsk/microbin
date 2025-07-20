extern crate core;

use crate::args::ARGS;
use crate::endpoints::admin::admin_router;
use crate::endpoints::auth_admin::auth_admin_router;
use crate::endpoints::create::create_routes;
use crate::endpoints::edit::edit_router;
use crate::endpoints::errors::not_found;
use crate::endpoints::file::files_router;
use crate::endpoints::guide::guide_router;
use crate::endpoints::list::list_router;
use crate::endpoints::pasta::pasta_routes;
use crate::endpoints::qr::qr_router;
use crate::endpoints::remove::remove_router;
use crate::endpoints::static_resources;
use crate::pasta::Pasta;
use crate::static_resources::static_resource_router;
use crate::util::auth::auth_validator;
use crate::util::db::read_all;
use crate::util::telemetry::start_telemetry_thread;
use axum::{middleware, Router};
use chrono::Local;
use env_logger::Builder;
use log::LevelFilter;
use std::fs;
use std::io::Write;
use std::sync::{Arc, Mutex};
use tower_http::normalize_path::NormalizePathLayer;

pub mod args;
mod error_handling;
pub mod pasta;

pub mod util {
    pub mod animalnumbers;
    pub mod auth;
    pub mod db;
    pub mod db_json;
    #[cfg(feature = "default")]
    pub mod db_sqlite;
    pub mod hashids;
    pub mod http_client;
    pub mod misc;
    pub mod syntaxhighlighter;
    pub mod telemetry;
    pub mod version;
}

pub mod endpoints {
    pub mod admin;
    pub mod auth_admin;
    pub mod auth_upload;
    pub mod create;
    pub mod edit;
    pub mod errors;
    pub mod file;
    pub mod guide;
    pub mod list;
    pub mod pasta;
    pub mod qr;
    pub mod remove;
    pub mod static_resources;
}

#[derive(Clone)]
pub struct AppState {
    pub pastas: Arc<Mutex<Vec<Pasta>>>,
}

#[tokio::main]
async fn main() -> std::io::Result<()> {
    Builder::new()
        .format(|buf, record| {
            writeln!(
                buf,
                "{} [{}] - {}",
                Local::now().format("%Y-%m-%dT%H:%M:%S"),
                record.level(),
                record.args()
            )
        })
        .filter(None, LevelFilter::Info)
        .init();

    log::info!(
        "MicroBin starting on http://{}:{}",
        ARGS.bind.to_string(),
        ARGS.port.to_string()
    );

    match fs::create_dir_all(format!("{}/public", ARGS.data_dir)) {
        Ok(dir) => dir,
        Err(error) => {
            log::error!(
                "Couldn't create data directory {}/attachments/: {:?}",
                ARGS.data_dir,
                error
            );
            panic!(
                "Couldn't create data directory {}/attachments/: {:?}",
                ARGS.data_dir, error
            );
        }
    };

    let app_state = AppState {
        pastas: Arc::new(Mutex::new(read_all())),
    };

    let mut router = Router::new()
        .merge(create_routes())
        .merge(admin_router())
        .merge(edit_router())
        .merge(files_router())
        .merge(guide_router())
        .merge(list_router())
        .merge(pasta_routes())
        .merge(qr_router())
        .merge(remove_router())
        .merge(static_resource_router())
        .merge(auth_admin_router())
        .fallback(not_found)
        .with_state(app_state);

    if !ARGS.disable_telemetry {
        start_telemetry_thread();
    }

    if ARGS.auth_basic_username.is_some() && ARGS.auth_basic_username.as_ref().unwrap().trim() != ""
    {
        log::info!("Basic authentication is enabled.");
        router = router.layer(middleware::from_fn(auth_validator));
    }

    let app = router.layer(NormalizePathLayer::trim_trailing_slash());

    let tcp = tokio::net::TcpListener::bind((ARGS.bind, ARGS.port)).await?;

    axum::serve(tcp, app).await.unwrap();
    Ok(())
}
