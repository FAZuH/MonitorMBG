//! Statistics and analytics service.

use serde::Serialize;
use sqlx::PgPool;
use uuid::Uuid;

use crate::error::AppError;

#[derive(Debug, Serialize)]
pub struct NationalStatsDto {
    pub period: PeriodDto,
    #[serde(rename = "totalKitchens")]
    pub total_kitchens: i32,
    #[serde(rename = "activeKitchens")]
    pub active_kitchens: i32,
    #[serde(rename = "certifiedKitchens")]
    pub certified_kitchens: i32,
    #[serde(rename = "totalReviews")]
    pub total_reviews: i32,
    #[serde(rename = "verifiedReviews")]
    pub verified_reviews: i32,
    #[serde(rename = "averageRating")]
    pub average_rating: f64,
    #[serde(rename = "averageComplianceScore")]
    pub average_compliance_score: f64,
    #[serde(rename = "totalIncidents")]
    pub total_incidents: i32,
    #[serde(rename = "activeIncidents")]
    pub active_incidents: i32,
    #[serde(rename = "resolvedIncidents")]
    pub resolved_incidents: i32,
    #[serde(rename = "criticalIncidents")]
    pub critical_incidents: i32,
    #[serde(rename = "totalVictims")]
    pub total_victims: i32,
    #[serde(rename = "totalDeaths")]
    pub total_deaths: i32,
    #[serde(rename = "provinceStats")]
    pub province_stats: Vec<ProvinceStatsDto>,
    #[serde(rename = "lastUpdated")]
    pub last_updated: String,
}

#[derive(Debug, Serialize)]
pub struct PeriodDto {
    pub year: i32,
    pub month: Option<i32>,
}

#[derive(Debug, Serialize)]
pub struct ProvinceStatsDto {
    pub province: String,
    #[serde(rename = "totalKitchens")]
    pub total_kitchens: i32,
    #[serde(rename = "avgRating")]
    pub avg_rating: f64,
    pub incidents: i32,
}

#[derive(Debug, Serialize)]
pub struct RegionalStatsDto {
    pub region: RegionDto,
    pub period: PeriodDto,
    #[serde(rename = "totalKitchens")]
    pub total_kitchens: i32,
    #[serde(rename = "activeKitchens")]
    pub active_kitchens: i32,
    #[serde(rename = "certifiedKitchens")]
    pub certified_kitchens: i32,
    #[serde(rename = "averageRating")]
    pub average_rating: f64,
    #[serde(rename = "averageComplianceScore")]
    pub average_compliance_score: f64,
    #[serde(rename = "totalReviews")]
    pub total_reviews: i32,
    #[serde(rename = "totalIncidents")]
    pub total_incidents: i32,
    #[serde(rename = "resolvedIncidents")]
    pub resolved_incidents: i32,
    #[serde(rename = "activeIncidents")]
    pub active_incidents: i32,
    #[serde(rename = "topPerformingKitchens")]
    pub top_performing_kitchens: Vec<TopKitchenDto>,
    #[serde(rename = "lastUpdated")]
    pub last_updated: String,
}

#[derive(Debug, Serialize)]
pub struct RegionDto {
    pub province: String,
    pub kabupaten: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct TopKitchenDto {
    pub id: Uuid,
    pub name: String,
    pub rating: f64,
    #[serde(rename = "complianceScore")]
    pub compliance_score: f64,
}

#[derive(Debug, Serialize)]
pub struct ComplianceTrendsDto {
    pub region: RegionDto,
    pub period: DateRangeDto,
    pub data: Vec<ComplianceTrendDataDto>,
    pub summary: ComplianceSummaryDto,
    #[serde(rename = "lastUpdated")]
    pub last_updated: String,
}

#[derive(Debug, Serialize)]
pub struct DateRangeDto {
    pub from: String,
    pub to: String,
}

#[derive(Debug, Serialize)]
pub struct ComplianceTrendDataDto {
    pub month: String,
    #[serde(rename = "averageScore")]
    pub average_score: f64,
    pub incidents: i32,
    pub reviews: i32,
    #[serde(rename = "averageRating")]
    pub average_rating: f64,
}

#[derive(Debug, Serialize)]
pub struct ComplianceSummaryDto {
    pub trend: String,
    #[serde(rename = "changePercent")]
    pub change_percent: f64,
    #[serde(rename = "totalIncidents")]
    pub total_incidents: i32,
    #[serde(rename = "totalReviews")]
    pub total_reviews: i32,
}

#[derive(Debug, Serialize)]
pub struct IncidentTrendsDto {
    pub period: DateRangeDto,
    #[serde(rename = "groupBy")]
    pub group_by: String,
    pub data: Vec<IncidentTrendDataDto>,
    pub summary: IncidentSummaryDto,
    #[serde(rename = "lastUpdated")]
    pub last_updated: String,
}

#[derive(Debug, Serialize)]
pub struct IncidentTrendDataDto {
    pub month: String,
    #[serde(rename = "totalIncidents")]
    pub total_incidents: i32,
    #[serde(rename = "totalVictims")]
    pub total_victims: i32,
    pub deaths: i32,
    #[serde(rename = "topCause")]
    pub top_cause: String,
}

#[derive(Debug, Serialize)]
pub struct IncidentSummaryDto {
    #[serde(rename = "totalIncidents")]
    pub total_incidents: i32,
    #[serde(rename = "totalVictims")]
    pub total_victims: i32,
    #[serde(rename = "totalDeaths")]
    pub total_deaths: i32,
    #[serde(rename = "mostCommonCause")]
    pub most_common_cause: String,
    pub trend: String,
}

/// Service for generating national and regional statistics.
pub struct StatsService {
    pool: PgPool,
}

impl StatsService {
    /// Creates a new `StatsService`.
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    /// Retrieves national-level statistics.
    pub async fn get_national_stats(
        &self,
        year: Option<i32>,
        month: Option<i32>,
    ) -> Result<NationalStatsDto, AppError> {
        // Mock implementation
        Ok(NationalStatsDto {
            period: PeriodDto {
                year: year.unwrap_or(2025),
                month,
            },
            total_kitchens: 1248,
            active_kitchens: 1205,
            certified_kitchens: 892,
            total_reviews: 15678,
            verified_reviews: 14234,
            average_rating: 4.6,
            average_compliance_score: 91.3,
            total_incidents: 23,
            active_incidents: 5,
            resolved_incidents: 18,
            critical_incidents: 2,
            total_victims: 456,
            total_deaths: 0,
            province_stats: vec![ProvinceStatsDto {
                province: "DKI Jakarta".to_string(),
                total_kitchens: 145,
                avg_rating: 4.7,
                incidents: 3,
            }],
            last_updated: chrono::Utc::now().to_rfc3339(),
        })
    }

