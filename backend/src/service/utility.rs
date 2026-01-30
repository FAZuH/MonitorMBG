//! Utility services including file uploads and health checks.

use std::sync::Arc;

use serde::Serialize;

use crate::database::Database;
use crate::database::table::Table;
use crate::error::AppError;
use crate::service::storage::StorageError;
use crate::service::storage::StorageService;

/// Response structure for a single image upload.
#[derive(Debug, Serialize)]
pub struct ImageUploadResponse {
    /// Whether the upload was successful.
    pub success: bool,
    /// The public URL where the image can be accessed.
    pub url: String,
    /// The original file name.
    #[serde(rename = "fileName")]
    pub file_name: String,
    /// The size of the uploaded file in bytes.
    pub size: usize,
    /// The MIME type of the file.
    #[serde(rename = "mimeType")]
    pub mime_type: String,
    /// The width of the image in pixels (if available).
    pub width: i32,
    /// The height of the image in pixels (if available).
    pub height: i32,
    /// The timestamp when the file was uploaded.
    #[serde(rename = "uploadedAt")]
    pub uploaded_at: String,
}

/// Response structure for multiple image uploads.
#[derive(Debug, Serialize)]
pub struct MultiImageUploadResponse {
    /// Whether the overall operation was successful.
    pub success: bool,
    /// The number of files successfully uploaded.
    pub uploaded: i32,
    /// The number of files that failed to upload.
    pub failed: i32,
    /// The results for each file upload attempt.
    pub results: Vec<ImageUploadResult>,
}

/// Result structure for a single file in a batch upload.
#[derive(Debug, Serialize)]
pub struct ImageUploadResult {
    /// The public URL where the file can be accessed.
    pub url: String,
    /// The original file name.
    #[serde(rename = "fileName")]
    pub file_name: String,
    /// The size of the uploaded file in bytes.
    pub size: usize,
}

/// Response structure for health check.
#[derive(Debug, Serialize)]
pub struct HealthCheckResponse {
    /// The overall status of the system.
    pub status: String,
    /// The application version.
    pub version: String,
    /// The timestamp of the health check.
    pub timestamp: String,
    /// The health status of individual services.
    pub services: ServicesHealth,
}

/// Health status of individual services.
#[derive(Debug, Serialize)]
pub struct ServicesHealth {
    /// The health status of the database.
    pub database: String,
    /// The health status of the storage backend.
    pub storage: String,
    /// The health status of the WhatsApp service.
    pub whatsapp: String,
}

/// Service for utility operations like image uploads and system health.
pub struct UtilityService {
    db: Arc<Database>,
    storage: Arc<StorageService>,
}

impl UtilityService {
    /// Creates a new `UtilityService`.
    ///
    /// # Arguments
    ///
    /// * `db` - The database instance.
    /// * `storage` - The storage service for file operations.
    pub fn new(db: Arc<Database>, storage: Arc<StorageService>) -> Self {
        Self { db, storage }
    }

    /// Uploads an image to the storage system.
    ///
    /// # Arguments
    ///
    /// * `file_name` - The original name of the file.
    /// * `data` - The raw bytes of the image file.
    /// * `mime_type` - The MIME type of the file.
    ///
    /// # Returns
    ///
    /// Returns an [`ImageUploadResponse`] with metadata about the uploaded image.
    ///
    /// # Errors
    ///
    /// Returns an error if the upload fails.
    pub async fn upload_image(
        &self,
        file_name: String,
        data: Vec<u8>,
        mime_type: String,
    ) -> Result<ImageUploadResponse, AppError> {
        let result = self
            .storage
            .upload(data, file_name.clone(), mime_type.clone())
            .await
            .map_err(|e| match e {
                StorageError::NotFound(_) => AppError::NotFound(e.to_string()),
                StorageError::InvalidPath(_) => AppError::BadRequest(e.to_string()),
                _ => AppError::InternalServerError(e.to_string()),
            })?;

        // TODO: Extract actual image dimensions using an image processing library
        // For now, we return placeholder dimensions
        Ok(ImageUploadResponse {
            success: true,
            url: result.url,
            file_name,
            size: result.size,
            mime_type,
            width: 1920,
            height: 1080,
            uploaded_at: result.uploaded_at,
        })
    }

