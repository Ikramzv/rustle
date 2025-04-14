use std::sync::Arc;

use axum::extract::Extension;

use crate::core::services::{mail::MailService, s3::S3Service};

pub type MailServiceExt = Extension<Arc<MailService>>;
pub type S3ServiceExt = Extension<Arc<S3Service>>;
