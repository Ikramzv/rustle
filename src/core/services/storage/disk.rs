use std::{
    fs::DirBuilder,
    io::{Cursor, Read},
    path::PathBuf,
};

use crate::{
    constants,
    core::services::storage::{Storage, StorageError},
};
use async_trait::async_trait;
use axum::body::Bytes;
use chrono::Utc;
use tokio::{
    fs::File,
    io::{AsyncWriteExt, BufWriter},
};

use super::UploadOptions;

pub struct DiskStorage {
    path: PathBuf,
}

impl DiskStorage {
    pub fn new() -> Self {
        let storage_path = constants::DISK_STORAGE_PATH.to_string();

        let mut path = std::env::current_dir().unwrap_or_default();

        path.push(storage_path);

        let exists = match std::fs::exists(&path) {
            Ok(v) => v,
            Err(_) => false,
        };

        match exists {
            true => tracing::info!("Disk storage path already exists at {}", path.display()),
            false => Self::create_storage_dir(&path),
        }

        Self { path }
    }

    fn get_full_path(&self, file_name: &str) -> String {
        format!("{}/{}", self.path.display(), file_name)
    }

    fn create_storage_dir(path: &PathBuf) {
        match DirBuilder::new().create(&path) {
            Ok(_) => tracing::info!("Disk storage path created at {}", path.display()),
            Err(e) => tracing::error!("Failed to create disk storage path: {}", e),
        }
    }
}

#[async_trait]
impl Storage for DiskStorage {
    async fn upload_file(
        &self,
        data: Bytes,
        upload_options: UploadOptions,
    ) -> Result<String, StorageError> {
        let file_name = upload_options.file_name;

        let (name, extension) = file_name.split_once(".").unwrap_or((&file_name, ""));

        let full_path = self.get_full_path(&format!(
            "{}_{}.{}",
            name,
            Utc::now().timestamp_millis(),
            extension
        ));

        let file = File::create(&full_path)
            .await
            .inspect_err(|e| tracing::error!(?e))
            .map_err(StorageError::Io)?;

        let mut cursor = Cursor::new(data);

        const CHUNK_SIZE: usize = 1024 * 1024; // 1 MB

        let mut buf = [0; CHUNK_SIZE];

        let mut writer = BufWriter::new(file);

        while let Ok(bytes_read) = cursor.read(&mut buf) {
            if bytes_read == 0 {
                break;
            }

            let bytes = &buf[..bytes_read];

            writer.write(bytes).await.map_err(StorageError::Io)?;
        }

        let url = format!("{}/{}", constants::SERVER_URL.to_string(), full_path);

        Ok(url)
    }
}
