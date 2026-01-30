//! Review and rating service.

use std::sync::Arc;

use rust_decimal::Decimal;
use rust_decimal::prelude::FromPrimitive;
use serde::Deserialize;
use serde::Serialize;
use serde_json::json;
use uuid::Uuid;

use crate::database::Database;
use crate::database::model::Review;
use crate::database::model::UserRole;
use crate::database::table::Table;
use crate::error::AppError;

#[derive(Debug, Serialize, Deserialize)]
pub struct HaccpRatingDto {
    pub taste: f64,
    pub hygiene: f64,
    pub freshness: f64,
    pub temperature: f64,
    pub packaging: f64,
    pub handling: f64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateReviewRequest {
    #[serde(rename = "kitchenId")]
    pub kitchen_id: Uuid,
    #[serde(rename = "reviewerName")]
    pub reviewer_name: String,
    #[serde(rename = "reviewerType")]
    pub reviewer_type: UserRole,
    pub ratings: HaccpRatingDto,
    pub comment: String,
    pub photos: Option<Vec<String>>,
    #[serde(rename = "deliveryDate")]
    pub delivery_date: Option<String>,
    #[serde(rename = "mealType")]
    pub meal_type: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateReviewRequest {
    pub ratings: Option<HaccpRatingDto>,
    pub comment: Option<String>,
    pub photos: Option<Vec<String>>,
}

#[derive(Debug, Serialize)]
pub struct ReviewDto {
    pub id: Uuid,
    #[serde(rename = "kitchenId")]
    pub kitchen_id: Uuid,
    #[serde(rename = "reviewerName")]
    pub reviewer_name: String,
    #[serde(rename = "reviewerType")]
    pub reviewer_type: UserRole,
    pub ratings: HaccpRatingDto,
    #[serde(rename = "averageRating")]
    pub average_rating: f64,
    pub comment: String,
    pub photos: Vec<String>,
    #[serde(rename = "deliveryDate")]
    pub delivery_date: Option<String>,
    #[serde(rename = "mealType")]
    pub meal_type: Option<String>,
    pub verified: bool,
    #[serde(rename = "verificationStatus")]
    pub verification_status: String,
    #[serde(rename = "reportSource")]
    pub report_source: String,
    #[serde(rename = "confidenceLevel")]
    pub confidence_level: String,
    #[serde(rename = "createdAt")]
    pub created_at: String,
    #[serde(rename = "updatedAt")]
    pub updated_at: String,
}

#[derive(Debug, Serialize)]
pub struct ReviewListResponse {
    pub data: Vec<ReviewDto>,
    pub pagination: Pagination,
}

#[derive(Debug, Serialize)]
pub struct Pagination {
    pub total: i64,
    pub limit: i64,
    pub offset: i64,
    #[serde(rename = "hasMore")]
    pub has_more: bool,
}

#[derive(Debug, Serialize)]
pub struct BatchReviewResult {
    #[serde(rename = "kitchenId")]
    pub kitchen_id: Uuid,
    pub status: String,
    #[serde(rename = "reviewId")]
    pub review_id: Option<Uuid>,
    pub error: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct BatchReviewResponse {
    pub success: bool,
    pub created: i32,
    pub failed: i32,
    pub results: Vec<BatchReviewResult>,
}

/// Service for managing kitchen reviews and ratings.
pub struct ReviewService {
    db: Arc<Database>,
}

impl ReviewService {
    /// Creates a new `ReviewService`.
    pub fn new(db: Arc<Database>) -> Self {
        Self { db }
    }

    /// Submits a new review for a kitchen.
    pub async fn submit_review(
        &self,
        reviewer_id: Uuid,
        req: CreateReviewRequest,
    ) -> Result<ReviewDto, AppError> {
        let review = Review {
            id: Uuid::new_v4(),
            kitchen_id: req.kitchen_id,
            reviewer_id,
            reviewer_name: req.reviewer_name.clone(),
            reviewer_type: req.reviewer_type,
            taste_rating: Decimal::from_f64(req.ratings.taste).unwrap_or_default(),
            hygiene_rating: Decimal::from_f64(req.ratings.hygiene).unwrap_or_default(),
            freshness_rating: Decimal::from_f64(req.ratings.freshness).unwrap_or_default(),
            temperature_rating: Decimal::from_f64(req.ratings.temperature).unwrap_or_default(),
            packaging_rating: Decimal::from_f64(req.ratings.packaging).unwrap_or_default(),
            handling_rating: Decimal::from_f64(req.ratings.handling).unwrap_or_default(),
            comment: req.comment.clone(),
            photos: req.photos.clone().map(|p| json!(p)),
            verification_status: Some("unverified".to_string()),
            report_source: "public".to_string(),
            confidence_level: "medium".to_string(),
            verified: Some(false),
            is_draft: Some(false),
            created_at: Some(chrono::Utc::now().naive_utc()),
            updated_at: Some(chrono::Utc::now().naive_utc()),
            ..Default::default()
        };

        let id = self.db.review_table.insert(&review).await?;

        // Fetch back to return DTO
        let saved =
            self.db
                .review_table
                .select(&id)
                .await?
                .ok_or(AppError::InternalServerError(
                    "Failed to retrieve saved review".into(),
                ))?;

        self.map_to_dto(saved)
    }

    pub async fn get_kitchen_reviews(
        &self,
        kitchen_id: Uuid,
        limit: i64,
        offset: i64,
    ) -> Result<ReviewListResponse, AppError> {
        // Get all reviews and filter by kitchen
        let all_reviews = self.db.review_table.select_all().await?;

        let filtered: Vec<Review> = all_reviews
            .into_iter()
            .filter(|r| r.kitchen_id == kitchen_id)
            .collect();

        let total = filtered.len() as i64;
        let paginated: Vec<Review> = filtered
            .into_iter()
            .skip(offset as usize)
            .take(limit as usize)
            .collect();

        let dtos = paginated
            .into_iter()
            .map(|r| self.map_to_dto_sync(r))
            .collect();

        Ok(ReviewListResponse {
            data: dtos,
            pagination: Pagination {
                total,
                limit,
                offset,
                has_more: offset + limit < total,
            },
        })
    }

    pub async fn get_public_reviews(
        &self,
        limit: i64,
        offset: i64,
    ) -> Result<ReviewListResponse, AppError> {
        // Get verified reviews only
        let all_reviews = self.db.review_table.select_all().await?;

        let filtered: Vec<Review> = all_reviews
            .into_iter()
            .filter(|r| r.verified.unwrap_or(false))
            .collect();

        let total = filtered.len() as i64;
        let paginated: Vec<Review> = filtered
            .into_iter()
            .skip(offset as usize)
            .take(limit as usize)
            .collect();

        let dtos = paginated
            .into_iter()
            .map(|r| self.map_to_dto_sync(r))
            .collect();

        Ok(ReviewListResponse {
            data: dtos,
            pagination: Pagination {
                total,
                limit,
                offset,
                has_more: offset + limit < total,
            },
        })
    }

    pub async fn submit_batch_reviews(
        &self,
        reviewer_id: Uuid,
        reviews: Vec<CreateReviewRequest>,
    ) -> Result<BatchReviewResponse, AppError> {
        let mut results = Vec::new();
        let mut created = 0;
        let mut failed = 0;

        for req in reviews {
            let kitchen_id = req.kitchen_id;
            match self.submit_review(reviewer_id, req).await {
                Ok(review) => {
                    created += 1;
                    results.push(BatchReviewResult {
                        kitchen_id,
                        status: "created".to_string(),
                        review_id: Some(review.id),
                        error: None,
                    });
                }
                Err(e) => {
                    failed += 1;
                    results.push(BatchReviewResult {
                        kitchen_id,
                        status: "failed".to_string(),
                        review_id: None,
                        error: Some(e.to_string()),
                    });
                }
            }
        }

        Ok(BatchReviewResponse {
            success: true,
            created,
            failed,
            results,
        })
    }

    pub async fn update_review(
        &self,
        review_id: Uuid,
        user_id: Uuid,
        req: UpdateReviewRequest,
    ) -> Result<ReviewDto, AppError> {
        // First, check if review exists and belongs to the user
        let review = self
            .db
            .review_table
            .select(&review_id)
            .await?
            .ok_or(AppError::NotFound("Review not found".into()))?;

        // Check ownership
        if review.reviewer_id != user_id {
            return Err(AppError::Unauthorized(
                "You can only update your own reviews".into(),
            ));
        }

        // Check if review is already verified
        if review.verified.unwrap_or(false) {
            return Err(AppError::BadRequest(
                "Cannot update a verified review".into(),
            ));
        }

        // Build updated review
        let mut updated_review = review.clone();

        if let Some(ratings) = req.ratings {
            updated_review.taste_rating = Decimal::from_f64(ratings.taste).unwrap_or_default();
            updated_review.hygiene_rating = Decimal::from_f64(ratings.hygiene).unwrap_or_default();
            updated_review.freshness_rating =
                Decimal::from_f64(ratings.freshness).unwrap_or_default();
            updated_review.temperature_rating =
                Decimal::from_f64(ratings.temperature).unwrap_or_default();
            updated_review.packaging_rating =
                Decimal::from_f64(ratings.packaging).unwrap_or_default();
            updated_review.handling_rating =
                Decimal::from_f64(ratings.handling).unwrap_or_default();
        }

        if let Some(comment) = req.comment {
            updated_review.comment = comment;
        }

        if let Some(photos) = req.photos {
            updated_review.photos = Some(serde_json::json!(photos));
        }

        updated_review.updated_at = Some(chrono::Utc::now().naive_utc());

        // Update in database
        self.db.review_table.update(&updated_review).await?;

        // Fetch back to return DTO
        let saved =
            self.db
                .review_table
                .select(&review_id)
                .await?
                .ok_or(AppError::InternalServerError(
                    "Failed to retrieve updated review".into(),
                ))?;

        self.map_to_dto(saved)
    }

    pub async fn delete_review(&self, review_id: Uuid, user_id: Uuid) -> Result<(), AppError> {
        // First, check if review exists and belongs to the user
        let review = self
            .db
            .review_table
            .select(&review_id)
            .await?
            .ok_or(AppError::NotFound("Review not found".into()))?;

        // Check ownership
        if review.reviewer_id != user_id {
            return Err(AppError::Unauthorized(
                "You can only delete your own reviews".into(),
            ));
        }

        // Check if review is already verified
        if review.verified.unwrap_or(false) {
            return Err(AppError::Forbidden(
                "Cannot delete a verified review".into(),
            ));
        }

        // Delete from database
        self.db.review_table.delete(&review_id).await?;

        Ok(())
    }

    fn map_to_dto(&self, r: Review) -> Result<ReviewDto, AppError> {
        Ok(self.map_to_dto_sync(r))
    }

    fn map_to_dto_sync(&self, r: Review) -> ReviewDto {
        use rust_decimal::prelude::ToPrimitive;

        let photos: Vec<String> = if let Some(p) = r.photos {
            serde_json::from_value(p).unwrap_or_default()
        } else {
            vec![]
        };

        let avg = (r.taste_rating
            + r.hygiene_rating
            + r.freshness_rating
            + r.temperature_rating
            + r.packaging_rating
            + r.handling_rating)
            / Decimal::from(6);

        ReviewDto {
            id: r.id,
            kitchen_id: r.kitchen_id,
            reviewer_name: r.reviewer_name,
            reviewer_type: r.reviewer_type,
            ratings: HaccpRatingDto {
                taste: r.taste_rating.to_f64().unwrap_or_default(),
                hygiene: r.hygiene_rating.to_f64().unwrap_or_default(),
                freshness: r.freshness_rating.to_f64().unwrap_or_default(),
                temperature: r.temperature_rating.to_f64().unwrap_or_default(),
                packaging: r.packaging_rating.to_f64().unwrap_or_default(),
                handling: r.handling_rating.to_f64().unwrap_or_default(),
            },
            average_rating: avg.to_f64().unwrap_or_default(),
            comment: r.comment,
            photos,
            delivery_date: None, // Not in DB model yet
            meal_type: None,     // Not in DB model yet
            verified: r.verified.unwrap_or(false),
            verification_status: r.verification_status.unwrap_or("unverified".to_string()),
            report_source: r.report_source,
            confidence_level: r.confidence_level,
            created_at: r.created_at.unwrap_or_default().to_string(),
            updated_at: r.updated_at.unwrap_or_default().to_string(),
        }
    }
}
