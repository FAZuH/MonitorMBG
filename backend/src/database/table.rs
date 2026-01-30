use async_trait::async_trait;
use sqlx::PgPool;
use sqlx::Postgres as Db;
use sqlx::postgres::PgArguments as Arguments;
use uuid::Uuid;

use crate::database::error::DatabaseError;
use crate::database::model::*;

type QA<'q, O> = sqlx::query::QueryAs<'q, Db, O, Arguments>;
type Q<'q> = sqlx::query::Query<'q, Db, Arguments>;

pub struct BaseTable {
    pub pool: PgPool,
}

impl BaseTable {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
pub trait TableBase {
    async fn create_table(&self) -> Result<(), DatabaseError>;
    async fn drop_table(&self) -> Result<(), DatabaseError>;
    async fn delete_all(&self) -> Result<(), DatabaseError>;
}

#[async_trait]
pub trait Table<T, ID>: TableBase {
    async fn select_all(&self) -> Result<Vec<T>, DatabaseError>;
    async fn insert(&self, model: &T) -> Result<ID, DatabaseError>;
    async fn select(&self, id: &ID) -> Result<Option<T>, DatabaseError>;
    async fn update(&self, model: &T) -> Result<(), DatabaseError>;
    async fn delete(&self, id: &ID) -> Result<(), DatabaseError>;
    async fn replace(&self, model: &T) -> Result<ID, DatabaseError>;
}

// Helper trait to handle binding parameters
pub trait BindParam<'q> {
    fn bind_param<O>(self, query: QA<'q, O>) -> QA<'q, O>;
    fn bind_param_q(self, query: Q<'q>) -> Q<'q>;
}

// Generic implementation for any type that can be bound in Postgres
impl<'q, T> BindParam<'q> for &'q T
where
    T: sqlx::Type<Db> + sqlx::Encode<'q, Db> + Send + Sync,
{
    fn bind_param<O>(self, query: QA<'q, O>) -> QA<'q, O> {
        query.bind(self)
    }
    fn bind_param_q(self, query: Q<'q>) -> Q<'q> {
        query.bind(self)
    }
}

// Special handling for Option<T> where T implements Encode
// Actually the generic impl above covers Option<T> because Option<T> implements Encode if T does.
// But we need to make sure strict types like &str vs String don't conflict.
// sqlx handles most.

macro_rules! impl_table {
    (
        $struct_name:ident,
        $model:ty,
        $table:expr,
        $pk:ident,
        $id_type:ty,
        $db_id_type:ty,
        $create_sql:expr,
        $cols:expr,
        $vals:expr,
        $update_set:expr,
        [ $( $field:ident ),+ ]
    ) => {
        pub struct $struct_name {
            base: BaseTable,
        }

        impl $struct_name {
            pub fn new(pool: PgPool) -> Self {
                Self {
                    base: BaseTable::new(pool),
                }
            }
        }

        #[async_trait]
        impl TableBase for $struct_name {
            async fn create_table(&self) -> Result<(), DatabaseError> {
                sqlx::query($create_sql)
                    .execute(&self.base.pool)
                    .await?;
                Ok(())
            }

            async fn drop_table(&self) -> Result<(), DatabaseError> {
                sqlx::query(concat!("DROP TABLE IF EXISTS ", $table))
                    .execute(&self.base.pool)
                    .await?;
                Ok(())
            }

            async fn delete_all(&self) -> Result<(), DatabaseError> {
                sqlx::query(concat!("DELETE FROM ", $table))
                    .execute(&self.base.pool)
                    .await?;
                Ok(())
            }
        }

        #[async_trait]
        impl Table<$model, $id_type> for $struct_name {
            async fn select_all(&self) -> Result<Vec<$model>, DatabaseError> {
                Ok(sqlx::query_as::<_, $model>(concat!("SELECT * FROM ", $table))
                    .fetch_all(&self.base.pool)
                    .await?)
            }

            async fn select(&self, id: &$id_type) -> Result<Option<$model>, DatabaseError> {
                let query = sqlx::query_as::<_, $model>(concat!("SELECT * FROM ", $table, " WHERE ", stringify!($pk), " = $1"));
                let query = BindParam::bind_param(id, query);
                Ok(
                    query
                        .fetch_optional(&self.base.pool)
                        .await?,
                )
            }

            async fn insert(&self, model: &$model) -> Result<$id_type, DatabaseError> {
                let mut query = sqlx::query_as(concat!(
                        "INSERT INTO ", $table, " (", $cols, ") VALUES (", $vals, ") RETURNING ", stringify!($pk)
                    ));

                $(
                    query = BindParam::bind_param(&model.$field, query);
                )+

                let row: ($db_id_type,) = query.fetch_one(&self.base.pool).await?;
                Ok(row.0 as $id_type)
            }

            async fn update(&self, model: &$model) -> Result<(), DatabaseError> {
                let mut query = sqlx::query(concat!(
                        "UPDATE ", $table, " SET ", $update_set
                    ));

                $(
                    query = BindParam::bind_param_q(&model.$field, query);
                )+
                query = BindParam::bind_param_q(&model.$pk, query);

                query.execute(&self.base.pool).await?;
                Ok(())
            }

            async fn delete(&self, id: &$id_type) -> Result<(), DatabaseError> {
                let query = sqlx::query(concat!("DELETE FROM ", $table, " WHERE ", stringify!($pk), " = $1"));
                let query = BindParam::bind_param_q(id, query);
                query.execute(&self.base.pool).await?;
                Ok(())
            }

            async fn replace(&self, _model: &$model) -> Result<$id_type, DatabaseError> {
                Err(DatabaseError::InternalError { message: "Replace not supported in Postgres, use Update or Insert".into() })
            }
        }
    };
}

impl UserTable {
    pub async fn find_by_unique_code(
        &self,
        unique_code: &str,
    ) -> Result<Option<User>, DatabaseError> {
        Ok(sqlx::query_as::<_, User>(
            r#"
            SELECT 
                id, name, role, unique_code, phone, verified, 
                institution_name, institution_id, ktp_photo_hash, last_login, 
                created_at, updated_at, password_hash
            FROM users 
            WHERE unique_code = $1
            "#,
        )
        .bind(unique_code)
        .fetch_optional(&self.base.pool)
        .await?)
    }
}

impl_table!(
    InstitutionTable,
    Institution,
    "institutions",
    id,
    Uuid,
    Uuid,
    r#"CREATE TABLE IF NOT EXISTS institutions (
        id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
        name VARCHAR(255) NOT NULL,
        type VARCHAR(50) NOT NULL,
        address TEXT,
        city VARCHAR(100),
        province VARCHAR(100),
        phone VARCHAR(20),
        email VARCHAR(255),
        registration_number VARCHAR(100) UNIQUE,
        verified BOOLEAN DEFAULT FALSE,
        created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
        updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
    )"#,
    "name, type, address, city, province, phone, email, registration_number, verified",
    "$1, $2, $3, $4, $5, $6, $7, $8, $9",
    "name=$1, type=$2, address=$3, city=$4, province=$5, phone=$6, email=$7, registration_number=$8, verified=$9 WHERE id=$10",
    [
        name,
        r#type,
        address,
        city,
        province,
        phone,
        email,
        registration_number,
        verified
    ]
);

impl_table!(
    UserTable,
    User,
    "users",
    id,
    Uuid,
    Uuid,
    r#"CREATE TABLE IF NOT EXISTS users (
        id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
        name VARCHAR(255) NOT NULL,
        role user_role_enum NOT NULL,
        unique_code VARCHAR(50) UNIQUE NOT NULL,
        phone VARCHAR(20),
        verified BOOLEAN DEFAULT FALSE,
        institution_name VARCHAR(255),
        institution_id UUID,
        password_hash VARCHAR(255),
        ktp_photo_hash VARCHAR(255),
        last_login TIMESTAMP,
        created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
        updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
    )"#,
    "name, role, unique_code, phone, verified, institution_name, institution_id, password_hash, ktp_photo_hash, last_login",
    "$1, $2, $3, $4, $5, $6, $7, $8, $9, $10",
    "name=$1, role=$2, unique_code=$3, phone=$4, verified=$5, institution_name=$6, institution_id=$7, password_hash=$8, ktp_photo_hash=$9, last_login=$10 WHERE id=$11",
    [
        name,
        role,
        unique_code,
        phone,
        verified,
        institution_name,
        institution_id,
        password_hash,
        ktp_photo_hash,
        last_login
    ]
);

// Kitchen Table
impl_table!(
    KitchenTable,
    Kitchen,
    "kitchens",
    id,
    Uuid,
    Uuid,
    r#"CREATE TABLE IF NOT EXISTS kitchens (
        id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
        name VARCHAR(255) NOT NULL,
        address TEXT,
        city VARCHAR(100),
        province VARCHAR(100),
        type kitchen_type_enum DEFAULT 'Central Kitchen',
        meals_served INTEGER DEFAULT 0,
        certifications JSONB,
        image_url VARCHAR(500),
        owner_id UUID,
        created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
        updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
    )"#,
    "name, address, city, province, type, meals_served, certifications, image_url, owner_id",
    "$1, $2, $3, $4, $5, $6, $7, $8, $9",
    "name=$1, address=$2, city=$3, province=$4, type=$5, meals_served=$6, certifications=$7, image_url=$8, owner_id=$9 WHERE id=$10",
    [
        name,
        address,
        city,
        province,
        r#type,
        meals_served,
        certifications,
        image_url,
        owner_id
    ]
);

// ComplianceMetric Table
impl_table!(
    ComplianceMetricTable,
    ComplianceMetric,
    "compliance_metrics",
    id,
    Uuid,
    Uuid,
    r#"CREATE TABLE IF NOT EXISTS compliance_metrics (
        id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
        kitchen_id UUID NOT NULL,
        hygiene_score DECIMAL(5, 2),
        portion_compliance DECIMAL(5, 2),
        nutrition_compliance DECIMAL(5, 2),
        temperature_control DECIMAL(5, 2),
        sla_performance JSONB,
        last_inspection_date TIMESTAMP,
        trend compliance_trend_enum,
        created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
        updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
    )"#,
    "kitchen_id, hygiene_score, portion_compliance, nutrition_compliance, temperature_control, sla_performance, last_inspection_date, trend",
    "$1, $2, $3, $4, $5, $6, $7, $8",
    "kitchen_id=$1, hygiene_score=$2, portion_compliance=$3, nutrition_compliance=$4, temperature_control=$5, sla_performance=$6, last_inspection_date=$7, trend=$8 WHERE id=$9",
    [
        kitchen_id,
        hygiene_score,
        portion_compliance,
        nutrition_compliance,
        temperature_control,
        sla_performance,
        last_inspection_date,
        trend
    ]
);

