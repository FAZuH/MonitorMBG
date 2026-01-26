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

impl Default for Institution {
    fn default() -> Self {
        Self {
            id: Uuid::new_v4(),
            name: String::default(),
            r#type: String::default(),
            address: None,
            city: None,
            province: None,
            phone: None,
            email: None,
            registration_number: None,
            verified: None,
            created_at: None,
            updated_at: None,
        }
    }
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
    pub password_hash: Option<String>,
    pub ktp_photo_hash: Option<String>,
    pub last_login: Option<NaiveDateTime>,
    pub created_at: Option<NaiveDateTime>,
    pub updated_at: Option<NaiveDateTime>,
}

impl Default for User {
    fn default() -> Self {
        Self {
            id: Uuid::new_v4(),
            name: String::default(),
            role: UserRole::Kitchen,
            unique_code: String::default(),
            phone: None,
            verified: None,
            institution_name: None,
            institution_id: None,
            password_hash: None,
            ktp_photo_hash: None,
            last_login: None,
            created_at: None,
            updated_at: None,
        }
    }
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

impl Default for Kitchen {
    fn default() -> Self {
        Self {
            id: Uuid::new_v4(),
            name: String::default(),
            address: None,
            city: None,
            province: None,
            location: None,
            r#type: None,
            meals_served: None,
            certifications: None,
            image_url: None,
            owner_id: None,
            created_at: None,
            updated_at: None,
        }
    }
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

impl Default for ComplianceMetric {
    fn default() -> Self {
        Self {
            id: Uuid::new_v4(),
            kitchen_id: Uuid::default(),
            hygiene_score: None,
            portion_compliance: None,
            nutrition_compliance: None,
            temperature_control: None,
            sla_performance: None,
            last_inspection_date: None,
            trend: None,
            created_at: None,
            updated_at: None,
        }
    }
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

impl Default for ChecklistItem {
    fn default() -> Self {
        Self {
            id: Uuid::new_v4(),
            compliance_metric_id: Uuid::default(),
            category: None,
            item: None,
            status: None,
            notes: None,
            created_at: None,
        }
    }
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

impl Default for Incident {
    fn default() -> Self {
        Self {
            id: Uuid::new_v4(),
            kitchen_id: Uuid::default(),
            r#type: IncidentType::Other,
            source: IncidentSource::Public,
            date: NaiveDateTime::default(),
            location: None,
            province: None,
            food_type: None,
            affected_count: None,
            deaths: None,
            cause: None,
            severity: IncidentSeverity::Minor,
            status: None,
            description: None,
            reported_by: None,
            map_coordinates: None,
            gps_coordinates: None,
            created_at: None,
            updated_at: None,
        }
    }
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

impl Default for Inspection {
    fn default() -> Self {
        Self {
            id: Uuid::new_v4(),
            kitchen_id: Uuid::default(),
            inspector_name: None,
            date: NaiveDateTime::default(),
            overall_score: None,
            recommendations: None,
            follow_up_status: None,
            attachments: None,
            next_inspection_date: None,
            created_at: None,
            updated_at: None,
        }
    }
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

impl Default for InspectionFinding {
    fn default() -> Self {
        Self {
            id: Uuid::new_v4(),
            inspection_id: Uuid::default(),
            category: FindingCategory::Observation,
            description: String::default(),
            evidence: None,
            correction_required: None,
            deadline: None,
            created_at: None,
        }
    }
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

impl Default for Complaint {
    fn default() -> Self {
        Self {
            id: Uuid::new_v4(),
            kitchen_id: Uuid::default(),
            category: ComplaintCategory::Other,
            description: String::default(),
            status: None,
            reported_by: None,
            reported_at: NaiveDateTime::default(),
            sla_deadline: None,
            assigned_to: None,
            resolution: None,
            satisfaction_rating: None,
            created_at: None,
            updated_at: None,
        }
    }
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

impl Default for ComplaintEvidence {
    fn default() -> Self {
        Self {
            id: Uuid::new_v4(),
            complaint_id: Uuid::default(),
            url: String::default(),
            timestamp: NaiveDateTime::default(),
            metadata_status: None,
            capture_method: None,
            created_at: None,
        }
    }
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

impl Default for ComplaintComment {
    fn default() -> Self {
        Self {
            id: Uuid::new_v4(),
            complaint_id: Uuid::default(),
            author_id: None,
            author_name: None,
            role: None,
            message: String::default(),
            created_at: None,
        }
    }
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

impl Default for Review {
    fn default() -> Self {
        Self {
            id: Uuid::new_v4(),
            kitchen_id: Uuid::default(),
            reviewer_id: Uuid::default(),
            reviewer_name: String::default(),
            reviewer_type: UserRole::School,
            taste_rating: Decimal::default(),
            hygiene_rating: Decimal::default(),
            freshness_rating: Decimal::default(),
            temperature_rating: Decimal::default(),
            packaging_rating: Decimal::default(),
            handling_rating: Decimal::default(),
            comment: String::default(),
            photos: None,
            verification_status: Some("unverified".to_string()),
            report_source: "public".to_string(),
            confidence_level: "low".to_string(),
            root_causes: None,
            evidence: None,
            dispute_status: Some("none".to_string()),
            verified: None,
            is_draft: None,
            created_at: None,
            updated_at: None,
        }
    }
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

impl Default for ReviewDisputeHistory {
    fn default() -> Self {
        Self {
            id: Uuid::new_v4(),
            review_id: Uuid::default(),
            timestamp: None,
            action: String::default(),
            by_user_id: None,
            by_user_code: None,
            notes: None,
        }
    }
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

impl Default for PerformanceBadge {
    fn default() -> Self {
        Self {
            id: Uuid::new_v4(),
            kitchen_id: Uuid::default(),
            r#type: "gold".to_string(),
            title: String::default(),
            description: String::default(),
            earned_date: NaiveDate::default(),
            created_at: None,
        }
    }
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

impl Default for AuditLog {
    fn default() -> Self {
        Self {
            id: Uuid::new_v4(),
            user_id: None,
            user_name: None,
            action: String::default(),
            entity_type: String::default(),
            entity_id: String::default(),
            timestamp: None,
            ip_address: None,
            metadata: None,
        }
    }
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

impl Default for Alert {
    fn default() -> Self {
        Self {
            id: Uuid::new_v4(),
            kitchen_id: None,
            r#type: AlertType::Compliance,
            severity: AlertSeverity::Low,
            title: String::default(),
            message: String::default(),
            timestamp: None,
            acknowledged: None,
            created_at: None,
        }
    }
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

impl Default for Notification {
    fn default() -> Self {
        Self {
            id: Uuid::new_v4(),
            title: String::default(),
            description: String::default(),
            category: "hygiene".to_string(),
            priority: "minor".to_string(),
            kitchen_code: None,
            school_code: None,
            review_id: None,
            status: Some("new".to_string()),
            target_role: "all".to_string(),
            created_by: String::default(),
            created_at: None,
            updated_at: None,
        }
    }
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

impl Default for NotificationAuditTrail {
    fn default() -> Self {
        Self {
            id: Uuid::new_v4(),
            notification_id: Uuid::default(),
            timestamp: None,
            action: String::default(),
            user_code: String::default(),
        }
    }
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

impl Default for Video {
    fn default() -> Self {
        Self {
            id: Uuid::new_v4(),
            youtube_id: String::default(),
            title: String::default(),
            description: None,
            category: None,
            duration: None,
            upload_date: None,
            thumbnail: None,
            haccp_relevance: None,
            created_at: None,
        }
    }
}
