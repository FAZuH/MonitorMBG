//! Statistics and analytics service.

use std::sync::Arc;

use serde::Serialize;
use uuid::Uuid;

use crate::database::Database;
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
    db: Arc<Database>,
}

impl StatsService {
    /// Creates a new `StatsService`.
    pub fn new(db: Arc<Database>) -> Self {
        Self { db }
    }

    /// Retrieves national-level statistics.
    pub async fn get_national_stats(
        &self,
        year: Option<i32>,
        month: Option<i32>,
    ) -> Result<NationalStatsDto, AppError> {
        let stats = self.db.stats_queries.get_national_stats().await?;
        let province_stats = self.db.stats_queries.get_province_stats().await?;

        Ok(NationalStatsDto {
            period: PeriodDto {
                year: year.unwrap_or(2025),
                month,
            },
            total_kitchens: stats.total_kitchens as i32,
            active_kitchens: stats.active_kitchens as i32,
            certified_kitchens: stats.certified_kitchens as i32,
            total_reviews: stats.total_reviews as i32,
            verified_reviews: stats.verified_reviews as i32,
            average_rating: stats
                .average_rating
                .and_then(|d| d.try_into().ok())
                .unwrap_or(0.0),
            average_compliance_score: stats
                .average_rating
                .and_then(|d| d.try_into().ok())
                .unwrap_or(0.0),
            total_incidents: stats.total_incidents as i32,
            active_incidents: stats.active_incidents as i32,
            resolved_incidents: stats.resolved_incidents as i32,
            critical_incidents: stats.critical_incidents as i32,
            total_victims: stats.total_victims as i32,
            total_deaths: stats.total_deaths as i32,
            province_stats: province_stats
                .into_iter()
                .map(|p| ProvinceStatsDto {
                    province: p.province,
                    total_kitchens: p.total_kitchens as i32,
                    avg_rating: p.avg_rating.and_then(|d| d.try_into().ok()).unwrap_or(0.0),
                    incidents: p.incidents as i32,
                })
                .collect(),
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
        let stats = self
            .db
            .stats_queries
            .get_regional_stats(province.as_deref(), kabupaten.as_deref())
            .await?;

        let top_kitchens = self
            .db
            .stats_queries
            .get_top_kitchens(province.as_deref(), kabupaten.as_deref(), 5)
            .await?;

        Ok(RegionalStatsDto {
            region: RegionDto {
                province: province.unwrap_or("National".to_string()),
                kabupaten,
            },
            period: PeriodDto {
                year: year.unwrap_or(2025),
                month,
            },
            total_kitchens: stats.total_kitchens as i32,
            active_kitchens: stats.active_kitchens as i32,
            certified_kitchens: stats.certified_kitchens as i32,
            average_rating: stats
                .average_rating
                .and_then(|d| d.try_into().ok())
                .unwrap_or(0.0),
            average_compliance_score: stats
                .average_rating
                .and_then(|d| d.try_into().ok())
                .unwrap_or(0.0),
            total_reviews: stats.total_reviews as i32,
            total_incidents: stats.total_incidents as i32,
            resolved_incidents: stats.resolved_incidents as i32,
            active_incidents: stats.active_incidents as i32,
            top_performing_kitchens: top_kitchens
                .into_iter()
                .map(|k| TopKitchenDto {
                    id: k.id,
                    name: k.name,
                    rating: k.rating.and_then(|d| d.try_into().ok()).unwrap_or(0.0),
                    compliance_score: k
                        .compliance_score
                        .and_then(|d| d.try_into().ok())
                        .unwrap_or(0.0),
                })
                .collect(),
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
        let months = months.unwrap_or(12);
        let trends = self
            .db
            .stats_queries
            .get_compliance_trends(province.as_deref(), kabupaten.as_deref(), kitchen_id, months)
            .await?;

        let total_reviews: i32 = trends.iter().map(|t| t.reviews as i32).sum();
        let total_incidents: i32 = trends.iter().map(|t| t.incidents as i32).sum();

        // Calculate trend direction
        let trend = if trends.len() >= 2 {
            let first = trends
                .first()
                .and_then(|t| t.average_score)
                .and_then(|d| d.try_into().ok())
                .unwrap_or(0.0);
            let last = trends
                .last()
                .and_then(|t| t.average_score)
                .and_then(|d| d.try_into().ok())
                .unwrap_or(0.0);
            if first > last {
                "improving"
            } else if first < last {
                "declining"
            } else {
                "stable"
            }
        } else {
            "stable"
        };

        // Calculate change percentage
        let change_percent = if trends.len() >= 2 {
            let first = trends
                .first()
                .and_then(|t| t.average_score)
                .and_then(|d| d.try_into().ok())
                .unwrap_or(0.0);
            let last = trends
                .last()
                .and_then(|t| t.average_score)
                .and_then(|d| d.try_into().ok())
                .unwrap_or(0.0);
            if first != 0.0 {
                ((last - first) / first * 100.0_f64).abs()
            } else {
                0.0
            }
        } else {
            0.0
        };

        Ok(ComplianceTrendsDto {
            region: RegionDto {
                province: province.unwrap_or("National".to_string()),
                kabupaten,
            },
            period: DateRangeDto {
                from: chrono::Utc::now()
                    .checked_sub_months(chrono::Months::new(months as u32))
                    .unwrap_or_else(|| chrono::Utc::now())
                    .to_rfc3339(),
                to: chrono::Utc::now().to_rfc3339(),
            },
            data: trends
                .into_iter()
                .map(|t| ComplianceTrendDataDto {
                    month: t.month,
                    average_score: t
                        .average_score
                        .and_then(|d| d.try_into().ok())
                        .unwrap_or(0.0),
                    incidents: t.incidents as i32,
                    reviews: t.reviews as i32,
                    average_rating: t
                        .average_rating
                        .and_then(|d| d.try_into().ok())
                        .unwrap_or(0.0),
                })
                .collect(),
            summary: ComplianceSummaryDto {
                trend: trend.to_string(),
                change_percent,
                total_incidents,
                total_reviews,
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
        let months = months.unwrap_or(12);
        let trends = self
            .db
            .stats_queries
            .get_incident_trends(province.as_deref(), months)
            .await?;

        let total_incidents: i32 = trends.iter().map(|t| t.total_incidents as i32).sum();
        let total_victims: i32 = trends.iter().map(|t| t.total_victims as i32).sum();
        let total_deaths: i32 = trends.iter().map(|t| t.deaths as i32).sum();

        // Determine most common cause
        let mut cause_counts: std::collections::HashMap<String, i32> = std::collections::HashMap::new();
        for t in &trends {
            if let Some(ref cause) = t.top_cause {
                *cause_counts.entry(cause.clone()).or_insert(0) += 1;
            }
        }
        let most_common_cause = cause_counts
            .iter()
            .max_by_key(|&(_, count)| count)
            .map(|(cause, _)| cause.clone())
            .unwrap_or_else(|| "Unknown".to_string());

        // Calculate trend direction
        let trend = if trends.len() >= 2 {
            let first = trends.first().map(|t| t.total_incidents).unwrap_or(0);
            let last = trends.last().map(|t| t.total_incidents).unwrap_or(0);
            if first > last {
                "decreasing"
            } else if first < last {
                "increasing"
            } else {
                "stable"
            }
        } else {
            "stable"
        };

        Ok(IncidentTrendsDto {
            period: DateRangeDto {
                from: chrono::Utc::now()
                    .checked_sub_months(chrono::Months::new(months as u32))
                    .unwrap_or_else(|| chrono::Utc::now())
                    .to_rfc3339(),
                to: chrono::Utc::now().to_rfc3339(),
            },
            group_by: group_by.unwrap_or("month".to_string()),
            data: trends
                .into_iter()
                .map(|t| IncidentTrendDataDto {
                    month: t.month,
                    total_incidents: t.total_incidents as i32,
                    total_victims: t.total_victims as i32,
                    deaths: t.deaths as i32,
                    top_cause: t.top_cause.unwrap_or_else(|| "Unknown".to_string()),
                })
                .collect(),
            summary: IncidentSummaryDto {
                total_incidents,
                total_victims,
                total_deaths,
                most_common_cause,
                trend: trend.to_string(),
            },
            last_updated: chrono::Utc::now().to_rfc3339(),
        })
    }
}
