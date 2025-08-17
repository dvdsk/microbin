use crate::util::animalnumbers::to_animal_names;
use crate::util::hashids::to_hashids;
use crate::util::syntaxhighlighter::html_highlight;
use bytesize::ByteSize;
use chrono::{Datelike, Local, TimeZone, Timelike};
use db::entities::pasta::PastaEntity;
use serde::{Deserialize, Serialize};
use std::fmt;
use std::path::Path;
use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Serialize, Deserialize, PartialEq, Debug, Eq, Clone)]
pub struct PastaFile {
    pub name: String,
    pub size: ByteSize,
}

impl PastaFile {
    pub fn from_unsanitized(path: &str) -> Result<Self, &'static str> {
        let path = Path::new(path);
        let name = path.file_name().ok_or("Path did not contain a file name")?;
        let name = name.to_string_lossy().replace(' ', "_");
        Ok(Self {
            name,
            size: ByteSize::b(0),
        })
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn is_image(&self) -> bool {
        let lowercase_name = self.name.to_lowercase();
        let extensions = [
            ".jpg", ".jpeg", ".png", ".gif", ".bmp", ".webp", ".ico", ".svg", ".tiff", ".tif",
            ".jfif", ".pjpeg", ".pjp", ".avif", ".jxl", ".heif",
        ];
        extensions.iter().any(|&ext| lowercase_name.ends_with(ext))
    }

    pub fn is_video(&self) -> bool {
        let lowercase_name = self.name.to_lowercase();
        let extensions = [
            ".mp4", ".mov", ".wmv", ".webm", ".avi", ".flv", ".mkv", ".mts",
        ];
        extensions.iter().any(|&ext| lowercase_name.ends_with(ext))
    }

