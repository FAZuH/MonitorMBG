//! Storage service for managing file uploads and downloads.
//!
//! This module provides an abstraction over different storage backends (S3, Local filesystem).
//! Users of this service do not need to know which backend is being used.

use std::fmt;
use std::path::PathBuf;
use std::sync::Arc;

use bytes::Bytes;
use chrono::Utc;
use futures::StreamExt;
use object_store::ObjectStore;
use object_store::PutPayload;
use object_store::aws::AmazonS3Builder;
use object_store::path::Path as ObjectPath;
use uuid::Uuid;

use crate::config::StorageConfig;
use crate::error::AppError;

/// Errors that can occur during storage operations.
#[derive(Debug, thiserror::Error)]
pub enum StorageError {
    /// An error occurred with the underlying storage backend.
    #[error("Storage backend error: {0}")]
    BackendError(String),

    /// The requested file was not found.
    #[error("File not found: {0}")]
    NotFound(String),

    /// The file path is invalid.
    #[error("Invalid file path: {0}")]
    InvalidPath(String),

    /// The storage backend is not configured properly.
    #[error("Storage configuration error: {0}")]
    ConfigurationError(String),
}

impl From<StorageError> for AppError {
    fn from(err: StorageError) -> Self {
        match err {
            StorageError::NotFound(_) => AppError::NotFound(err.to_string()),
            StorageError::InvalidPath(_) => AppError::BadRequest(err.to_string()),
            _ => AppError::InternalServerError(err.to_string()),
        }
    }
}

impl From<object_store::Error> for StorageError {
    fn from(err: object_store::Error) -> Self {
        match err {
            object_store::Error::NotFound { path, .. } => {
                StorageError::NotFound(format!("Path: {}", path))
            }
            _ => StorageError::BackendError(err.to_string()),
        }
    }
}

impl From<std::io::Error> for StorageError {
    fn from(err: std::io::Error) -> Self {
        StorageError::BackendError(err.to_string())
    }
}

/// The type of storage backend to use.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StorageType {
    /// Local filesystem storage.
    Local,
    /// Amazon S3 or S3-compatible storage.
    S3,
}

impl fmt::Display for StorageType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            StorageType::Local => write!(f, "local"),
            StorageType::S3 => write!(f, "s3"),
        }
    }
}

impl std::str::FromStr for StorageType {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "local" => Ok(StorageType::Local),
            "s3" => Ok(StorageType::S3),
            _ => Err(format!("Unknown storage type: {}", s)),
        }
    }
}

/// Result of a successful file upload.
#[derive(Debug, Clone)]
pub struct UploadResult {
    /// The unique identifier for the uploaded file.
    pub file_id: String,
    /// The public URL where the file can be accessed.
    pub url: String,
    /// The size of the uploaded file in bytes.
    pub size: usize,
    /// The MIME type of the file.
    pub mime_type: String,
    /// The timestamp when the file was uploaded.
    pub uploaded_at: String,
}

/// Trait defining the interface for storage backends.
///
/// This trait abstracts over different storage implementations, allowing
/// the application to switch between local filesystem and S3 without
/// changing the business logic.
#[async_trait::async_trait]
pub trait StorageBackend: Send + Sync {
    /// Uploads a file to the storage backend.
    ///
    /// # Arguments
    ///
    /// * `data` - The raw bytes of the file to upload.
    /// * `file_name` - The original name of the file.
    /// * `mime_type` - The MIME type of the file.
    ///
    /// # Returns
    ///
    /// Returns an [`UploadResult`] containing metadata about the uploaded file.
    async fn upload(
        &self,
        data: Vec<u8>,
        file_name: String,
        mime_type: String,
    ) -> Result<UploadResult, StorageError>;

    /// Downloads a file from the storage backend.
    ///
    /// # Arguments
    ///
    /// * `file_id` - The unique identifier of the file to download.
    ///
    /// # Returns
    ///
    /// Returns the raw bytes of the file.
    async fn download(&self, file_id: &str) -> Result<Vec<u8>, StorageError>;

    /// Deletes a file from the storage backend.
    ///
    /// # Arguments
    ///
    /// * `file_id` - The unique identifier of the file to delete.
    async fn delete(&self, file_id: &str) -> Result<(), StorageError>;

