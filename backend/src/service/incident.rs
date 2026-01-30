//! Incident reporting and management service.

use std::sync::Arc;

use serde::Serialize;
use serde_json::Value;
use uuid::Uuid;

use crate::database::Database;
use crate::database::model::Incident;
use crate::database::model::IncidentSeverity;
use crate::database::model::IncidentSource;
use crate::database::model::IncidentStatus;
use crate::database::table::Table;
use crate::error::AppError;

#[derive(Debug, Serialize)]
pub struct IncidentDto {
    pub id: Uuid,
    pub location: Option<String>,
    pub address: Option<String>,
    pub province: Option<String>,
    pub kabupaten: Option<String>,
    pub date: String,
    #[serde(rename = "victims")]
    pub affected_count: i32,
    pub deaths: i32,
    pub hospitalized: i32,
    pub cause: Option<String>,
    pub status: Option<IncidentStatus>,
    pub severity: IncidentSeverity,
    pub coordinates: Option<CoordinatesDto>,
    #[serde(rename = "relatedKitchenId")]
    pub related_kitchen_id: Uuid,
    pub source: IncidentSource,
    #[serde(rename = "createdAt")]
    pub created_at: String,
    #[serde(rename = "updatedAt")]
    pub updated_at: String,
    #[serde(rename = "resolvedAt")]
    pub resolved_at: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct CoordinatesDto {
    pub lat: f64,
    pub lng: f64,
}

#[derive(Debug, Serialize)]
pub struct IncidentDetailDto {
    #[serde(flatten)]
    pub incident: IncidentDto,
    #[serde(rename = "relatedKitchenName")]
    pub related_kitchen_name: Option<String>,
    pub description: Option<String>,
    pub timeline: Vec<TimelineEventDto>,
    #[serde(rename = "affectedInstitutions")]
    pub affected_institutions: Vec<String>,
    #[serde(rename = "laboratoryResults")]
    pub laboratory_results: Option<LaboratoryResultsDto>,
    #[serde(rename = "correctiveActions")]
    pub corrective_actions: Vec<String>,
    #[serde(rename = "sourceUrl")]
    pub source_url: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct TimelineEventDto {
    pub date: String,
    pub event: String,
    pub description: String,
}

#[derive(Debug, Serialize)]
pub struct LaboratoryResultsDto {
    pub pathogen: String,
    #[serde(rename = "testDate")]
    pub test_date: String,
    #[serde(rename = "confirmedBy")]
    pub confirmed_by: String,
}

#[derive(Debug, Serialize)]
pub struct IncidentListResponse {
    pub data: Vec<IncidentDto>,
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

/// Service for handling food safety incidents and reports.
pub struct IncidentService {
    db: Arc<Database>,
}

impl IncidentService {
    /// Creates a new `IncidentService`.
    pub fn new(db: Arc<Database>) -> Self {
        Self { db }
    }

    /// Lists incidents with optional filtering and pagination.
    pub async fn list_incidents(
        &self,
        limit: i64,
        offset: i64,
        status: Option<String>,
        province: Option<String>,
        _date_from: Option<String>,
        _date_to: Option<String>,
        min_victims: Option<i32>,
    ) -> Result<IncidentListResponse, AppError> {
        let (incidents, total) = self
            .db
            .incident_table
            .list_incidents(
                status.as_deref(),
                province.as_deref(),
                min_victims,
                limit,
                offset,
            )
            .await?;

        let dtos = incidents.into_iter().map(|i| self.map_to_dto(i)).collect();

        Ok(IncidentListResponse {
            data: dtos,
            pagination: Pagination {
                total,
                limit,
                offset,
                has_more: offset + limit < total,
            },
        })
    }

    pub async fn get_incident_detail(&self, id: Uuid) -> Result<IncidentDetailDto, AppError> {
        let incident = self
            .db
            .incident_table
            .select(&id)
            .await?
            .ok_or(AppError::NotFound("Incident not found".into()))?;

        // Get kitchen name
        let kitchen_name = self
            .db
            .kitchen_table
            .get_kitchen_name(&incident.kitchen_id)
            .await?;

        // Get timeline events
        let timeline_events = self
            .db
            .incident_table
            .get_timeline_events(&id)
            .await?;

        let timeline = timeline_events
            .into_iter()
            .map(|e| TimelineEventDto {
                date: e.event_date.to_string(),
                event: e.event_title,
                description: e.description.unwrap_or_default(),
            })
            .collect();

        // Get lab results
        let lab_results = self
            .db
            .incident_table
            .get_lab_results(&id)
            .await?;

        let laboratory_results = lab_results.map(|lr| LaboratoryResultsDto {
            pathogen: lr.pathogen,
            test_date: lr.test_date.to_string(),
            confirmed_by: lr.confirmed_by,
        });

        // Get affected institutions
        let affected_institutions = self
            .db
            .incident_table
            .get_affected_institutions(&id)
            .await?;

        // Get corrective actions
        let corrective_actions = self
            .db
            .incident_table
            .get_corrective_actions(&id)
            .await?;

        let dto = self.map_to_dto(incident);

        Ok(IncidentDetailDto {
            incident: dto,
            related_kitchen_name: kitchen_name,
            description: None, // Not in base incident model
            timeline,
            affected_institutions,
            laboratory_results,
            corrective_actions,
            source_url: None, // Not in base incident model
        })
    }

    fn map_to_dto(&self, i: Incident) -> IncidentDto {
        let coords = if let Some(Value::Object(map)) = i.map_coordinates {
            if let (Some(Value::Number(lat)), Some(Value::Number(lng))) =
                (map.get("lat"), map.get("lng"))
            {
                Some(CoordinatesDto {
                    lat: lat.as_f64().unwrap_or_default(),
                    lng: lng.as_f64().unwrap_or_default(),
                })
            } else {
                None
            }
        } else {
            None
        };

        IncidentDto {
            id: i.id,
            location: i.location,
            address: None, // Will be populated from kitchen in detail view
            province: i.province,
            kabupaten: None, // Will be populated from kitchen in detail view
            date: i.date.to_string(),
            affected_count: i.affected_count.unwrap_or(0),
            deaths: i.deaths.unwrap_or(0),
            hospitalized: 0, // Not in DB model
            cause: i.cause,
            status: i.status,
            severity: i.severity,
            coordinates: coords,
            related_kitchen_id: i.kitchen_id,
            source: i.source,
            created_at: i.created_at.unwrap_or_default().to_string(),
            updated_at: i.updated_at.unwrap_or_default().to_string(),
            resolved_at: if i.status == Some(IncidentStatus::Resolved) {
                i.updated_at.map(|d| d.to_string())
            } else {
                None
            },
        }
    }
}
