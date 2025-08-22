use crate::error_handling::AppError;
use crate::{util::misc::clean_up_expired_pastes};
use db::database::DatabaseType;
use std::{fs, sync::Arc, thread, time::Duration};
use crate::args::Args;
use crate::pasta::Pasta;

pub fn start_cleanup_thread(db: &DatabaseType, args: Args) {
    let db = Arc::clone(db);
    thread::spawn(move || {
        log::info!("Started background cleanup thread - running immediately and then every hour");

        loop {
            let pastas = match db.find_all_pastas() {
                Ok(pastas) => pastas.iter().map(Pasta::from).collect::<Vec<Pasta>>(),
                Err(e) => {
                    log::error!("Failed to fetch pastas for cleanup: {}", e);
                    thread::sleep(Duration::from_secs(60 * 60)); // 1 hour
                    continue;
                }
            };
            let count_before = pastas.len();
            clean_up_expired_pastes(&args, &db).unwrap_or_else(|e| {
                log::error!("Failed to clean up expired pastas: {}", e);
            });
            let count_after = pastas.len();
            let removed_count = count_before - count_after;

            if removed_count > 0 {
                log::info!(
                    "Background cleanup: removed {} expired paste(s)",
                    removed_count
                );
            } else {
                log::debug!("Background cleanup: no expired pastes found");
            }

            if let Err(e) = cleanup_orphaned_files(&pastas, &args) {
                log::error!("Failed to cleanup paste: {}", e);
            }

            thread::sleep(Duration::from_secs(60 * 60)); // 1 hour
        }
    });
}

// Clean up orphaned files (files without pasta references)
pub fn cleanup_orphaned_files(pastas: &[Pasta], args: &Args) -> Result<(), AppError> {
    log::debug!("Starting orphaned files cleanup");

    let attachments_dir = format!("{}/attachments", args.data_dir);
    let dirs = match fs::read_dir(&attachments_dir) {
        Ok(dirs) => dirs,
        Err(e) => {
            if e.kind() == std::io::ErrorKind::NotFound {
                fs::create_dir_all(&attachments_dir)?;
                return cleanup_orphaned_files(pastas, args);
            }
            log::debug!("No attachments directory found or unable to read: {}", e);
            return Err(e.into());
        }
    };

    let mut orphaned_count = 0;
    for dir_entry in dirs.flatten() {
        if !dir_entry
            .file_type()
            .map(|ft| ft.is_dir())
            .map_err(AppError::from)?
        {
            continue;
        }

        let dir_name = dir_entry.file_name().to_string_lossy().to_string();

        // Check if any pasta references this directory
        let is_referenced = pastas
            .iter()
            .any(|pasta| pasta.file.is_some() && pasta.id_as_animals(&args.hash_ids) == dir_name);

        if !is_referenced {
            log::debug!("Found orphaned directory: {}", dir_name);
            let full_path = dir_entry.path();

            // Remove the entire directory and its contents
            if let Err(e) = fs::remove_dir_all(&full_path) {
                log::error!("Failed to remove orphaned directory {:?}: {}", full_path, e);
            } else {
                log::debug!("Removed orphaned directory: {:?}", full_path);
                orphaned_count += 1;
            }
        }
    }

    if orphaned_count > 0 {
        log::info!(
            "Orphaned files cleanup: removed {} orphaned directories",
            orphaned_count
        );
    } else {
        log::debug!("Orphaned files cleanup: no orphaned files found");
    }
    Ok(())
}
