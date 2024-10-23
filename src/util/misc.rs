use crate::args::ARGS;
use linkify::{LinkFinder, LinkKind};
use magic_crypt::{new_magic_crypt, MagicCryptTrait};
use qrcode_generator::QrCodeEcc;
use std::path::Path;
use std::time::{SystemTime, UNIX_EPOCH};
use tokio::fs;

use crate::Pasta;

use super::db::delete;

pub async fn remove_expired(pastas: &mut Vec<Pasta>) {
    // get current time - this will be needed to check which pastas have expired
    let timenow: i64 = match SystemTime::now().duration_since(UNIX_EPOCH) {
        Ok(n) => n.as_secs(),
        Err(_) => {
            log::error!("SystemTime before UNIX EPOCH!");
            0
        }
    } as i64;

    // TODO these not set things should really use
    // Option instead of comparing to zero.
    let to_remove: Vec<usize> = pastas
        .iter()
        .enumerate()
        .filter(|(_, p)| {
            !(p.expiration == 0 || p.expiration > timenow)
                && (p.read_count < p.burn_after_reads || p.burn_after_reads == 0)
                && (p.last_read_days_ago() < ARGS.gc_days || ARGS.gc_days == 0)
        })
        .map(|(index, _)| index)
        .collect();

    // TODO feed all the pastas that need to be removed in bulk to the
    // database
    for pasta_idx in to_remove.into_iter().rev() {
        let removed = pastas.swap_remove(pasta_idx);

        // TODO database should be an object preferable a trait object
        // implementing some unified interface.
        // TODO database should be the single source of truth and replace
        // the pastas vector

        // remove from database
        delete(None, Some(removed.id)).await;

        let Some(ref pasta_file) = removed.file else {
            continue;
        };

        // TODO all of this should probably be a member function of
        // `PastaFile`. Though `id_as_animals` could make that cumbersome
        // it occurs multiple times throughout microbin. So make sure
        // to replace it everywhere. At the very least the path generation for
        // attachments (files) and dirs should be deduplicated
        if fs::remove_file(format!(
            "{}/attachments/{}/{}",
            ARGS.data_dir,
            removed.id_as_animals(),
            pasta_file.name()
        ))
        .await
        .is_err()
        {
            log::error!("Failed to delete file {}!", pasta_file.name())
        }

        // and remove the containing directory
        if fs::remove_dir(format!(
            "{}/attachments/{}/",
            ARGS.data_dir,
            removed.id_as_animals()
        ))
        .await
        .is_err()
        {
            log::error!("Failed to delete directory {}!", pasta_file.name())
        }
    }
}

pub fn string_to_qr_svg(str: &str) -> String {
    qrcode_generator::to_svg_to_string(str, QrCodeEcc::Low, 256, None::<&str>).unwrap()
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

pub async fn encrypt_file(
    passphrase: &str,
    input_file_path: &str,
    // TODO use something like color-eyre (my personal favorite) or anyhow
) -> Result<(), Box<dyn std::error::Error>> {
    // Read the input file into memory
    let input_data = fs::read(input_file_path)
        .await
        .expect("Tried to encrypt non-existent file");

    // Create a MagicCrypt instance with the given passphrase
    let mc = new_magic_crypt!(passphrase, 256);

    // Encrypt the input data
    let ciphertext = mc.encrypt_bytes_to_bytes(&input_data[..]);

    // Write the encrypted data to a new file with the `.enc` extension
    let path = Path::new(input_file_path)
        .with_file_name("data")
        .with_extension("enc");
    fs::write(path, &ciphertext).await?;

    // Delete the original input file
    fs::remove_file(input_file_path).await?;

    Ok(())
}

pub async fn decrypt_file(
    passphrase: &str,
    input_file_path: &str,
) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
    // Read the input file into memory
    let ciphertext = fs::read(input_file_path).await?;

    // Create a MagicCrypt instance with the given passphrase
    let mc = new_magic_crypt!(passphrase, 256);
    // Encrypt the input data
    let res = mc.decrypt_bytes_to_bytes(&ciphertext[..]);

    if res.is_err() {
        return Err("Failed to decrypt file".into());
    }

    Ok(res.unwrap())
}
