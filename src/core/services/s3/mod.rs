use std::{io::Cursor, str::FromStr};

use axum::body::Bytes;
use chrono::Utc;
use s3::{Bucket, creds::Credentials, error::S3Error};

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

    pub async fn upload_file(
        &self,
        data: Bytes,
        file_name: String,
        content_type: String,
    ) -> Result<String, S3Error> {
        let mut reader = Cursor::new(data);

        let (name, extension) = file_name.split_once(".").unwrap_or((&file_name, ""));

        let s3_path = format!(
            "uploads/{}_{}.{}",
            name,
            Utc::now().timestamp_millis().to_string(),
            extension
        );

        self.bucket
            .put_object_stream_with_content_type(&mut reader, s3_path.clone(), content_type)
            .await?;

        let url = format!(
            "https://{}.s3.{}.amazonaws.com/{}",
            self.bucket.name(),
            self.bucket.region(),
            s3_path
        );

        Ok(url)
    }
}