// ChecklistItem Table
impl_table!(
    ChecklistItemTable,
    ChecklistItem,
    "checklist_items",
    id,
    Uuid,
    Uuid,
    r#"CREATE TABLE IF NOT EXISTS checklist_items (
        id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
        compliance_metric_id UUID NOT NULL,
        category VARCHAR(100),
        item VARCHAR(255),
        status checklist_status_enum,
        notes TEXT,
        created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
    )"#,
    "compliance_metric_id, category, item, status, notes",
    "$1, $2, $3, $4, $5",
    "compliance_metric_id=$1, category=$2, item=$3, status=$4, notes=$5 WHERE id=$6",
    [compliance_metric_id, category, item, status, notes]
);

// Incident Table
impl_table!(
    IncidentTable,
    Incident,
    "incidents",
    id,
    Uuid,
    Uuid,
    r#"CREATE TABLE IF NOT EXISTS incidents (
        id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
        kitchen_id UUID NOT NULL,
        type incident_type_enum NOT NULL,
        source incident_source_enum NOT NULL,
        date TIMESTAMP NOT NULL,
        location VARCHAR(255),
        province VARCHAR(100),
        food_type VARCHAR(100),
        affected_count INTEGER DEFAULT 0,
        deaths INTEGER DEFAULT 0,
        cause TEXT,
        severity incident_severity_enum NOT NULL,
        status incident_status_enum DEFAULT 'investigating',
        description TEXT,
        reported_by VARCHAR(255),
        map_coordinates JSONB,
        gps_coordinates JSONB,
        created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
        updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
    )"#,
    "kitchen_id, type, source, date, location, province, food_type, affected_count, deaths, cause, severity, status, description, reported_by, map_coordinates, gps_coordinates",
    "$1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14, $15, $16",
    "kitchen_id=$1, type=$2, source=$3, date=$4, location=$5, province=$6, food_type=$7, affected_count=$8, deaths=$9, cause=$10, severity=$11, status=$12, description=$13, reported_by=$14, map_coordinates=$15, gps_coordinates=$16 WHERE id=$17",
    [
        kitchen_id,
        r#type,
        source,
        date,
        location,
        province,
        food_type,
        affected_count,
        deaths,
        cause,
        severity,
        status,
        description,
        reported_by,
        map_coordinates,
        gps_coordinates
    ]
);

