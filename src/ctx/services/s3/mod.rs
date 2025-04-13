use std::{
    io::{Cursor, Read},
    str::FromStr,
    sync::LazyLock,
};

use axum::body::Bytes;
use chrono::Utc;
use s3::{Bucket, creds::Credentials, error::S3Error};

pub fn get_bucket() -> Bucket {
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

    *bucket.unwrap()
}

pub async fn upload_file_to_s3(
    s3: &Bucket,
    data: Bytes,
    file_name: String,
    content_type: String,
) -> Result<String, S3Error> {
    let mut reader = Cursor::new(data);

    let (name, extension) = file_name.split_once(".").unwrap_or((&file_name, ""));

    let s3_path = format!(
        "uploads/{}_{}.{}",
        name,
        Utc::now().timestamp().to_string(),
        extension
    );

    s3.put_object_stream_with_content_type(&mut reader, s3_path.clone(), content_type)
        .await?;

    let url = format!(
        "https://{}.s3.{}.amazonaws.com/{}",
        s3.name(),
        s3.region(),
        s3_path
    );

    Ok(url)
}
