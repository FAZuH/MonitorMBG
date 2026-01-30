//! Utility services including file uploads and health checks.

use serde::Serialize;
use sqlx::PgPool;
use uuid::Uuid;

use crate::error::AppError;

#[derive(Debug, Serialize)]
pub struct ImageUploadResponse {
    pub success: bool,
    pub url: String,
    #[serde(rename = "fileName")]
    pub file_name: String,
    pub size: usize,
    #[serde(rename = "mimeType")]
    pub mime_type: String,
    pub width: i32,
    pub height: i32,
    #[serde(rename = "uploadedAt")]
    pub uploaded_at: String,
}

#[derive(Debug, Serialize)]
pub struct MultiImageUploadResponse {
    pub success: bool,
    pub uploaded: i32,
    pub failed: i32,
    pub results: Vec<ImageUploadResult>,
}

#[derive(Debug, Serialize)]
pub struct ImageUploadResult {
    pub url: String,
    #[serde(rename = "fileName")]
    pub file_name: String,
    pub size: usize,
}

#[derive(Debug, Serialize)]
pub struct HealthCheckResponse {
    pub status: String,
    pub version: String,
    pub timestamp: String,
    pub services: ServicesHealth,
}

#[derive(Debug, Serialize)]
pub struct ServicesHealth {
    pub database: String,
    pub storage: String,
    pub whatsapp: String,
}

/// Service for utility operations like image uploads and system health.
pub struct UtilityService {
    pool: PgPool,
}

impl UtilityService {
    /// Creates a new `UtilityService`.
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    /// Uploads an image to the storage system.
    pub async fn upload_image(
        &self,
        file_name: String,
        data: Vec<u8>,
        mime_type: String,
    ) -> Result<ImageUploadResponse, AppError> {
        // Mock implementation - in real app would upload to S3/GCS
        let id = Uuid::new_v4();
        let url = format!(
            "https://storage.monitormbg.go.id/uploads/{}/{}.jpg",
            chrono::Utc::now().format("%Y/%m/%d"),
            id
        );

        Ok(ImageUploadResponse {
            success: true,
            url,
            file_name,
            size: data.len(),
            mime_type,
            width: 1920,  // Mock
            height: 1080, // Mock
            uploaded_at: chrono::Utc::now().to_rfc3339(),
        })
    }

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

    pub async fn health_check(&self) -> Result<HealthCheckResponse, AppError> {
        // Check DB connection
        let db_status = match sqlx::query("SELECT 1").execute(&self.pool).await {
            Ok(_) => "healthy",
            Err(_) => "unhealthy",
        };

        Ok(HealthCheckResponse {
            status: "healthy".to_string(),
            version: "1.0.0".to_string(),
            timestamp: chrono::Utc::now().to_rfc3339(),
            services: ServicesHealth {
                database: db_status.to_string(),
                storage: "healthy".to_string(),  // Mock
                whatsapp: "healthy".to_string(), // Mock
            },
        })
    }
}