    pub fn embeddable(&self) -> bool {
        self.is_image() || self.is_video()
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Pasta {
    pub id: u64,
    pub content: String,
    pub file: Option<PastaFile>,
    pub extension: String,
    pub private: bool,
    pub readonly: bool,
    pub editable: bool,
    pub hide_read_count: bool,
    pub encrypt_server: bool,
    pub encrypt_client: bool,
    pub encrypted_key: Option<String>,
    pub created: i64,
    pub expiration: i64,
    pub last_read: i64,
    pub read_count: u64,
    pub burn_after_reads: u64,
    pub pasta_type: String,
}

impl From<Pasta> for PastaEntity {
    fn from(pasta: Pasta) -> Self {
        PastaEntity {
            id: pasta.id,
            content: pasta.content,
            file_name: pasta.file.clone().map(|f| f.name),
            file_size: pasta.file.map(|f| f.size.as_u64()).map(|u| u as i64),
            extension: pasta.extension,
            read_only: pasta.readonly,
            private: pasta.private,
            editable: i32::from(pasta.editable),
            encrypt_server: i32::from(pasta.encrypt_server),
            encrypt_client: i32::from(pasta.encrypt_client),
            encrypted_key: pasta.encrypted_key,
            created: pasta.created,
            expiration: pasta.expiration,
            last_read: pasta.last_read,
            read_count: pasta.read_count as i64,
            burn_after_reads: pasta.burn_after_reads as i64,
            pasta_type: pasta.pasta_type,
            hide_read_count: pasta.hide_read_count,
        }
    }
}

impl From<&PastaEntity> for Pasta {
    fn from(value: &PastaEntity) -> Self {
        let cloned_pasta_entity = value.clone();
        Pasta::from(cloned_pasta_entity)
    }
}

impl From<PastaEntity> for Pasta {
    fn from(pasta: PastaEntity) -> Self {
        Pasta {
            id: pasta.id,
            content: pasta.content,
            file: pasta.file_name.map(|name| PastaFile {
                name,
                size: ByteSize::b(pasta.file_size.unwrap_or(0) as u64),
            }),
            extension: pasta.extension,
            private: pasta.private,
            readonly: pasta.read_only,
            editable: pasta.editable != 0,
            hide_read_count: pasta.hide_read_count,
            encrypt_server: pasta.encrypt_server != 0,
            encrypt_client: pasta.encrypt_client != 0,
            encrypted_key: pasta.encrypted_key,
            created: pasta.created,
            expiration: pasta.expiration,
            last_read: pasta.last_read,
            read_count: pasta.read_count as u64,
            burn_after_reads: pasta.burn_after_reads as u64,
            pasta_type: pasta.pasta_type,
        }
    }
}

impl Pasta {
    pub fn id_as_animals(&self, hash_ids: &bool) -> String {
        if *hash_ids {
            to_hashids(self.id)
        } else {
            to_animal_names(self.id)
        }
    }

    pub fn has_file(&self) -> bool {
        self.file.is_some()
    }

    pub fn total_size_as_string(&self) -> String {
        let total_size_bytes = if let Some(file) = &self.file {
            file.size.as_u64() as usize + self.content.len()
        } else {
            self.content.len()
        };

        if total_size_bytes < 1024 {
            format!("{total_size_bytes} B")
        } else if total_size_bytes < 1024 * 1024 {
            format!("{} KB", total_size_bytes / 1024)
        } else if total_size_bytes < 1024 * 1024 * 1024 {
            format!("{} MB", total_size_bytes / (1024 * 1024))
        } else {
            format!("{} GB", total_size_bytes / (1024 * 1024 * 1024))
        }
    }

    pub fn file_embeddable(&self) -> bool {
        let first_file_result = match self.file {
            Some(ref file) => file.embeddable(),
            None => false,
        };

        first_file_result && !(self.encrypt_server || self.encrypt_client)
    }

    pub fn created_as_string(&self) -> String {
        Local
            .timestamp_opt(self.created, 0)
            .map(|date| {
                format!(
                    "{:02}-{:02} {:02}:{:02}",
                    date.month(),
                    date.day(),
                    date.hour(),
                    date.minute(),
                )
            })
            .earliest()
            .unwrap_or_else(|| {
                log::error!("Failed to process created date");
                String::from("Unknow")
            })
    }

    pub fn expiration_as_string(&self) -> String {
        if self.expiration == 0 {
            String::from("Never")
        } else {
            Local
                .timestamp_opt(self.expiration, 0)
                .map(|date| {
                    format!(
                        "{:02}-{:02} {:02}:{:02}",
                        date.month(),
                        date.day(),
                        date.hour(),
                        date.minute(),
                    )
                })
                .earliest()
                .unwrap_or_else(|| {
                    log::error!("Failed to process expiration");
                    String::from("Never")
                })
        }
    }

    pub fn last_read_time_ago_as_string(&self) -> String {
        // get current unix time in seconds
        let timenow: i64 = match SystemTime::now().duration_since(UNIX_EPOCH) {
            Ok(n) => n.as_secs(),
            Err(_) => {
                log::error!("SystemTime before UNIX EPOCH!");
                0
            }
        } as i64;

        // get seconds since last read and convert it to days
        let days = ((timenow - self.last_read) / 86400) as u16;
        if days > 1 {
            return format!("{days} days ago");
        };

        // it's less than 1 day, let's do hours then
        let hours = ((timenow - self.last_read) / 3600) as u16;
        if hours > 1 {
            return format!("{hours} hours ago");
        };

        // it's less than 1 hour, let's do minutes then
        let minutes = ((timenow - self.last_read) / 60) as u16;
        if minutes > 1 {
            return format!("{minutes} minutes ago");
        };

        // it's less than 1 minute, let's do seconds then
        let seconds = (timenow - self.last_read) as u16;
        if seconds > 1 {
            return format!("{seconds} seconds ago");
        };

        // it's less than 1 second?????
        String::from("just now")
    }

    pub fn short_last_read_time_ago_as_string(&self) -> String {
        // get current unix time in seconds
        let timenow: i64 = match SystemTime::now().duration_since(UNIX_EPOCH) {
            Ok(n) => n.as_secs(),
            Err(_) => {
                log::error!("SystemTime before UNIX EPOCH!");
                0
            }
        } as i64;

        // get seconds since last read and convert it to days
        let days = ((timenow - self.last_read) / 86400) as u16;
        if days > 1 {
            return format!("{days} d ago");
        };

        // it's less than 1 day, let's do hours then
        let hours = ((timenow - self.last_read) / 3600) as u16;
        if hours > 1 {
            return format!("{hours} h ago");
        };

        // it's less than 1 hour, let's do minutes then
        let minutes = ((timenow - self.last_read) / 60) as u16;
        if minutes > 1 {
            return format!("{minutes} m ago");
        };

        // it's less than 1 minute, let's do seconds then
        let seconds = (timenow - self.last_read) as u16;
        if seconds > 1 {
            return format!("{seconds} s ago");
        };

        // it's less than 1 second?????
        String::from("just now")
    }

    pub fn last_read_days_ago(&self) -> u16 {
        // get current unix time in seconds
        let timenow: i64 = match SystemTime::now().duration_since(UNIX_EPOCH) {
            Ok(n) => n.as_secs(),
            Err(_) => {
                log::error!("SystemTime before UNIX EPOCH!");
                0
            }
        } as i64;

        // get seconds since last read and convert it to days
        ((timenow - self.last_read) / 86400) as u16
    }

    pub fn content_syntax_highlighted(&self) -> String {
        html_highlight(&self.content, &self.extension)
    }

    pub fn content_not_highlighted(&self) -> String {
        html_highlight(&self.content, "txt")
    }

    pub fn content_escaped(&self) -> String {
        html_escape::encode_text(
            &self
                .content
                .replace('\\', "\\\\")
                .replace('`', "\\`")
                .replace('$', "\\$"),
        )
        .to_string()
    }
}

impl fmt::Display for Pasta {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.content)
    }
}
