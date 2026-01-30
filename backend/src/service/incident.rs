use serde::Serialize;
use serde_json::Value;
use sqlx::PgPool;
use uuid::Uuid;

use crate::database::model::Incident;
use crate::database::model::IncidentSeverity;
use crate::database::model::IncidentSource;
use crate::database::model::IncidentStatus;
use crate::database::table::IncidentTable;
use crate::database::table::Table;
use crate::error::AppError;

#[derive(Debug, Serialize)]
pub struct IncidentDto {
    pub id: Uuid,
    pub location: Option<String>,
    pub address: Option<String>, // Not in DB model, will mock or derive
    pub province: Option<String>,
    pub kabupaten: Option<String>, // Not in DB model
    pub date: String,
    #[serde(rename = "victims")]
    pub affected_count: i32,
    pub deaths: i32,
    pub hospitalized: i32, // Not in DB model
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
    pub resolved_at: Option<String>, // Not in DB model
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
    pub related_kitchen_name: Option<String>, // Need to fetch kitchen
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

pub struct IncidentService {
    pool: PgPool,
    incident_table: IncidentTable,
}

impl IncidentService {
    pub fn new(pool: PgPool) -> Self {
        Self {
            pool: pool.clone(),
            incident_table: IncidentTable::new(pool),
        }
    }

    pub async fn list_incidents(
        &self,
        limit: i64,
        offset: i64,
        status: Option<String>,
        province: Option<String>,
        date_from: Option<String>,
        date_to: Option<String>,
        min_victims: Option<i32>,
    ) -> Result<IncidentListResponse, AppError> {
        // TODO: Implement filtering in DB
        let all_incidents = self.incident_table.select_all().await?;

        let mut filtered: Vec<Incident> = all_incidents;

        if let Some(s) = status {
            // Simple string matching for now, ideally enum parsing
            // filtered.retain(|i| i.status.map(|st| format!("{:?}", st).to_lowercase()) == Some(s.to_lowercase()));
        }

        if let Some(p) = province {
            filtered.retain(|i| {
                i.province
                    .as_ref()
                    .map(|pr| pr.to_lowercase().contains(&p.to_lowercase()))
                    .unwrap_or(false)
            });
        }

        if let Some(mv) = min_victims {
            filtered.retain(|i| i.affected_count.unwrap_or(0) >= mv);
        }

        let total = filtered.len() as i64;
        let paginated: Vec<Incident> = filtered
            .into_iter()
            .skip(offset as usize)
            .take(limit as usize)
            .collect();

        let dtos = paginated.into_iter().map(|i| self.map_to_dto(i)).collect();

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
            .incident_table
            .select(&id)
            .await?
            .ok_or(AppError::NotFound("Incident not found".into()))?;

        let dto = self.map_to_dto(incident.clone());

        Ok(IncidentDetailDto {
            incident: dto,
            related_kitchen_name: Some("Unknown Kitchen".to_string()), // TODO: Fetch kitchen name
            description: incident.description,
            timeline: vec![],              // Mock
            affected_institutions: vec![], // Mock
            laboratory_results: None,      // Mock
            corrective_actions: vec![],    // Mock
            source_url: None,              // Mock
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
            address: None, // Mock
            province: i.province,
            kabupaten: None, // Mock
            date: i.date.to_string(),
            affected_count: i.affected_count.unwrap_or(0),
            deaths: i.deaths.unwrap_or(0),
            hospitalized: 0, // Mock
            cause: i.cause,
            status: i.status,
            severity: i.severity,
            coordinates: coords,
            related_kitchen_id: i.kitchen_id,
            source: i.source,
            created_at: i.created_at.unwrap_or_default().to_string(),
            updated_at: i.updated_at.unwrap_or_default().to_string(),
            resolved_at: None, // Mock
        }
    }
}
