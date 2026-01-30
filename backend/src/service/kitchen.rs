//! Kitchen management service.

use std::sync::Arc;

use serde::Serialize;
use uuid::Uuid;

use crate::database::Database;
use crate::database::model::KitchenType;
use crate::database::table::Table;
use crate::error::AppError;

#[derive(Debug, Serialize)]
pub struct KitchenListResponse {
    pub data: Vec<KitchenDto>,
    pub pagination: Pagination,
}

#[derive(Debug, Serialize)]
pub struct KitchenDto {
    pub id: Uuid,
    pub name: String,
    pub location: String,
    pub r#type: Option<KitchenType>,
    #[serde(rename = "mealsServed")]
    pub meals_served: i32,
    pub certifications: Vec<String>,
    pub image: Option<String>,
    pub rating: f64,
    #[serde(rename = "totalReviews")]
    pub total_reviews: i32,
    #[serde(rename = "createdAt")]
    pub created_at: String,
    #[serde(rename = "updatedAt")]
    pub updated_at: String,
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
pub struct KitchenDetailDto {
    #[serde(flatten)]
    pub kitchen: KitchenDto,
    pub address: Option<String>,
    #[serde(rename = "contactPhone")]
    pub contact_phone: Option<String>,
    #[serde(rename = "contactEmail")]
    pub contact_email: Option<String>,
    #[serde(rename = "operatingHours")]
    pub operating_hours: Option<String>,
    pub capacity: Option<i32>,
    #[serde(rename = "performanceBadges")]
    pub performance_badges: Vec<PerformanceBadgeDto>,
    #[serde(rename = "complianceTrend")]
    pub compliance_trend: Vec<ComplianceTrendDto>,
}

#[derive(Debug, Serialize)]
pub struct PerformanceBadgeDto {
    pub r#type: String,
    pub title: String,
    pub description: String,
    #[serde(rename = "earnedDate")]
    pub earned_date: String,
    pub icon: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct ComplianceTrendDto {
    pub month: String,
    pub score: f64,
    pub incidents: i32,
}

#[derive(Debug, Serialize)]
pub struct KitchenStatsDto {
    #[serde(rename = "kitchenId")]
    pub kitchen_id: Uuid,
    #[serde(rename = "totalReviews")]
    pub total_reviews: i32,
    #[serde(rename = "verifiedReviews")]
    pub verified_reviews: i32,
    #[serde(rename = "averageRating")]
    pub average_rating: f64,
    #[serde(rename = "haccpScores")]
    pub haccp_scores: HaccpScoresDto,
    #[serde(rename = "reviewDistribution")]
    pub review_distribution: ReviewDistributionDto,
    #[serde(rename = "lastUpdated")]
    pub last_updated: String,
}

#[derive(Debug, Serialize, Default)]
pub struct HaccpScoresDto {
    pub taste: f64,
    pub hygiene: f64,
    pub freshness: f64,
    pub temperature: f64,
    pub packaging: f64,
    pub handling: f64,
}

#[derive(Debug, Serialize, Default)]
pub struct ReviewDistributionDto {
    #[serde(rename = "5")]
    pub five: i32,
    #[serde(rename = "4")]
    pub four: i32,
    #[serde(rename = "3")]
    pub three: i32,
    #[serde(rename = "2")]
    pub two: i32,
    #[serde(rename = "1")]
    pub one: i32,
}

/// Service for managing kitchen data and statistics.
pub struct KitchenService {
    db: Arc<Database>,
}

impl KitchenService {
    /// Creates a new `KitchenService`.
    pub fn new(db: Arc<Database>) -> Self {
        Self { db }
    }

    /// Lists kitchens with optional filtering and pagination.
    pub async fn list_kitchens(
        &self,
        limit: i64,
        offset: i64,
        q: Option<String>,
        loc: Option<String>,
        type_: Option<String>,
        min_rating: Option<f64>,
    ) -> Result<KitchenListResponse, AppError> {
        let (kitchens, total) = self
            .db
            .kitchen_table
            .list_kitchens_with_stats(
                q.as_deref(),
                loc.as_deref(),
                type_.as_deref(),
                min_rating,
                limit,
                offset,
            )
            .await?;

        let dtos = kitchens
            .into_iter()
            .map(|k| KitchenDto {
                id: k.id,
                name: k.name,
                location: format!(
                    "{}, {}",
                    k.city.unwrap_or_default(),
                    k.province.unwrap_or_default()
                ),
                r#type: k.r#type,
                meals_served: k.meals_served.unwrap_or(0),
                certifications: k
                    .certifications
                    .and_then(|c| serde_json::from_value(c).ok())
                    .unwrap_or_default(),
                image: k.image_url,
                rating: k
                    .average_rating
                    .and_then(|d| d.try_into().ok())
                    .unwrap_or(0.0),
                total_reviews: k.total_reviews as i32,
                created_at: k.created_at.unwrap_or_default().to_string(),
                updated_at: k.updated_at.unwrap_or_default().to_string(),
            })
            .collect();

        Ok(KitchenListResponse {
            data: dtos,
            pagination: Pagination {
                total,
                limit,
                offset,
                has_more: offset + limit < total,
            },
        })
    }

