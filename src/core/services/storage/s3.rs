use std::{io::Cursor, str::FromStr};

use async_trait::async_trait;
use axum::body::Bytes;
use chrono::Utc;
use s3::{Bucket, creds::Credentials};

use super::{Storage, StorageError, UploadOptions};

pub struct S3Service {
    bucket: Bucket,
}

impl S3Service {
    pub fn new() -> Self {
        let access_key = std::env::var("AWS_ACCESS_KEY").unwrap();
        let secret_key = std::env::var("AWS_SECRET_KEY").unwrap();
        let bucket_name = std::env::var("AWS_BUCKET_NAME").unwrap();
        let region = std::env::var("AWS_REGION").unwrap();
        let credentials =
            Credentials::new(Some(&access_key), Some(&secret_key), None, None, None).unwrap();
        let bucket = Bucket::new(
            &bucket_name,
            s3::Region::from_str(&region).unwrap(),
            credentials,
        );

        Self {
            bucket: *bucket.unwrap(),
        }
    }
}

#[async_trait]
impl Storage for S3Service {
    async fn upload_file(
        &self,
        data: Bytes,
        upload_options: UploadOptions,
    ) -> Result<String, StorageError> {
        let mut reader: Cursor<Bytes> = Cursor::new(data);

        let file_name = upload_options.file_name;
        let content_type =
            upload_options
                .content_type
                .ok_or(StorageError::InvalidUploadOptions(
                    "Content type is required".into(),
                ))?;

        let (name, extension) = file_name.split_once(".").unwrap_or((&file_name, ""));

        let s3_path = format!(
            "uploads/{}_{}.{}",
            name,
            Utc::now().timestamp_millis().to_string(),
            extension
        );

        self.bucket
            .put_object_stream_with_content_type(&mut reader, s3_path.clone(), content_type)
            .await
            .map_err(StorageError::S3)?;

        let url = format!(
            "https://{}.s3.{}.amazonaws.com/{}",
            self.bucket.name(),
            self.bucket.region(),
            s3_path
        );

        Ok(url)
    }
}
