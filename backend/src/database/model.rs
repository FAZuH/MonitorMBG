use chrono::{NaiveDate, NaiveDateTime};
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

// Enums

#[derive(Debug, Clone, Copy, PartialEq, Eq, sqlx::Type, Serialize, Deserialize)]
#[sqlx(type_name = "user_role_enum", rename_all = "lowercase")]
pub enum UserRole {
    Kitchen,
    Supplier,
    School,
    Admin,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, sqlx::Type, Serialize, Deserialize)]
#[sqlx(type_name = "compliance_trend_enum", rename_all = "lowercase")]
pub enum ComplianceTrend {
    Improving,
    Stable,
    Declining,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, sqlx::Type, Serialize, Deserialize)]
#[sqlx(type_name = "checklist_status_enum", rename_all = "lowercase")]
pub enum ChecklistStatus {
    Pass,
    Fail,
    Warning,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, sqlx::Type, Serialize, Deserialize)]
#[sqlx(type_name = "incident_type_enum", rename_all = "lowercase")]
pub enum IncidentType {
    Poisoning,
    Nutrition,
    Sanitation,
    Other,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, sqlx::Type, Serialize, Deserialize)]
#[sqlx(type_name = "incident_source_enum", rename_all = "lowercase")]
pub enum IncidentSource {
    Consumer,
    Inspector,
    Public,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, sqlx::Type, Serialize, Deserialize)]
#[sqlx(type_name = "incident_severity_enum", rename_all = "lowercase")]
pub enum IncidentSeverity {
    Minor,
    Major,
    Critical,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, sqlx::Type, Serialize, Deserialize)]
#[sqlx(type_name = "incident_status_enum", rename_all = "lowercase")]
pub enum IncidentStatus {
    Investigating,
    Resolved,
    Escalated,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, sqlx::Type, Serialize, Deserialize)]
#[sqlx(
    type_name = "inspection_follow_up_status_enum",
    rename_all = "kebab-case"
)] // 'in-progress'
pub enum InspectionFollowUpStatus {
    Pending,
    #[serde(rename = "in-progress")]
    InProgress,
    Completed,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, sqlx::Type, Serialize, Deserialize)]
#[sqlx(type_name = "finding_category_enum", rename_all = "lowercase")]
pub enum FindingCategory {
    Major,
    Minor,
    Observation,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, sqlx::Type, Serialize, Deserialize)]
#[sqlx(type_name = "complaint_category_enum", rename_all = "lowercase")]
pub enum ComplaintCategory {
    Hygiene,
    Taste,
    Portion,
    Temperature,
    Packaging,
    Other,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, sqlx::Type, Serialize, Deserialize)]
#[sqlx(type_name = "complaint_status_enum", rename_all = "kebab-case")] // 'in-progress'
pub enum ComplaintStatus {
    Pending,
    #[serde(rename = "in-progress")]
    InProgress,
    Resolved,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, sqlx::Type, Serialize, Deserialize)]
#[sqlx(type_name = "evidence_metadata_status_enum", rename_all = "lowercase")]
pub enum EvidenceMetadataStatus {
    Verified,
    Mismatch,
    Unverified,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, sqlx::Type, Serialize, Deserialize)]
#[sqlx(type_name = "evidence_capture_method_enum", rename_all = "lowercase")]
pub enum EvidenceCaptureMethod {
    Camera,
    Fallback,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, sqlx::Type, Serialize, Deserialize)]
#[sqlx(type_name = "alert_type_enum", rename_all = "lowercase")]
pub enum AlertType {
    Compliance,
    Incident,
    Inspection,
    Complaint,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, sqlx::Type, Serialize, Deserialize)]
#[sqlx(type_name = "alert_severity_enum", rename_all = "lowercase")]
pub enum AlertSeverity {
    Low,
    Medium,
    High,
    Critical,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, sqlx::Type, Serialize, Deserialize)]
#[sqlx(type_name = "kitchen_type_enum")] // Values have spaces and Title Case in DB definition
pub enum KitchenType {
    #[sqlx(rename = "Central Kitchen")]
    #[serde(rename = "Central Kitchen")]
    CentralKitchen,
    #[sqlx(rename = "Regional Kitchen")]
    #[serde(rename = "Regional Kitchen")]
    RegionalKitchen,
    #[sqlx(rename = "Satellite Kitchen")]
    #[serde(rename = "Satellite Kitchen")]
    SatelliteKitchen,
}

// Models

#[derive(FromRow, Serialize, Deserialize, Debug, Clone)]
pub struct Institution {
    #[serde(default)]
    pub id: Uuid,
    pub name: String,
    pub r#type: String, // school, supplier, kitchen, government
    pub address: Option<String>,
    pub city: Option<String>,
    pub province: Option<String>,
    pub phone: Option<String>,
    pub email: Option<String>,
    pub registration_number: Option<String>,
    pub verified: Option<bool>,
    pub created_at: Option<NaiveDateTime>,
    pub updated_at: Option<NaiveDateTime>,
}