    pub async fn get_kitchen_detail(&self, id: Uuid) -> Result<KitchenDetailDto, AppError> {
        let kitchen = self
            .db
            .kitchen_table
            .select(&id)
            .await?
            .ok_or(AppError::NotFound("Kitchen not found".into()))?;

        // Get badges
        let badges = self
            .db
            .performance_badge_table
            .select_all()
            .await?
            .into_iter()
            .filter(|b| b.kitchen_id == id)
            .map(|b| PerformanceBadgeDto {
                r#type: b.r#type,
                title: b.title,
                description: b.description,
                earned_date: b.earned_date.to_string(),
                icon: None,
            })
            .collect();

        // Get compliance trend
        let trend = self
            .db
            .kitchen_table
            .get_compliance_trend(&id, 6)
            .await?
            .into_iter()
            .map(|t| ComplianceTrendDto {
                month: t.month,
                score: t
                    .average_score
                    .and_then(|d| d.try_into().ok())
                    .unwrap_or(0.0),
                incidents: t.incidents as i32,
            })
            .collect();

        // Get stats for rating
        let stats = self.db.kitchen_table.get_kitchen_stats(&id).await?;

        let rating = stats
            .as_ref()
            .and_then(|s| s.average_rating)
            .and_then(|d| d.try_into().ok())
            .unwrap_or(0.0);

        let total_reviews = stats.map(|s| s.total_reviews as i32).unwrap_or(0);

        Ok(KitchenDetailDto {
            kitchen: KitchenDto {
                id: kitchen.id,
                name: kitchen.name.clone(),
                location: format!(
                    "{}, {}",
                    kitchen.city.clone().unwrap_or_default(),
                    kitchen.province.clone().unwrap_or_default()
                ),
                r#type: kitchen.r#type,
                meals_served: kitchen.meals_served.unwrap_or(0),
                certifications: kitchen
                    .certifications
                    .clone()
                    .and_then(|c| serde_json::from_value(c).ok())
                    .unwrap_or_default(),
                image: kitchen.image_url.clone(),
                rating,
                total_reviews,
                created_at: kitchen.created_at.unwrap_or_default().to_string(),
                updated_at: kitchen.updated_at.unwrap_or_default().to_string(),
            },
            address: kitchen.address,
            contact_phone: None,   // Not in DB model
            contact_email: None,   // Not in DB model
            operating_hours: None, // Not in DB model
            capacity: None,        // Not in DB model
            performance_badges: badges,
            compliance_trend: trend,
        })
    }

    pub async fn get_kitchen_stats(&self, id: Uuid) -> Result<KitchenStatsDto, AppError> {
        // Verify kitchen exists
        let _ = self
            .db
            .kitchen_table
            .select(&id)
            .await?
            .ok_or(AppError::NotFound("Kitchen not found".into()))?;

        // Get stats from reviews
        let stats = self.db.kitchen_table.get_kitchen_stats(&id).await?;

        // Get review distribution
        let distribution = self.db.kitchen_table.get_review_distribution(&id).await?;

        let mut review_dist = ReviewDistributionDto::default();
        for d in distribution {
            match d.rating_bucket {
                5 => review_dist.five = d.count as i32,
                4 => review_dist.four = d.count as i32,
                3 => review_dist.three = d.count as i32,
                2 => review_dist.two = d.count as i32,
                1 => review_dist.one = d.count as i32,
                _ => {}
            }
        }

        let haccp_scores = if let Some(ref s) = stats {
            HaccpScoresDto {
                taste: s.taste_avg.and_then(|d| d.try_into().ok()).unwrap_or(0.0),
                hygiene: s.hygiene_avg.and_then(|d| d.try_into().ok()).unwrap_or(0.0),
                freshness: s
                    .freshness_avg
                    .and_then(|d| d.try_into().ok())
                    .unwrap_or(0.0),
                temperature: s
                    .temperature_avg
                    .and_then(|d| d.try_into().ok())
                    .unwrap_or(0.0),
                packaging: s
                    .packaging_avg
                    .and_then(|d| d.try_into().ok())
                    .unwrap_or(0.0),
                handling: s
                    .handling_avg
                    .and_then(|d| d.try_into().ok())
                    .unwrap_or(0.0),
            }
        } else {
            HaccpScoresDto::default()
        };

        Ok(KitchenStatsDto {
            kitchen_id: id,
            total_reviews: stats.as_ref().map(|s| s.total_reviews as i32).unwrap_or(0),
            verified_reviews: stats
                .as_ref()
                .map(|s| s.verified_reviews as i32)
                .unwrap_or(0),
            average_rating: stats
                .and_then(|s| s.average_rating)
                .and_then(|d| d.try_into().ok())
                .unwrap_or(0.0),
            haccp_scores,
            review_distribution: review_dist,
            last_updated: chrono::Utc::now().to_rfc3339(),
        })
    }

    pub async fn get_multiple_kitchens(&self, ids: Vec<Uuid>) -> Result<Vec<KitchenDto>, AppError> {
        let mut kitchens = Vec::new();
        for id in ids {
            if let Ok(Some(k)) = self.db.kitchen_table.select(&id).await {
                // Get stats for this kitchen
                let stats = self
                    .db
                    .kitchen_table
                    .get_kitchen_stats(&id)
                    .await
                    .ok()
                    .flatten();

                kitchens.push(KitchenDto {
                    id: k.id,
                    name: k.name,
                    location: format!(
                        "{}, {}",
                        k.city.unwrap_or_default(),
                        k.province.unwrap_or_default()
                    ),
                    r#type: k.r#type,
                    meals_served: k.meals_served.unwrap_or(0),
                    certifications: k
                        .certifications
                        .and_then(|c| serde_json::from_value(c).ok())
                        .unwrap_or_default(),
                    image: k.image_url,
                    rating: stats
                        .as_ref()
                        .and_then(|s| s.average_rating)
                        .and_then(|d| d.try_into().ok())
                        .unwrap_or(0.0),
                    total_reviews: stats.map(|s| s.total_reviews as i32).unwrap_or(0),
                    created_at: k.created_at.unwrap_or_default().to_string(),
                    updated_at: k.updated_at.unwrap_or_default().to_string(),
                });
            }
        }
        Ok(kitchens)
    }
}