// Inspection Table
impl_table!(
    InspectionTable,
    Inspection,
    "inspections",
    id,
    Uuid,
    Uuid,
    r#"CREATE TABLE IF NOT EXISTS inspections (
        id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
        kitchen_id UUID NOT NULL,
        inspector_name VARCHAR(255),
        date TIMESTAMP NOT NULL,
        overall_score DECIMAL(5, 2),
        recommendations JSONB,
        follow_up_status inspection_follow_up_status_enum,
        attachments JSONB,
        next_inspection_date TIMESTAMP,
        created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
        updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
    )"#,
    "kitchen_id, inspector_name, date, overall_score, recommendations, follow_up_status, attachments, next_inspection_date",
    "$1, $2, $3, $4, $5, $6, $7, $8",
    "kitchen_id=$1, inspector_name=$2, date=$3, overall_score=$4, recommendations=$5, follow_up_status=$6, attachments=$7, next_inspection_date=$8 WHERE id=$9",
    [
        kitchen_id,
        inspector_name,
        date,
        overall_score,
        recommendations,
        follow_up_status,
        attachments,
        next_inspection_date
    ]
);

// InspectionFinding Table
impl_table!(
    InspectionFindingTable,
    InspectionFinding,
    "inspection_findings",
    id,
    Uuid,
    Uuid,
    r#"CREATE TABLE IF NOT EXISTS inspection_findings (
        id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
        inspection_id UUID NOT NULL,
        category finding_category_enum NOT NULL,
        description TEXT NOT NULL,
        evidence VARCHAR(255),
        correction_required BOOLEAN DEFAULT FALSE,
        deadline TIMESTAMP,
        created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
    )"#,
    "inspection_id, category, description, evidence, correction_required, deadline",
    "$1, $2, $3, $4, $5, $6",
    "inspection_id=$1, category=$2, description=$3, evidence=$4, correction_required=$5, deadline=$6 WHERE id=$7",
    [
        inspection_id,
        category,
        description,
        evidence,
        correction_required,
        deadline
    ]
);

