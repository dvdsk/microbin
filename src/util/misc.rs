use crate::Pasta;
use crate::error_handling::AppError;
use linkify::{LinkFinder, LinkKind};
use magic_crypt::{MagicCryptTrait, new_magic_crypt};
use qrcode_generator::QrCodeEcc;
use std::fs::{self, File};
use std::io::{BufReader, Read, Write};
use std::path::Path;
use std::time::{SystemTime, UNIX_EPOCH};
use crate::args::Args;
use super::db::delete;

pub fn cleanup_orphaned_files(pastas: &Vec<Pasta>, args: &Args) {
    log::debug!("Starting orphaned files cleanup");
    
    let attachments_dir = format!("{}/attachments", args.data_dir);
    let dirs = match std::fs::read_dir(&attachments_dir) {
        Ok(dirs) => dirs,
        Err(e) => {
            log::debug!("No attachments directory found or unable to read: {}", e);
            return;
        }
    };
    
    let mut orphaned_count = 0;
    for dir_entry in dirs.flatten() {
        if !dir_entry.file_type().map(|ft| ft.is_dir()).unwrap_or(false) {
            continue;
        }
        
        let dir_name = dir_entry.file_name().to_string_lossy().to_string();
        
        // Check if any pasta references this directory
        let is_referenced = pastas.iter().any(|pasta| {
            pasta.file.is_some() && pasta.id_as_animals(&args.hash_ids) == dir_name
        });
        
        if !is_referenced {
            log::debug!("Found orphaned directory: {}", dir_name);
            let full_path = dir_entry.path();
            
            // Remove the entire directory and its contents
            if let Err(e) = std::fs::remove_dir_all(&full_path) {
                log::error!("Failed to remove orphaned directory {:?}: {}", full_path, e);
            } else {
                log::info!("Removed orphaned directory: {:?}", full_path);
                orphaned_count += 1;
            }
        }
    }
    
    if orphaned_count > 0 {
        log::info!("Orphaned files cleanup: removed {} orphaned directories", orphaned_count);
    } else {
        log::debug!("Orphaned files cleanup: no orphaned files found");
    }
}

pub fn remove_expired(pastas: &mut Vec<Pasta>, args: &Args) {
    // get current time - this will be needed to check which pastas have expired
    let timenow: i64 = match SystemTime::now().duration_since(UNIX_EPOCH) {
        Ok(n) => n.as_secs(),
        Err(_) => {
            log::error!("SystemTime before UNIX EPOCH!");
            0
        }
    } as i64;

    pastas.retain(|p| {
        // keep if:
        //  expiration is `never` or not reached
        //  AND
        //  read count is less than burn limit, or no limit set
        //  AND
        //  has been read in the last N days where N is the arg --gc-days OR N is 0 (no GC)
        if (p.expiration == 0 || p.expiration > timenow)
            && (p.read_count < p.burn_after_reads || p.burn_after_reads == 0)
            && (p.last_read_days_ago() < args.gc_days || args.gc_days == 0)
        {
            // keep
            true
        } else {
            log::debug!("Removing expired pasta ID: {} ({})", p.id, p.id_as_animals(&args.hash_ids));
            if p.expiration > 0 && p.expiration <= timenow {
                log::debug!("  Reason: Expired (expiration: {}, now: {})", p.expiration, timenow);
            }
            if p.burn_after_reads > 0 && p.read_count >= p.burn_after_reads {
                log::debug!("  Reason: Burn after reads limit reached ({}/{})", p.read_count, p.burn_after_reads);
            }
            if args.gc_days > 0 && p.last_read_days_ago() >= args.gc_days {
                log::debug!("  Reason: GC limit reached (last read {} days ago, limit: {})", p.last_read_days_ago(), args.gc_days);
            }
            
            // remove from database
            delete(None, Some(p.id), args);

            // remove the file itself
            if let Some(file) = &p.file {
                let file_path = format!(
                    "{}/attachments/{}/{}",
                    args.data_dir,
                    p.id_as_animals(&args.hash_ids),
                    file.name()
                );
                log::debug!("Attempting to delete file: {}", file_path);
                if let Err(e) = fs::remove_file(&file_path) {
                    log::error!("Failed to delete file {}: {}", file_path, e);
                } else {
                    log::debug!("Successfully deleted file: {}", file_path);
                }

                // and remove the containing directory
                let dir_path = format!(
                    "{}/attachments/{}/",
                    args.data_dir,
                    p.id_as_animals(&args.hash_ids)
                );
                log::debug!("Attempting to delete directory: {}", dir_path);
                if let Err(e) = fs::remove_dir(&dir_path) {
                    log::error!("Failed to delete directory {}: {}", dir_path, e);
                } else {
                    log::debug!("Successfully deleted directory: {}", dir_path);
                }
            }
            false
        }
    });
}

pub fn string_to_qr_svg(str: &str) -> String {
    qrcode_generator::to_svg_to_string(str, QrCodeEcc::Low, 256, None::<&str>).expect(
        "should be able to generate QR code SVG if the string is not empty",
    )
}

pub fn is_valid_url(url: &str) -> bool {
    let finder = LinkFinder::new();
    let spans: Vec<_> = finder.spans(url).collect();
    spans[0].as_str() == url && Some(&LinkKind::Url) == spans[0].kind()
}

pub fn encrypt(text_str: &str, key_str: &str) -> String {
    if text_str.is_empty() {
        return String::from("");
    }

    let mc = new_magic_crypt!(key_str, 256);

    mc.encrypt_str_to_base64(text_str)
}

pub fn decrypt(text_str: &str, key_str: &str) -> Result<String, magic_crypt::MagicCryptError> {
    if text_str.is_empty() {
        return Ok(String::from(""));
    }

    let mc = new_magic_crypt!(key_str, 256);

    mc.decrypt_base64_to_string(text_str)
}

pub fn encrypt_file(
    passphrase: &str,
    input_file_path: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    // Read the input file into memory
    let file = File::open(input_file_path).expect("Tried to encrypt non-existent file");
    let mut reader = BufReader::new(file);
    let mut input_data = Vec::new();
    reader.read_to_end(&mut input_data)?;

    // Create a MagicCrypt instance with the given passphrase
    let mc = new_magic_crypt!(passphrase, 256);

    // Encrypt the input data
    let ciphertext = mc.encrypt_bytes_to_bytes(&input_data[..]);

    // Write the encrypted data to a new file with the .enc extension
    let mut f = File::create(
        Path::new(input_file_path)
            .with_file_name("data")
            .with_extension("enc"),
    )?;
    f.write_all(ciphertext.as_slice())?;

    // Delete the original input file
    // input_file.seek(SeekFrom::Start(0))?;
    fs::remove_file(input_file_path)?;

    Ok(())
}

pub fn decrypt_file(passphrase: &str, input_file: &File) -> Result<Vec<u8>, AppError> {
    // Read the input file into memory
    let mut reader = BufReader::new(input_file);
    let mut ciphertext = Vec::new();
    reader.read_to_end(&mut ciphertext)?;

    // Create a MagicCrypt instance with the given passphrase
    let mc = new_magic_crypt!(passphrase, 256);
    // Encrypt the input data
    let res = mc.decrypt_bytes_to_bytes(&ciphertext[..]);

    match res {
        Ok(data) => Ok(data),
        Err(e) => Err(AppError::from(e)),
    }
}