    /// Generates a public URL for accessing a file.
    ///
    /// # Arguments
    ///
    /// * `file_id` - The unique identifier of the file.
    ///
    /// # Returns
    ///
    /// Returns a URL string that can be used to access the file.
    fn get_url(&self, file_id: &str) -> String;

    /// Checks if the storage backend is healthy.
    ///
    /// # Returns
    ///
    /// Returns `true` if the storage is accessible, `false` otherwise.
    async fn health_check(&self) -> bool;
}

/// Local filesystem storage backend.
pub struct LocalStorage {
    base_path: PathBuf,
    base_url: String,
}

impl LocalStorage {
    /// Creates a new local storage backend.
    ///
    /// # Arguments
    ///
    /// * `base_path` - The directory where files will be stored.
    /// * `base_url` - The base URL for generating public file URLs.
    pub fn new(base_path: PathBuf, base_url: String) -> Result<Self, StorageError> {
        std::fs::create_dir_all(&base_path)?;
        Ok(Self {
            base_path,
            base_url,
        })
    }

    fn get_file_path(&self, file_id: &str) -> PathBuf {
        // Organize files by date: YYYY/MM/DD/file_id
        let now = Utc::now();
        let date_path = format!(
            "{}/{}/{}/",
            now.format("%Y"),
            now.format("%m"),
            now.format("%d")
        );
        self.base_path.join(&date_path).join(file_id)
    }

    fn get_relative_path(&self, file_id: &str) -> String {
        let now = Utc::now();
        format!(
            "{}/{}/{}/{}",
            now.format("%Y"),
            now.format("%m"),
            now.format("%d"),
            file_id
        )
    }
}

#[async_trait::async_trait]
impl StorageBackend for LocalStorage {
    async fn upload(
        &self,
        data: Vec<u8>,
        _file_name: String,
        mime_type: String,
    ) -> Result<UploadResult, StorageError> {
        let file_id = Uuid::new_v4().to_string();
        let file_path = self.get_file_path(&file_id);

        // Create parent directories if they don't exist
        if let Some(parent) = file_path.parent() {
            tokio::fs::create_dir_all(parent).await?;
        }

        // Write file
        tokio::fs::write(&file_path, &data).await?;

        let relative_path = self.get_relative_path(&file_id);
        let url = format!("{}/{}", self.base_url, relative_path);

        Ok(UploadResult {
            file_id,
            url,
            size: data.len(),
            mime_type,
            uploaded_at: Utc::now().to_rfc3339(),
        })
    }

    async fn download(&self, file_id: &str) -> Result<Vec<u8>, StorageError> {
        let file_path = self.get_file_path(file_id);

        match tokio::fs::read(&file_path).await {
            Ok(data) => Ok(data),
            Err(e) if e.kind() == std::io::ErrorKind::NotFound => {
                Err(StorageError::NotFound(file_id.to_string()))
            }
            Err(e) => Err(StorageError::from(e)),
        }
    }

    async fn delete(&self, file_id: &str) -> Result<(), StorageError> {
        let file_path = self.get_file_path(file_id);

        match tokio::fs::remove_file(&file_path).await {
            Ok(_) => Ok(()),
            Err(e) if e.kind() == std::io::ErrorKind::NotFound => {
                Err(StorageError::NotFound(file_id.to_string()))
            }
            Err(e) => Err(StorageError::from(e)),
        }
    }

    fn get_url(&self, file_id: &str) -> String {
        let relative_path = self.get_relative_path(file_id);
        format!("{}/{}", self.base_url, relative_path)
    }

    async fn health_check(&self) -> bool {
        tokio::fs::metadata(&self.base_path).await.is_ok()
    }
}

/// S3 storage backend.
pub struct S3Storage {
    client: Box<dyn ObjectStore>,
    /// The S3 bucket name (stored for reference).
    #[allow(dead_code)]
    bucket: String,
    base_url: String,
    /// The AWS region (stored for reference).
    #[allow(dead_code)]
    region: String,
}