// Complaint Table
impl_table!(
    ComplaintTable,
    Complaint,
    "complaints",
    id,
    Uuid,
    Uuid,
    r#"CREATE TABLE IF NOT EXISTS complaints (
        id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
        kitchen_id UUID NOT NULL,
        category complaint_category_enum NOT NULL,
        description TEXT NOT NULL,
        status complaint_status_enum DEFAULT 'pending',
        reported_by VARCHAR(255),
        reported_at TIMESTAMP NOT NULL,
        sla_deadline TIMESTAMP,
        assigned_to UUID,
        resolution TEXT,
        satisfaction_rating INTEGER,
        created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
        updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
    )"#,
    "kitchen_id, category, description, status, reported_by, reported_at, sla_deadline, assigned_to, resolution, satisfaction_rating",
    "$1, $2, $3, $4, $5, $6, $7, $8, $9, $10",
    "kitchen_id=$1, category=$2, description=$3, status=$4, reported_by=$5, reported_at=$6, sla_deadline=$7, assigned_to=$8, resolution=$9, satisfaction_rating=$10 WHERE id=$11",
    [
        kitchen_id,
        category,
        description,
        status,
        reported_by,
        reported_at,
        sla_deadline,
        assigned_to,
        resolution,
        satisfaction_rating
    ]
);

