use std::{fmt, sync::Arc};

use ::s3::error::S3Error;
use async_trait::async_trait;
use axum::body::Bytes;
use chrono::Utc;
use disk::DiskStorage;
use s3::S3Service;

pub mod disk;
pub mod s3;

#[derive(Debug)]
pub enum StorageError {
    S3(S3Error),
    Io(std::io::Error),
    InvalidUploadOptions(String),
}

impl std::error::Error for StorageError {}

impl fmt::Display for StorageError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            StorageError::S3(e) => write!(f, "[Storage Error] S3: {}", e),
            StorageError::Io(e) => write!(f, "[Storage Error] Io: {}", e),
            StorageError::InvalidUploadOptions(e) => {
                write!(f, "[Storage Error] Invalid Upload Options: {}", e)
            }
        }
    }
}

pub struct UploadOptions {
    pub file_name: String,
    pub content_type: Option<String>,
}

impl UploadOptions {
    pub fn new() -> Self {
        let file_name = format!("{}", Utc::now().timestamp_millis());

        Self {
            file_name,
            content_type: None,
        }
    }

    pub fn set_file_name(mut self, file_name: String) -> Self {
        self.file_name = file_name;
        self
    }

    pub fn set_content_type(mut self, content_type: String) -> Self {
        self.content_type = Some(content_type);
        self
    }
}

#[async_trait]
pub trait Storage {
    async fn upload_file(
        &self,
        data: Bytes,
        upload_options: UploadOptions,
    ) -> Result<String, StorageError>;
}

pub struct StorageProvider {
    pub storage: TStorage,
}

impl StorageProvider {
    pub fn new() -> Self {
        let storage_type = Self::get_storage_type();

        let storage: TStorage = match storage_type {
            StorageType::S3 => Arc::new(S3Service::new()),
            StorageType::Disk => Arc::new(DiskStorage::new()),
        };

        Self { storage }
    }

    fn get_storage_type() -> StorageType {
        match std::env::var("STORAGE_TYPE") {
            Ok(storage_type) => match storage_type.as_str() {
                "s3" => StorageType::S3,
                "disk" => StorageType::Disk,
                _ => StorageType::Disk,
            },
            Err(_) => StorageType::Disk,
        }
    }
}

enum StorageType {
    Disk,
    S3,
}

type TStorage = Arc<dyn Storage + Send + Sync + 'static>;