#[derive(FromRow, Serialize, Deserialize, Debug, Clone)]
pub struct User {
    #[serde(default)]
    pub id: Uuid,
    pub name: String,
    pub role: UserRole,
    pub unique_code: String,
    pub phone: Option<String>,
    pub verified: Option<bool>,
    pub institution_name: Option<String>,
    pub institution_id: Option<Uuid>,
    pub ktp_photo_hash: Option<String>,
    pub last_login: Option<NaiveDateTime>,
    pub created_at: Option<NaiveDateTime>,
    pub updated_at: Option<NaiveDateTime>,
}

#[derive(FromRow, Serialize, Deserialize, Debug, Clone)]
pub struct Kitchen {
    #[serde(default)]
    pub id: Uuid,
    pub name: String,
    pub address: Option<String>,
    pub city: Option<String>,
    pub province: Option<String>,
    // location is generated, so strictly speaking read-only, but mapped here
    pub location: Option<String>,
    pub r#type: Option<KitchenType>,
    pub meals_served: Option<i32>,
    pub certifications: Option<serde_json::Value>,
    pub image_url: Option<String>,
    pub owner_id: Option<Uuid>,
    pub created_at: Option<NaiveDateTime>,
    pub updated_at: Option<NaiveDateTime>,
}

#[derive(FromRow, Serialize, Deserialize, Debug, Clone)]
pub struct ComplianceMetric {
    #[serde(default)]
    pub id: Uuid,
    pub kitchen_id: Uuid,
    pub hygiene_score: Option<Decimal>, // DECIMAL
    pub portion_compliance: Option<Decimal>,
    pub nutrition_compliance: Option<Decimal>,
    pub temperature_control: Option<Decimal>,
    pub sla_performance: Option<serde_json::Value>,
    pub last_inspection_date: Option<NaiveDateTime>,
    pub trend: Option<ComplianceTrend>,
    pub created_at: Option<NaiveDateTime>,
    pub updated_at: Option<NaiveDateTime>,
}

#[derive(FromRow, Serialize, Deserialize, Debug, Clone)]
pub struct ChecklistItem {
    #[serde(default)]
    pub id: Uuid,
    pub compliance_metric_id: Uuid,
    pub category: Option<String>,
    pub item: Option<String>,
    pub status: Option<ChecklistStatus>,
    pub notes: Option<String>,
    pub created_at: Option<NaiveDateTime>,
}

#[derive(FromRow, Serialize, Deserialize, Debug, Clone)]
pub struct Incident {
    #[serde(default)]
    pub id: Uuid,
    pub kitchen_id: Uuid,
    pub r#type: IncidentType,
    pub source: IncidentSource,
    pub date: NaiveDateTime,
    pub location: Option<String>,
    pub province: Option<String>,
    pub food_type: Option<String>,
    pub affected_count: Option<i32>,
    pub deaths: Option<i32>,
    pub cause: Option<String>,
    pub severity: IncidentSeverity,
    pub status: Option<IncidentStatus>,
    pub description: Option<String>,
    pub reported_by: Option<String>,
    pub map_coordinates: Option<serde_json::Value>,
    pub gps_coordinates: Option<serde_json::Value>,
    pub created_at: Option<NaiveDateTime>,
    pub updated_at: Option<NaiveDateTime>,
}

#[derive(FromRow, Serialize, Deserialize, Debug, Clone)]
pub struct Inspection {
    #[serde(default)]
    pub id: Uuid,
    pub kitchen_id: Uuid,
    pub inspector_name: Option<String>,
    pub date: NaiveDateTime,
    pub overall_score: Option<Decimal>,
    pub recommendations: Option<serde_json::Value>,
    pub follow_up_status: Option<InspectionFollowUpStatus>,
    pub attachments: Option<serde_json::Value>,
    pub next_inspection_date: Option<NaiveDateTime>,
    pub created_at: Option<NaiveDateTime>,
    pub updated_at: Option<NaiveDateTime>,
}

#[derive(FromRow, Serialize, Deserialize, Debug, Clone)]
pub struct InspectionFinding {
    #[serde(default)]
    pub id: Uuid,
    pub inspection_id: Uuid,
    pub category: FindingCategory,
    pub description: String,
    pub evidence: Option<String>,
    pub correction_required: Option<bool>,
    pub deadline: Option<NaiveDateTime>,
    pub created_at: Option<NaiveDateTime>,
}

#[derive(FromRow, Serialize, Deserialize, Debug, Clone)]
pub struct Complaint {
    #[serde(default)]
    pub id: Uuid,
    pub kitchen_id: Uuid,
    pub category: ComplaintCategory,
    pub description: String,
    pub status: Option<ComplaintStatus>,
    pub reported_by: Option<String>,
    pub reported_at: NaiveDateTime,
    pub sla_deadline: Option<NaiveDateTime>,
    pub assigned_to: Option<Uuid>,
    pub resolution: Option<String>,
    pub satisfaction_rating: Option<i32>,
    pub created_at: Option<NaiveDateTime>,
    pub updated_at: Option<NaiveDateTime>,
}