// ComplaintEvidence Table
impl_table!(
    ComplaintEvidenceTable,
    ComplaintEvidence,
    "complaint_evidence",
    id,
    Uuid,
    Uuid,
    r#"CREATE TABLE IF NOT EXISTS complaint_evidence (
        id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
        complaint_id UUID NOT NULL,
        url VARCHAR(255) NOT NULL,
        timestamp TIMESTAMP NOT NULL,
        metadata_status evidence_metadata_status_enum,
        capture_method evidence_capture_method_enum,
        created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
    )"#,
    "complaint_id, url, timestamp, metadata_status, capture_method",
    "$1, $2, $3, $4, $5",
    "complaint_id=$1, url=$2, timestamp=$3, metadata_status=$4, capture_method=$5 WHERE id=$6",
    [
        complaint_id,
        url,
        timestamp,
        metadata_status,
        capture_method
    ]
);

// ComplaintComment Table
impl_table!(
    ComplaintCommentTable,
    ComplaintComment,
    "complaint_comments",
    id,
    Uuid,
    Uuid,
    r#"CREATE TABLE IF NOT EXISTS complaint_comments (
        id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
        complaint_id UUID NOT NULL,
        author_id UUID,
        author_name VARCHAR(255),
        role VARCHAR(50),
        message TEXT NOT NULL,
        created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
    )"#,
    "complaint_id, author_id, author_name, role, message",
    "$1, $2, $3, $4, $5",
    "complaint_id=$1, author_id=$2, author_name=$3, role=$4, message=$5 WHERE id=$6",
    [complaint_id, author_id, author_name, role, message]
);

// Review Table
impl_table!(
    ReviewTable,
    Review,
    "reviews",
    id,
    Uuid,
    Uuid,
    r#"CREATE TABLE IF NOT EXISTS reviews (
        id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
        kitchen_id UUID NOT NULL,
        reviewer_id UUID NOT NULL,
        reviewer_name VARCHAR(255) NOT NULL,
        reviewer_type user_role_enum NOT NULL,
        taste_rating DECIMAL(2,1) NOT NULL,
        hygiene_rating DECIMAL(2,1) NOT NULL,
        freshness_rating DECIMAL(2,1) NOT NULL,
        temperature_rating DECIMAL(2,1) NOT NULL,
        packaging_rating DECIMAL(2,1) NOT NULL,
        handling_rating DECIMAL(2,1) NOT NULL,
        comment TEXT NOT NULL,
        photos JSONB,
        verification_status VARCHAR(20) DEFAULT 'unverified',
        report_source VARCHAR(20) NOT NULL,
        confidence_level VARCHAR(10) NOT NULL,
        root_causes JSONB,
        evidence JSONB,
        dispute_status VARCHAR(20) DEFAULT 'none',
        verified BOOLEAN DEFAULT FALSE,
        is_draft BOOLEAN DEFAULT FALSE,
        created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
        updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
    )"#,
    "kitchen_id, reviewer_id, reviewer_name, reviewer_type, taste_rating, hygiene_rating, freshness_rating, temperature_rating, packaging_rating, handling_rating, comment, photos, verification_status, report_source, confidence_level, root_causes, evidence, dispute_status, verified, is_draft",
    "$1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14, $15, $16, $17, $18, $19, $20",
    "kitchen_id=$1, reviewer_id=$2, reviewer_name=$3, reviewer_type=$4, taste_rating=$5, hygiene_rating=$6, freshness_rating=$7, temperature_rating=$8, packaging_rating=$9, handling_rating=$10, comment=$11, photos=$12, verification_status=$13, report_source=$14, confidence_level=$15, root_causes=$16, evidence=$17, dispute_status=$18, verified=$19, is_draft=$20 WHERE id=$21",
    [
        kitchen_id,
        reviewer_id,
        reviewer_name,
        reviewer_type,
        taste_rating,
        hygiene_rating,
        freshness_rating,
        temperature_rating,
        packaging_rating,
        handling_rating,
        comment,
        photos,
        verification_status,
        report_source,
        confidence_level,
        root_causes,
        evidence,
        dispute_status,
        verified,
        is_draft
    ]
);