    pub async fn get_regional_stats(
        &self,
        province: Option<String>,
        kabupaten: Option<String>,
        year: Option<i32>,
        month: Option<i32>,
    ) -> Result<RegionalStatsDto, AppError> {
        // Mock implementation
        Ok(RegionalStatsDto {
            region: RegionDto {
                province: province.unwrap_or("DKI Jakarta".to_string()),
                kabupaten,
            },
            period: PeriodDto {
                year: year.unwrap_or(2025),
                month,
            },
            total_kitchens: 45,
            active_kitchens: 43,
            certified_kitchens: 38,
            average_rating: 4.7,
            average_compliance_score: 92.5,
            total_reviews: 2341,
            total_incidents: 1,
            resolved_incidents: 1,
            active_incidents: 0,
            top_performing_kitchens: vec![TopKitchenDto {
                id: Uuid::new_v4(),
                name: "Dapur Sehat Jakarta Pusat".to_string(),
                rating: 4.9,
                compliance_score: 98.5,
            }],
            last_updated: chrono::Utc::now().to_rfc3339(),
        })
    }

    pub async fn get_compliance_trends(
        &self,
        province: Option<String>,
        kabupaten: Option<String>,
        kitchen_id: Option<Uuid>,
        months: Option<i32>,
    ) -> Result<ComplianceTrendsDto, AppError> {
        // Mock implementation
        Ok(ComplianceTrendsDto {
            region: RegionDto {
                province: province.unwrap_or("DKI Jakarta".to_string()),
                kabupaten,
            },
            period: DateRangeDto {
                from: "2024-02-01T00:00:00Z".to_string(),
                to: "2025-01-31T23:59:59Z".to_string(),
            },
            data: vec![ComplianceTrendDataDto {
                month: "2024-02".to_string(),
                average_score: 88.5,
                incidents: 5,
                reviews: 1234,
                average_rating: 4.5,
            }],
            summary: ComplianceSummaryDto {
                trend: "improving".to_string(),
                change_percent: 4.5,
                total_incidents: 23,
                total_reviews: 15678,
            },
            last_updated: chrono::Utc::now().to_rfc3339(),
        })
    }

    pub async fn get_incident_trends(
        &self,
        province: Option<String>,
        months: Option<i32>,
        group_by: Option<String>,
    ) -> Result<IncidentTrendsDto, AppError> {
        // Mock implementation
        Ok(IncidentTrendsDto {
            period: DateRangeDto {
                from: "2024-02-01T00:00:00Z".to_string(),
                to: "2025-01-31T23:59:59Z".to_string(),
            },
            group_by: group_by.unwrap_or("month".to_string()),
            data: vec![IncidentTrendDataDto {
                month: "2024-02".to_string(),
                total_incidents: 5,
                total_victims: 125,
                deaths: 0,
                top_cause: "Bacterial contamination".to_string(),
            }],
            summary: IncidentSummaryDto {
                total_incidents: 23,
                total_victims: 456,
                total_deaths: 0,
                most_common_cause: "Bacterial contamination".to_string(),
                trend: "decreasing".to_string(),
            },
            last_updated: chrono::Utc::now().to_rfc3339(),
        })
    }
}