impl S3Storage {
    /// Creates a new S3 storage backend.
    ///
    /// # Arguments
    ///
    /// * `config` - The S3 configuration including credentials and bucket info.
    pub fn new(config: &StorageConfig) -> Result<Self, StorageError> {
        let bucket = config
            .bucket
            .as_ref()
            .ok_or_else(|| StorageError::ConfigurationError("S3 bucket not configured".into()))?;

        let region = config
            .region
            .clone()
            .unwrap_or_else(|| "us-east-1".to_string());

        let mut builder = AmazonS3Builder::new()
            .with_bucket_name(bucket)
            .with_region(&region);

        // Configure credentials
        if let Some(access_key) = &config.access_key {
            builder = builder.with_access_key_id(access_key);
        }

        if let Some(secret_key) = &config.secret_key {
            builder = builder.with_secret_access_key(secret_key);
        }

        // Configure endpoint for S3-compatible services (MinIO, etc.)
        if let Some(endpoint) = &config.endpoint {
            builder = builder.with_endpoint(endpoint);
        }

        let client = builder.build().map_err(|e| {
            StorageError::ConfigurationError(format!("Failed to build S3 client: {}", e))
        })?;

        // Determine base URL
        let base_url = config
            .base_url
            .clone()
            .unwrap_or_else(|| format!("https://{}.s3.{}.amazonaws.com", bucket, region));

        Ok(Self {
            client: Box::new(client),
            bucket: bucket.clone(),
            base_url,
            region,
        })
    }

    fn get_object_path(&self, file_id: &str) -> ObjectPath {
        // Organize files by date: uploads/YYYY/MM/DD/file_id
        let now = Utc::now();
        let path = format!(
            "uploads/{}/{}/{}/{}",
            now.format("%Y"),
            now.format("%m"),
            now.format("%d"),
            file_id
        );
        ObjectPath::from(path)
    }
}

#[async_trait::async_trait]
impl StorageBackend for S3Storage {
    async fn upload(
        &self,
        data: Vec<u8>,
        _file_name: String,
        mime_type: String,
    ) -> Result<UploadResult, StorageError> {
        let file_id = Uuid::new_v4().to_string();
        let path = self.get_object_path(&file_id);

        let size = data.len();
        let payload = PutPayload::from(Bytes::from(data));

        // Upload to S3
        self.client.put(&path, payload).await?;

        // Construct URL
        let url = format!("{}/{}", self.base_url, path);

        Ok(UploadResult {
            file_id,
            url,
            size,
            mime_type,
            uploaded_at: Utc::now().to_rfc3339(),
        })
    }

    async fn download(&self, file_id: &str) -> Result<Vec<u8>, StorageError> {
        let path = self.get_object_path(file_id);

        let result = self.client.get(&path).await?;
        let bytes = result.bytes().await?;

        Ok(bytes.to_vec())
    }

    async fn delete(&self, file_id: &str) -> Result<(), StorageError> {
        let path = self.get_object_path(file_id);
        self.client.delete(&path).await?;
        Ok(())
    }

    fn get_url(&self, file_id: &str) -> String {
        let path = self.get_object_path(file_id);
        format!("{}/{}", self.base_url, path)
    }

    async fn health_check(&self) -> bool {
        // Try to list objects to verify connectivity
        match self.client.list(None).next().await {
            Some(_) => true,
            None => {
                // If no objects, try to check if we can at least connect
                // by listing with a prefix that likely doesn't exist
                self.client
                    .list(Some(&ObjectPath::from("health_check_test")))
                    .next()
                    .await
                    .is_none()
            }
        }
    }
}

/// Storage service that abstracts over different storage backends.
///
/// This is the main interface that business logic should use. It provides
/// a unified API regardless of whether files are stored locally or in S3.
pub struct StorageService {
    backend: Arc<dyn StorageBackend>,
    storage_type: StorageType,
}

impl StorageService {
    /// Creates a new storage service from configuration.
    ///
    /// # Arguments
    ///
    /// * `config` - The storage configuration specifying backend type and settings.
    ///
    /// # Returns
    ///
    /// Returns a new [`StorageService`] instance.
    ///
    /// # Errors
    ///
    /// Returns an error if the storage backend cannot be initialized.
    pub fn new(config: &StorageConfig) -> Result<Self, StorageError> {
        let storage_type = config
            .storage_type
            .parse::<StorageType>()
            .map_err(StorageError::ConfigurationError)?;

        let backend: Arc<dyn StorageBackend> = match storage_type {
            StorageType::Local => {
                let base_path = config
                    .local_path
                    .clone()
                    .map(PathBuf::from)
                    .unwrap_or_else(|| PathBuf::from("./uploads"));
                let base_url = config
                    .base_url
                    .clone()
                    .unwrap_or_else(|| "http://localhost:3000/uploads".to_string());
                Arc::new(LocalStorage::new(base_path, base_url)?)
            }
            StorageType::S3 => Arc::new(S3Storage::new(config)?),
        };

        Ok(Self {
            backend,
            storage_type,
        })
    }