// ReviewDisputeHistory Table
impl_table!(
    ReviewDisputeHistoryTable,
    ReviewDisputeHistory,
    "review_dispute_history",
    id,
    Uuid,
    Uuid,
    r#"CREATE TABLE IF NOT EXISTS review_dispute_history (
        id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
        review_id UUID NOT NULL,
        timestamp TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
        action VARCHAR(50) NOT NULL,
        by_user_id UUID,
        by_user_code VARCHAR(50),
        notes TEXT
    )"#,
    "review_id, timestamp, action, by_user_id, by_user_code, notes",
    "$1, $2, $3, $4, $5, $6",
    "review_id=$1, timestamp=$2, action=$3, by_user_id=$4, by_user_code=$5, notes=$6 WHERE id=$7",
    [
        review_id,
        timestamp,
        action,
        by_user_id,
        by_user_code,
        notes
    ]
);

// PerformanceBadge Table
impl_table!(
    PerformanceBadgeTable,
    PerformanceBadge,
    "performance_badges",
    id,
    Uuid,
    Uuid,
    r#"CREATE TABLE IF NOT EXISTS performance_badges (
        id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
        kitchen_id UUID NOT NULL,
        type VARCHAR(20) NOT NULL,
        title VARCHAR(255) NOT NULL,
        description TEXT NOT NULL,
        earned_date DATE NOT NULL,
        created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
    )"#,
    "kitchen_id, type, title, description, earned_date",
    "$1, $2, $3, $4, $5",
    "kitchen_id=$1, type=$2, title=$3, description=$4, earned_date=$5 WHERE id=$6",
    [kitchen_id, r#type, title, description, earned_date]
);

// AuditLog Table
impl_table!(
    AuditLogTable,
    AuditLog,
    "audit_logs",
    id,
    Uuid,
    Uuid,
    r#"CREATE TABLE IF NOT EXISTS audit_logs (
        id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
        user_id UUID,
        user_name VARCHAR(255),
        action VARCHAR(100) NOT NULL,
        entity_type VARCHAR(100) NOT NULL,
        entity_id VARCHAR(100) NOT NULL,
        timestamp TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
        ip_address VARCHAR(45),
        metadata JSONB
    )"#,
    "user_id, user_name, action, entity_type, entity_id, timestamp, ip_address, metadata",
    "$1, $2, $3, $4, $5, $6, $7, $8",
    "user_id=$1, user_name=$2, action=$3, entity_type=$4, entity_id=$5, timestamp=$6, ip_address=$7, metadata=$8 WHERE id=$9",
    [
        user_id,
        user_name,
        action,
        entity_type,
        entity_id,
        timestamp,
        ip_address,
        metadata
    ]
);

