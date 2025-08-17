extern crate core;

use crate::args::Args;
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
use crate::util::cleanup::start_cleanup_thread;
use crate::util::telemetry::start_telemetry_thread;
use ::db::database::{Database, get_database};
use axum::extract::DefaultBodyLimit;
use axum::{Router, middleware};
use chrono::Local;
use clap::Parser;
use env_logger::{Builder, Env};
use std::io::Write;
use std::sync::Arc;
use std::{env, fs};
use tower_http::normalize_path::NormalizePathLayer;

pub mod args;
mod error_handling;
pub mod pasta;

pub mod util {
    pub mod animalnumbers;
    pub mod auth;
    pub mod cleanup;
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
    pub args: Args,
    pub db: Arc<dyn Database + Send + Sync>,
}

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let args = Args::parse();

    if env::var("MICROBIN_LOG").is_err() {
        unsafe {
            env::set_var("MICROBIN_LOG", "info");
        }
    }

    let default_log_env = Env::new().filter_or("MICROBIN_LOG", "info");

    Builder::from_env(default_log_env)
        .format(|buf, record| {
            writeln!(
                buf,
                "{} [{}] - {}",
                Local::now().format("%Y-%m-%dT%H:%M:%S"),
                record.level(),
                record.args()
            )
        })
        .init();

    match fs::create_dir_all(format!("{}/public", args.data_dir)) {
        Ok(dir) => dir,
        Err(error) => {
            log::error!(
                "Couldn't create data directory {}/attachments/: {:?}",
                args.data_dir,
                error
            );
            panic!(
                "Couldn't create data directory {}/attachments/: {:?}",
                args.data_dir, error
            );
        }
    };

    let db_args = args.clone();
    let db = get_database(db_args.into());
    let app_state = AppState {
        db,
        args: args.clone(),
    };

    start_cleanup_thread(&app_state.db, args.clone());

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
        .with_state(app_state.clone());

    if args.enable_telemetry {
        start_telemetry_thread(&args);
    }

    if let Some(username) = args.auth_basic_username.as_ref()
        && username.trim() != ""
    {
        log::info!("Basic authentication is enabled.");
        router = router.layer(middleware::from_fn_with_state(app_state, auth_validator));
    }

    let max_size = std::cmp::max(
        args.max_file_size_encrypted_mb,
        args.max_file_size_unencrypted_mb,
    );
    let body_limit = (max_size + 10) * 1024 * 1024; // Add 10MB overhead for multipart encoding

    log::info!(
        "Configured file size limits - encrypted: {}MB, unencrypted: {}MB",
        args.max_file_size_encrypted_mb,
        args.max_file_size_unencrypted_mb
    );
    log::info!(
        "Setting HTTP body limit to: {}MB ({} bytes)",
        (max_size + 10),
        body_limit
    );
    log::info!("MicroBin starting on http://{}:{}", args.bind, args.port);

    let app = router
        .layer(DefaultBodyLimit::max(body_limit))
        .layer(NormalizePathLayer::trim_trailing_slash());
    let tcp = tokio::net::TcpListener::bind((args.bind, args.port)).await?;
    axum::serve(tcp, app).await?;
    Ok(())
}
