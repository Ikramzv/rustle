use std::sync::Arc;

use axum::extract::Extension;

use crate::core::services::{mail::MailService, storage::StorageProvider};

pub type MailServiceExt = Extension<Arc<MailService>>;
pub type StorageServiceExt = Extension<Arc<StorageProvider>>;
