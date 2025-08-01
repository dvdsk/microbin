use std::{
    sync::{Arc, Mutex},
    thread,
    time::{Duration, Instant},
};

use crate::{pasta::Pasta, util::misc::remove_expired};

pub fn start_cleanup_thread(app_state: Arc<Mutex<Vec<Pasta>>>) {
    // Start a new thread that calls the cleanup function every hour
    thread::spawn(move || {
        let mut last_run = Instant::now();
        
        log::info!("Started background cleanup thread - checking for expired pastes every hour");
        
        loop {
            // Wait for 1 hour since the last run
            let next_run = last_run + Duration::from_secs(60 * 60); // 1 hour
            let now = Instant::now();
            if next_run > now {
                thread::sleep(next_run - now);
            }
            
            // Perform cleanup
            match app_state.lock() {
                Ok(mut pastas) => {
                    let count_before = pastas.len();
                    remove_expired(&mut pastas);
                    let count_after = pastas.len();
                    let removed_count = count_before - count_after;
                    
                    if removed_count > 0 {
                        log::info!("Background cleanup: removed {} expired paste(s)", removed_count);
                    } else {
                        log::debug!("Background cleanup: no expired pastes found");
                    }
                }
                Err(e) => {
                    log::error!("Background cleanup failed to acquire pasta lock: {}", e);
                }
            }
            
            last_run = Instant::now();
        }
    });
}