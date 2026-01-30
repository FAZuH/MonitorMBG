//! Kitchen management service.

use serde::Serialize;
use sqlx::PgPool;
use uuid::Uuid;

use crate::database::model::KitchenType;
use crate::database::model::PerformanceBadge;
use crate::database::table::KitchenTable;
use crate::database::table::PerformanceBadgeTable;
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
    pub performance_badges: Vec<PerformanceBadge>,
    #[serde(rename = "complianceTrend")]
    pub compliance_trend: Vec<ComplianceTrendDto>,
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
    pool: PgPool,
    kitchen_table: KitchenTable,
    badge_table: PerformanceBadgeTable,
}

impl KitchenService {
    /// Creates a new `KitchenService`.
    pub fn new(pool: PgPool) -> Self {
        Self {
            pool: pool.clone(),
            kitchen_table: KitchenTable::new(pool.clone()),
            badge_table: PerformanceBadgeTable::new(pool),
        }
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
        // TODO: Implement actual filtering and pagination in DB
        // For now, fetch all and filter in memory (not efficient but works for prototype)
        // In production, we should add methods to KitchenTable for filtered queries

        let kitchens = self.kitchen_table.select_all().await?;

        // Mock data for computed fields
        let mut dtos = Vec::new();
        for k in kitchens {
            // Filter logic
            if let Some(ref query) = q
                && !k.name.to_lowercase().contains(&query.to_lowercase()) {
                    continue;
                }

            // Map to DTO
            dtos.push(KitchenDto {
                id: k.id,
                name: k.name.clone(),
                location: format!(
                    "{}, {}",
                    k.city.clone().unwrap_or_default(),
                    k.province.clone().unwrap_or_default()
                ),
                r#type: k.r#type,
                meals_served: k.meals_served.unwrap_or(0),
                certifications: vec!["Halal".to_string(), "HACCP".to_string()], // Mock
                image: k.image_url,
                rating: 4.5,        // Mock
                total_reviews: 100, // Mock
                created_at: k.created_at.unwrap_or_default().to_string(),
                updated_at: k.updated_at.unwrap_or_default().to_string(),
            });
        }

        let total = dtos.len() as i64;
        let paginated_dtos = dtos
            .into_iter()
            .skip(offset as usize)
            .take(limit as usize)
            .collect();

        Ok(KitchenListResponse {
            data: paginated_dtos,
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
            .kitchen_table
            .select(&id)
            .await?
            .ok_or(AppError::NotFound("Kitchen not found".into()))?;

        // Mock badges
        let badges = vec![];

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
                certifications: vec!["Halal".to_string()],
                image: kitchen.image_url,
                rating: 4.8,
                total_reviews: 150,
                created_at: kitchen.created_at.unwrap_or_default().to_string(),
                updated_at: kitchen.updated_at.unwrap_or_default().to_string(),
            },
            address: kitchen.address,
            contact_phone: Some("08123456789".to_string()), // Mock
            contact_email: Some("kitchen@example.com".to_string()), // Mock
            operating_hours: Some("08:00 - 17:00".to_string()), // Mock
            capacity: Some(5000),                           // Mock
            performance_badges: badges,
            compliance_trend: vec![], // Mock
        })
    }

    pub async fn get_kitchen_stats(&self, id: Uuid) -> Result<KitchenStatsDto, AppError> {
        // Verify kitchen exists
        let _ = self
            .kitchen_table
            .select(&id)
            .await?
            .ok_or(AppError::NotFound("Kitchen not found".into()))?;

        Ok(KitchenStatsDto {
            kitchen_id: id,
            total_reviews: 156,
            verified_reviews: 142,
            average_rating: 4.7,
            haccp_scores: HaccpScoresDto {
                taste: 4.5,
                hygiene: 4.8,
                freshness: 4.7,
                temperature: 4.9,
                packaging: 4.6,
                handling: 4.8,
            },
            review_distribution: ReviewDistributionDto {
                five: 98,
                four: 42,
                three: 12,
                two: 3,
                one: 1,
            },
            last_updated: chrono::Utc::now().to_rfc3339(),
        })
    }

    pub async fn get_multiple_kitchens(&self, ids: Vec<Uuid>) -> Result<Vec<KitchenDto>, AppError> {
        let mut kitchens = Vec::new();
        for id in ids {
            if let Ok(Some(k)) = self.kitchen_table.select(&id).await {
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
                    certifications: vec![],
                    image: k.image_url,
                    rating: 0.0,
                    total_reviews: 0,
                    created_at: k.created_at.unwrap_or_default().to_string(),
                    updated_at: k.updated_at.unwrap_or_default().to_string(),
                });
            }
        }
        Ok(kitchens)
    }
}