    /// Uploads a file to storage.
    ///
    /// # Arguments
    ///
    /// * `data` - The raw bytes of the file.
    /// * `file_name` - The original file name.
    /// * `mime_type` - The MIME type of the file.
    ///
    /// # Returns
    ///
    /// Returns an [`UploadResult`] with metadata about the uploaded file.
    pub async fn upload(
        &self,
        data: Vec<u8>,
        file_name: String,
        mime_type: String,
    ) -> Result<UploadResult, StorageError> {
        self.backend.upload(data, file_name, mime_type).await
    }

    /// Downloads a file from storage.
    ///
    /// # Arguments
    ///
    /// * `file_id` - The unique identifier of the file.
    ///
    /// # Returns
    ///
    /// Returns the raw bytes of the file.
    pub async fn download(&self, file_id: &str) -> Result<Vec<u8>, StorageError> {
        self.backend.download(file_id).await
    }

    /// Deletes a file from storage.
    ///
    /// # Arguments
    ///
    /// * `file_id` - The unique identifier of the file.
    pub async fn delete(&self, file_id: &str) -> Result<(), StorageError> {
        self.backend.delete(file_id).await
    }

    /// Gets the public URL for a file.
    ///
    /// # Arguments
    ///
    /// * `file_id` - The unique identifier of the file.
    ///
    /// # Returns
    ///
    /// Returns a URL string.
    pub fn get_url(&self, file_id: &str) -> String {
        self.backend.get_url(file_id)
    }

    /// Checks if the storage backend is healthy.
    ///
    /// # Returns
    ///
    /// Returns `true` if storage is accessible.
    pub async fn health_check(&self) -> bool {
        self.backend.health_check().await
    }

    /// Returns the type of storage backend being used.
    pub fn storage_type(&self) -> StorageType {
        self.storage_type
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_local_storage_upload_and_download() {
        let temp_dir = std::env::temp_dir().join(format!("test_storage_{}", Uuid::new_v4()));
        let storage = LocalStorage::new(
            temp_dir.clone(),
            "http://localhost:3000/uploads".to_string(),
        )
        .unwrap();

        let test_data = b"Hello, World!".to_vec();
        let result = storage
            .upload(
                test_data.clone(),
                "test.txt".to_string(),
                "text/plain".to_string(),
            )
            .await
            .unwrap();

        assert_eq!(result.size, test_data.len());
        assert!(result.url.contains(&result.file_id));

        // Download and verify
        let downloaded = storage.download(&result.file_id).await.unwrap();
        assert_eq!(downloaded, test_data);

        // Cleanup
        let _ = tokio::fs::remove_dir_all(&temp_dir).await;
    }

    #[tokio::test]
    async fn test_local_storage_delete() {
        let temp_dir = std::env::temp_dir().join(format!("test_storage_{}", Uuid::new_v4()));
        let storage = LocalStorage::new(
            temp_dir.clone(),
            "http://localhost:3000/uploads".to_string(),
        )
        .unwrap();

        let test_data = b"Test data".to_vec();
        let result = storage
            .upload(test_data, "test.txt".to_string(), "text/plain".to_string())
            .await
            .unwrap();

        // Delete the file
        storage.delete(&result.file_id).await.unwrap();

        // Try to download - should fail
        let download_result = storage.download(&result.file_id).await;
        assert!(matches!(download_result, Err(StorageError::NotFound(_))));

        // Cleanup
        let _ = tokio::fs::remove_dir_all(&temp_dir).await;
    }

    #[test]
    fn test_storage_type_from_str() {
        assert_eq!("local".parse::<StorageType>().unwrap(), StorageType::Local);
        assert_eq!("s3".parse::<StorageType>().unwrap(), StorageType::S3);
        assert!("unknown".parse::<StorageType>().is_err());
    }

    #[test]
    fn test_storage_type_display() {
        assert_eq!(StorageType::Local.to_string(), "local");
        assert_eq!(StorageType::S3.to_string(), "s3");
    }
}