#[derive(FromRow, Serialize, Deserialize, Debug, Clone)]
pub struct ComplaintEvidence {
    #[serde(default)]
    pub id: Uuid,
    pub complaint_id: Uuid,
    pub url: String,
    pub timestamp: NaiveDateTime,
    pub metadata_status: Option<EvidenceMetadataStatus>,
    pub capture_method: Option<EvidenceCaptureMethod>,
    pub created_at: Option<NaiveDateTime>,
}

#[derive(FromRow, Serialize, Deserialize, Debug, Clone)]
pub struct ComplaintComment {
    #[serde(default)]
    pub id: Uuid,
    pub complaint_id: Uuid,
    pub author_id: Option<Uuid>,
    pub author_name: Option<String>,
    pub role: Option<String>,
    pub message: String,
    pub created_at: Option<NaiveDateTime>,
}

#[derive(FromRow, Serialize, Deserialize, Debug, Clone)]
pub struct Review {
    #[serde(default)]
    pub id: Uuid,
    pub kitchen_id: Uuid,
    pub reviewer_id: Uuid,
    pub reviewer_name: String,
    pub reviewer_type: UserRole,
    pub taste_rating: Decimal,
    pub hygiene_rating: Decimal,
    pub freshness_rating: Decimal,
    pub temperature_rating: Decimal,
    pub packaging_rating: Decimal,
    pub handling_rating: Decimal,
    pub comment: String,
    pub photos: Option<serde_json::Value>,
    pub verification_status: Option<String>,
    pub report_source: String,
    pub confidence_level: String,
    pub root_causes: Option<serde_json::Value>,
    pub evidence: Option<serde_json::Value>,
    pub dispute_status: Option<String>,
    pub verified: Option<bool>,
    pub is_draft: Option<bool>,
    pub created_at: Option<NaiveDateTime>,
    pub updated_at: Option<NaiveDateTime>,
}

#[derive(FromRow, Serialize, Deserialize, Debug, Clone)]
pub struct ReviewDisputeHistory {
    #[serde(default)]
    pub id: Uuid,
    pub review_id: Uuid,
    pub timestamp: Option<NaiveDateTime>,
    pub action: String,
    pub by_user_id: Option<Uuid>,
    pub by_user_code: Option<String>,
    pub notes: Option<String>,
}

#[derive(FromRow, Serialize, Deserialize, Debug, Clone)]
pub struct PerformanceBadge {
    #[serde(default)]
    pub id: Uuid,
    pub kitchen_id: Uuid,
    pub r#type: String,
    pub title: String,
    pub description: String,
    pub earned_date: NaiveDate,
    pub created_at: Option<NaiveDateTime>,
}

#[derive(FromRow, Serialize, Deserialize, Debug, Clone)]
pub struct AuditLog {
    #[serde(default)]
    pub id: Uuid,
    pub user_id: Option<Uuid>,
    pub user_name: Option<String>,
    pub action: String,
    pub entity_type: String,
    pub entity_id: String,
    pub timestamp: Option<NaiveDateTime>,
    pub ip_address: Option<String>,
    pub metadata: Option<serde_json::Value>,
}

#[derive(FromRow, Serialize, Deserialize, Debug, Clone)]
pub struct Alert {
    #[serde(default)]
    pub id: Uuid,
    pub kitchen_id: Option<Uuid>,
    pub r#type: AlertType,
    pub severity: AlertSeverity,
    pub title: String,
    pub message: String,
    pub timestamp: Option<NaiveDateTime>,
    pub acknowledged: Option<bool>,
    pub created_at: Option<NaiveDateTime>,
}

#[derive(FromRow, Serialize, Deserialize, Debug, Clone)]
pub struct Notification {
    #[serde(default)]
    pub id: Uuid,
    pub title: String,
    pub description: String,
    pub category: String,
    pub priority: String,
    pub kitchen_code: Option<String>,
    pub school_code: Option<String>,
    pub review_id: Option<Uuid>,
    pub status: Option<String>,
    pub target_role: String,
    pub created_by: String,
    pub created_at: Option<NaiveDateTime>,
    pub updated_at: Option<NaiveDateTime>,
}

#[derive(FromRow, Serialize, Deserialize, Debug, Clone)]
pub struct NotificationAuditTrail {
    #[serde(default)]
    pub id: Uuid,
    pub notification_id: Uuid,
    pub timestamp: Option<NaiveDateTime>,
    pub action: String,
    pub user_code: String,
}

#[derive(FromRow, Serialize, Deserialize, Debug, Clone)]
pub struct Video {
    #[serde(default)]
    pub id: Uuid,
    pub youtube_id: String,
    pub title: String,
    pub description: Option<String>,
    pub category: Option<String>,
    pub duration: Option<String>,
    pub upload_date: Option<NaiveDate>,
    pub thumbnail: Option<String>,
    pub haccp_relevance: Option<String>,
    pub created_at: Option<NaiveDateTime>,
}