    /// Uploads multiple images to the storage system.
    ///
    /// # Arguments
    ///
    /// * `files` - A vector of tuples containing (file_name, data, content_type).
    ///
    /// # Returns
    ///
    /// Returns a [`MultiImageUploadResponse`] with results for each file.
    ///
    /// # Errors
    ///
    /// Returns an error if the operation fails catastrophically.
    pub async fn upload_multiple_images(
        &self,
        files: Vec<(String, Vec<u8>, String)>,
    ) -> Result<MultiImageUploadResponse, AppError> {
        let mut results = Vec::new();
        let mut uploaded = 0;
        let mut failed = 0;

        for (file_name, data, content_type) in files {
            match self
                .upload_image(file_name.clone(), data, content_type)
                .await
            {
                Ok(response) => {
                    uploaded += 1;
                    results.push(ImageUploadResult {
                        url: response.url,
                        file_name: response.file_name,
                        size: response.size,
                    });
                }
                Err(_) => {
                    failed += 1;
                    results.push(ImageUploadResult {
                        url: String::new(),
                        file_name,
                        size: 0,
                    });
                }
            }
        }

        Ok(MultiImageUploadResponse {
            success: true,
            uploaded,
            failed,
            results,
        })
    }

    /// Performs a health check on all system services.
    ///
    /// # Returns
    ///
    /// Returns a [`HealthCheckResponse`] with the status of each service.
    ///
    /// # Errors
    ///
    /// Returns an error if the health check cannot be performed.
    pub async fn health_check(&self) -> Result<HealthCheckResponse, AppError> {
        // Check DB connection by running a simple query through the database
        let db_status = match self.db.user_table.select_all().await {
            Ok(_) => "healthy",
            Err(_) => "unhealthy",
        };

        // Check storage health
        let storage_status = if self.storage.health_check().await {
            "healthy"
        } else {
            "unhealthy"
        };

        Ok(HealthCheckResponse {
            status: "healthy".to_string(),
            version: "1.0.0".to_string(),
            timestamp: chrono::Utc::now().to_rfc3339(),
            services: ServicesHealth {
                database: db_status.to_string(),
                storage: storage_status.to_string(),
                whatsapp: "healthy".to_string(), // Mock
            },
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Note: These tests would need a mock database to run properly
    // For now, they serve as documentation of expected behavior

    #[test]
    fn test_image_upload_response_structure() {
        let response = ImageUploadResponse {
            success: true,
            url: "http://example.com/image.jpg".to_string(),
            file_name: "test.jpg".to_string(),
            size: 1024,
            mime_type: "image/jpeg".to_string(),
            width: 1920,
            height: 1080,
            uploaded_at: chrono::Utc::now().to_rfc3339(),
        };

        assert!(response.success);
        assert_eq!(response.size, 1024);
    }

    #[test]
    fn test_multi_upload_response_structure() {
        let response = MultiImageUploadResponse {
            success: true,
            uploaded: 2,
            failed: 1,
            results: vec![
                ImageUploadResult {
                    url: "http://example.com/1.jpg".to_string(),
                    file_name: "1.jpg".to_string(),
                    size: 1024,
                },
                ImageUploadResult {
                    url: "http://example.com/2.jpg".to_string(),
                    file_name: "2.jpg".to_string(),
                    size: 2048,
                },
            ],
        };

        assert_eq!(response.uploaded, 2);
        assert_eq!(response.failed, 1);
        assert_eq!(response.results.len(), 2);
    }

    #[test]
    fn test_health_check_response_structure() {
        let response = HealthCheckResponse {
            status: "healthy".to_string(),
            version: "1.0.0".to_string(),
            timestamp: chrono::Utc::now().to_rfc3339(),
            services: ServicesHealth {
                database: "healthy".to_string(),
                storage: "healthy".to_string(),
                whatsapp: "healthy".to_string(),
            },
        };

        assert_eq!(response.status, "healthy");
        assert_eq!(response.services.database, "healthy");
    }
}
