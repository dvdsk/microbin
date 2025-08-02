use std::{
    sync::{Arc, Mutex},
    thread,
    time::Duration,
};

use crate::{pasta::Pasta, util::misc::{remove_expired, cleanup_orphaned_files}, args::Args};

pub fn start_cleanup_thread(app_state: Arc<Mutex<Vec<Pasta>>>, args: Args) {
    thread::spawn(move || {
        log::info!("Started background cleanup thread - running immediately and then every hour");
        
        loop {
            match app_state.lock() {
                Ok(mut pastas) => {
                    // Clean up expired pastes
                    let count_before = pastas.len();
                    remove_expired(&mut pastas, &args);
                    let count_after = pastas.len();
                    let removed_count = count_before - count_after;
                    
                    if removed_count > 0 {
                        log::info!("Background cleanup: removed {} expired paste(s)", removed_count);
                    } else {
                        log::debug!("Background cleanup: no expired pastes found");
                    }
                    
                    // Clean up orphaned files (files without pasta references)
                    cleanup_orphaned_files(&pastas, &args);
                }
                Err(e) => {
                    log::error!("Background cleanup failed to acquire pasta lock: {}", e);
                }
            }
            
            thread::sleep(Duration::from_secs(60 * 60)); // 1 hour
        }
    });
}