use std::fs::DirBuilder;

use crate::{
    config::CONFIG,
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
    path: String,
}

impl DiskStorage {
    pub fn new() -> Self {
        let path = constants::DISK_STORAGE_PATH.to_string();

        match std::fs::exists(&path) {
            Ok(_) => (),
            Err(_) => match DirBuilder::new().create(&path) {
                Ok(_) => (),
                Err(e) => panic!("Failed to create disk storage path: {}", e),
            },
        }

        Self { path }
    }

    fn get_full_path(&self, file_name: &str) -> String {
        format!("{}/{}", self.path, file_name)
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

        let mut writer = BufWriter::new(file);

        writer.write_all(&data).await.map_err(StorageError::Io)?;

        let url = format!("{}/{}", constants::SERVER_URL.to_string(), full_path);

        Ok(url)
    }
}