// Alert Table
impl_table!(
    AlertTable,
    Alert,
    "alerts",
    id,
    Uuid,
    Uuid,
    r#"CREATE TABLE IF NOT EXISTS alerts (
        id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
        kitchen_id UUID,
        type alert_type_enum NOT NULL,
        severity alert_severity_enum NOT NULL,
        title VARCHAR(255) NOT NULL,
        message TEXT NOT NULL,
        timestamp TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
        acknowledged BOOLEAN DEFAULT FALSE,
        created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
    )"#,
    "kitchen_id, type, severity, title, message, timestamp, acknowledged",
    "$1, $2, $3, $4, $5, $6, $7",
    "kitchen_id=$1, type=$2, severity=$3, title=$4, message=$5, timestamp=$6, acknowledged=$7 WHERE id=$8",
    [
        kitchen_id,
        r#type,
        severity,
        title,
        message,
        timestamp,
        acknowledged
    ]
);

// Notification Table
impl_table!(
    NotificationTable,
    Notification,
    "notifications",
    id,
    Uuid,
    Uuid,
    r#"CREATE TABLE IF NOT EXISTS notifications (
        id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
        title VARCHAR(255) NOT NULL,
        description TEXT NOT NULL,
        category VARCHAR(50) NOT NULL,
        priority VARCHAR(20) NOT NULL,
        kitchen_code VARCHAR(50),
        school_code VARCHAR(50),
        review_id UUID,
        status VARCHAR(20) DEFAULT 'new',
        target_role VARCHAR(20) NOT NULL,
        created_by VARCHAR(50) NOT NULL,
        created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
        updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
    )"#,
    "title, description, category, priority, kitchen_code, school_code, review_id, status, target_role, created_by",
    "$1, $2, $3, $4, $5, $6, $7, $8, $9, $10",
    "title=$1, description=$2, category=$3, priority=$4, kitchen_code=$5, school_code=$6, review_id=$7, status=$8, target_role=$9, created_by=$10 WHERE id=$11",
    [
        title,
        description,
        category,
        priority,
        kitchen_code,
        school_code,
        review_id,
        status,
        target_role,
        created_by
    ]
);

// NotificationAuditTrail Table
impl_table!(
    NotificationAuditTrailTable,
    NotificationAuditTrail,
    "notification_audit_trail",
    id,
    Uuid,
    Uuid,
    r#"CREATE TABLE IF NOT EXISTS notification_audit_trail (
        id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
        notification_id UUID NOT NULL,
        timestamp TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
        action VARCHAR(50) NOT NULL,
        user_code VARCHAR(50) NOT NULL
    )"#,
    "notification_id, timestamp, action, user_code",
    "$1, $2, $3, $4",
    "notification_id=$1, timestamp=$2, action=$3, user_code=$4 WHERE id=$5",
    [notification_id, timestamp, action, user_code]
);

// Video Table
impl_table!(
    VideoTable,
    Video,
    "videos",
    id,
    Uuid,
    Uuid,
    r#"CREATE TABLE IF NOT EXISTS videos (
        id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
        youtube_id VARCHAR(50) NOT NULL,
        title VARCHAR(255) NOT NULL,
        description TEXT,
        category VARCHAR(100),
        duration VARCHAR(20),
        upload_date DATE,
        thumbnail VARCHAR(255),
        haccp_relevance VARCHAR(255),
        created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
    )"#,
    "youtube_id, title, description, category, duration, upload_date, thumbnail, haccp_relevance",
    "$1, $2, $3, $4, $5, $6, $7, $8",
    "youtube_id=$1, title=$2, description=$3, category=$4, duration=$5, upload_date=$6, thumbnail=$7, haccp_relevance=$8 WHERE id=$9",
    [
        youtube_id,
        title,
        description,
        category,
        duration,
        upload_date,
        thumbnail,
        haccp_relevance
    ]
);
