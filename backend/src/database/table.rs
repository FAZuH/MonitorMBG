//! Generic and specific table implementations.
//!
//! This module defines the [`Table`] trait and provides a macro for implementing
//! standard CRUD operations for database tables.

use async_trait::async_trait;
use rust_decimal::prelude::FromPrimitive;
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

/// Base trait for all database tables.
#[async_trait]
pub trait TableBase {
    /// Creates the table in the database.
    async fn create_table(&self) -> Result<(), DatabaseError>;
    /// Drops the table from the database.
    async fn drop_table(&self) -> Result<(), DatabaseError>;
    /// Deletes all records from the table.
    async fn delete_all(&self) -> Result<(), DatabaseError>;
}

/// Generic trait for CRUD operations on a database table.
#[async_trait]
pub trait Table<T, ID>: TableBase {
    /// Selects all records from the table.
    async fn select_all(&self) -> Result<Vec<T>, DatabaseError>;
    /// Inserts a new record into the table.
    async fn insert(&self, model: &T) -> Result<ID, DatabaseError>;
    /// Selects a record by its ID.
    async fn select(&self, id: &ID) -> Result<Option<T>, DatabaseError>;
    /// Updates an existing record in the table.
    async fn update(&self, model: &T) -> Result<(), DatabaseError>;
    /// Deletes a record by its ID.
    async fn delete(&self, id: &ID) -> Result<(), DatabaseError>;
    /// Replaces a record (not supported in Postgres).
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

/// Kitchen statistics computed from reviews
#[derive(Debug, Clone, sqlx::FromRow)]
pub struct KitchenStats {
    pub kitchen_id: Uuid,
    pub total_reviews: i64,
    pub verified_reviews: i64,
    pub average_rating: Option<rust_decimal::Decimal>,
    pub taste_avg: Option<rust_decimal::Decimal>,
    pub hygiene_avg: Option<rust_decimal::Decimal>,
    pub freshness_avg: Option<rust_decimal::Decimal>,
    pub temperature_avg: Option<rust_decimal::Decimal>,
    pub packaging_avg: Option<rust_decimal::Decimal>,
    pub handling_avg: Option<rust_decimal::Decimal>,
}

/// Review distribution by rating
#[derive(Debug, Clone, sqlx::FromRow)]
pub struct ReviewDistribution {
    pub rating_bucket: i32,
    pub count: i64,
}

/// Compliance trend data for a kitchen
#[derive(Debug, Clone, sqlx::FromRow)]
pub struct ComplianceTrendData {
    pub month: String,
    pub average_score: Option<rust_decimal::Decimal>,
    pub incidents: i64,
    pub reviews: i64,
    pub average_rating: Option<rust_decimal::Decimal>,
}

/// Kitchen with computed statistics for listing
#[derive(Debug, Clone, sqlx::FromRow)]
pub struct KitchenWithStats {
    pub id: Uuid,
    pub name: String,
    pub address: Option<String>,
    pub city: Option<String>,
    pub province: Option<String>,
    pub r#type: Option<KitchenType>,
    pub meals_served: Option<i32>,
    pub certifications: Option<serde_json::Value>,
    pub image_url: Option<String>,
    pub created_at: Option<chrono::NaiveDateTime>,
    pub updated_at: Option<chrono::NaiveDateTime>,
    pub total_reviews: i64,
    pub average_rating: Option<rust_decimal::Decimal>,
}

impl KitchenTable {
    /// Get statistics for a specific kitchen computed from reviews
    pub async fn get_kitchen_stats(
        &self,
        kitchen_id: &Uuid,
    ) -> Result<Option<KitchenStats>, DatabaseError> {
        let stats = sqlx::query_as::<_, KitchenStats>(
            r#"
            SELECT 
                kitchen_id,
                COUNT(*) as total_reviews,
                COUNT(*) FILTER (WHERE verified = true) as verified_reviews,
                AVG((taste_rating + hygiene_rating + freshness_rating + temperature_rating + packaging_rating + handling_rating) / 6) as average_rating,
                AVG(taste_rating) as taste_avg,
                AVG(hygiene_rating) as hygiene_avg,
                AVG(freshness_rating) as freshness_avg,
                AVG(temperature_rating) as temperature_avg,
                AVG(packaging_rating) as packaging_avg,
                AVG(handling_rating) as handling_avg
            FROM reviews 
            WHERE kitchen_id = $1
            GROUP BY kitchen_id
            "#,
        )
        .bind(kitchen_id)
        .fetch_optional(&self.base.pool)
        .await?;

        Ok(stats)
    }

    /// Get review distribution (count of reviews per rating bucket)
    pub async fn get_review_distribution(
        &self,
        kitchen_id: &Uuid,
    ) -> Result<Vec<ReviewDistribution>, DatabaseError> {
        let distribution = sqlx::query_as::<_, ReviewDistribution>(
            r#"
            SELECT 
                FLOOR((taste_rating + hygiene_rating + freshness_rating + temperature_rating + packaging_rating + handling_rating) / 6) as rating_bucket,
                COUNT(*) as count
            FROM reviews 
            WHERE kitchen_id = $1
            GROUP BY rating_bucket
            ORDER BY rating_bucket DESC
            "#,
        )
        .bind(kitchen_id)
        .fetch_all(&self.base.pool)
        .await?;

        Ok(distribution)
    }

    /// Get compliance trend for a kitchen over the last N months
    pub async fn get_compliance_trend(
        &self,
        kitchen_id: &Uuid,
        months: i32,
    ) -> Result<Vec<ComplianceTrendData>, DatabaseError> {
        let trend = sqlx::query_as::<_, ComplianceTrendData>(
            r#"
            SELECT 
                TO_CHAR(DATE_TRUNC('month', created_at), 'YYYY-MM') as month,
                AVG((taste_rating + hygiene_rating + freshness_rating + temperature_rating + packaging_rating + handling_rating) / 6) as average_score,
                0 as incidents,
                COUNT(*) as reviews,
                AVG((taste_rating + hygiene_rating + freshness_rating + temperature_rating + packaging_rating + handling_rating) / 6) as average_rating
            FROM reviews 
            WHERE kitchen_id = $1 
                AND created_at >= DATE_TRUNC('month', CURRENT_DATE - INTERVAL '1 month' * $2)
            GROUP BY DATE_TRUNC('month', created_at)
            ORDER BY month DESC
            "#,
        )
        .bind(kitchen_id)
        .bind(months)
        .fetch_all(&self.base.pool)
        .await?;

        Ok(trend)
    }

    /// List kitchens with computed statistics and filtering
    pub async fn list_kitchens_with_stats(
        &self,
        query: Option<&str>,
        location: Option<&str>,
        kitchen_type: Option<&str>,
        min_rating: Option<f64>,
        limit: i64,
        offset: i64,
    ) -> Result<(Vec<KitchenWithStats>, i64), DatabaseError> {
        // Build the query dynamically
        let mut conditions = vec!["1=1"];

        if query.is_some() {
            conditions.push("k.name ILIKE $1");
        }
        if location.is_some() {
            conditions.push("(k.city ILIKE $2 OR k.province ILIKE $2)");
        }
        if kitchen_type.is_some() {
            conditions.push("k.type::text = $3");
        }
        if min_rating.is_some() {
            conditions.push("COALESCE(stats.average_rating, 0) >= $4");
        }

        let where_clause = conditions.join(" AND ");

        // Count query
        let count_sql = format!(
            r#"
            SELECT COUNT(*) 
            FROM kitchens k
            LEFT JOIN (
                SELECT 
                    kitchen_id,
                    AVG((taste_rating + hygiene_rating + freshness_rating + temperature_rating + packaging_rating + handling_rating) / 6) as average_rating
                FROM reviews
                GROUP BY kitchen_id
            ) stats ON k.id = stats.kitchen_id
            WHERE {}
            "#,
            where_clause
        );

        let mut count_query = sqlx::query_scalar::<_, i64>(&count_sql);

        if let Some(q) = query {
            count_query = count_query.bind(format!("%{}%", q));
        }
        if let Some(loc) = location {
            count_query = count_query.bind(format!("%{}%", loc));
        }
        if let Some(t) = kitchen_type {
            count_query = count_query.bind(t);
        }
        if let Some(r) = min_rating {
            count_query = count_query.bind(rust_decimal::Decimal::from_f64(r).unwrap_or_default());
        }

        let total = count_query.fetch_one(&self.base.pool).await?;

        // Data query
        let data_sql = format!(
            r#"
            SELECT 
                k.id,
                k.name,
                k.address,
                k.city,
                k.province,
                k.type,
                k.meals_served,
                k.certifications,
                k.image_url,
                k.created_at,
                k.updated_at,
                COALESCE(stats.total_reviews, 0) as total_reviews,
                stats.average_rating
            FROM kitchens k
            LEFT JOIN (
                SELECT 
                    kitchen_id,
                    COUNT(*) as total_reviews,
                    AVG((taste_rating + hygiene_rating + freshness_rating + temperature_rating + packaging_rating + handling_rating) / 6) as average_rating
                FROM reviews
                GROUP BY kitchen_id
            ) stats ON k.id = stats.kitchen_id
            WHERE {}
            ORDER BY stats.average_rating DESC NULLS LAST
            LIMIT $5 OFFSET $6
            "#,
            where_clause
        );

        let mut data_query = sqlx::query_as::<_, KitchenWithStats>(&data_sql);

        if let Some(q) = query {
            data_query = data_query.bind(format!("%{}%", q));
        }
        if let Some(loc) = location {
            data_query = data_query.bind(format!("%{}%", loc));
        }
        if let Some(t) = kitchen_type {
            data_query = data_query.bind(t);
        }
        if let Some(r) = min_rating {
            data_query = data_query.bind(rust_decimal::Decimal::from_f64(r).unwrap_or_default());
        }

        let kitchens = data_query
            .bind(limit)
            .bind(offset)
            .fetch_all(&self.base.pool)
            .await?;

        Ok((kitchens, total))
    }

    /// Get kitchen name by ID
    pub async fn get_kitchen_name(
        &self,
        kitchen_id: &Uuid,
    ) -> Result<Option<String>, DatabaseError> {
        let name = sqlx::query_scalar::<_, String>(r#"SELECT name FROM kitchens WHERE id = $1"#)
            .bind(kitchen_id)
            .fetch_optional(&self.base.pool)
            .await?;

        Ok(name)
    }
}

/// Incident timeline event
#[derive(Debug, Clone, sqlx::FromRow)]
pub struct IncidentTimelineEvent {
    pub id: Uuid,
    pub incident_id: Uuid,
    pub event_date: chrono::NaiveDateTime,
    pub event_title: String,
    pub description: Option<String>,
    pub created_at: Option<chrono::NaiveDateTime>,
}

/// Laboratory results for an incident
#[derive(Debug, Clone, sqlx::FromRow)]
pub struct IncidentLabResult {
    pub id: Uuid,
    pub incident_id: Uuid,
    pub pathogen: String,
    pub test_date: chrono::NaiveDate,
    pub confirmed_by: String,
    pub created_at: Option<chrono::NaiveDateTime>,
}

/// Affected institution in an incident
#[derive(Debug, Clone, sqlx::FromRow)]
pub struct AffectedInstitution {
    pub id: Uuid,
    pub incident_id: Uuid,
    pub institution_name: String,
    pub institution_type: Option<String>,
    pub affected_count: Option<i32>,
    pub created_at: Option<chrono::NaiveDateTime>,
}

/// Corrective action for an incident
#[derive(Debug, Clone, sqlx::FromRow)]
pub struct CorrectiveAction {
    pub id: Uuid,
    pub incident_id: Uuid,
    pub action_description: String,
    pub completed: Option<bool>,
    pub completed_date: Option<chrono::NaiveDateTime>,
    pub created_at: Option<chrono::NaiveDateTime>,
}

/// Incident with extended details
#[derive(Debug, Clone, sqlx::FromRow)]
pub struct IncidentWithDetails {
    pub id: Uuid,
    pub kitchen_id: Uuid,
    pub kitchen_name: Option<String>,
    pub r#type: IncidentType,
    pub source: IncidentSource,
    pub date: chrono::NaiveDateTime,
    pub location: Option<String>,
    pub address: Option<String>,
    pub province: Option<String>,
    pub kabupaten: Option<String>,
    pub food_type: Option<String>,
    pub affected_count: Option<i32>,
    pub deaths: Option<i32>,
    pub hospitalized: Option<i32>,
    pub cause: Option<String>,
    pub severity: IncidentSeverity,
    pub status: Option<IncidentStatus>,
    pub description: Option<String>,
    pub reported_by: Option<String>,
    pub source_url: Option<String>,
    pub resolved_at: Option<chrono::NaiveDateTime>,
    pub map_coordinates: Option<serde_json::Value>,
    pub created_at: Option<chrono::NaiveDateTime>,
    pub updated_at: Option<chrono::NaiveDateTime>,
}

impl IncidentTable {
    /// Get incident with extended details including kitchen name
    pub async fn get_incident_with_details(
        &self,
        incident_id: &Uuid,
    ) -> Result<Option<IncidentWithDetails>, DatabaseError> {
        let incident = sqlx::query_as::<_, IncidentWithDetails>(
            r#"
            SELECT 
                i.id,
                i.kitchen_id,
                k.name as kitchen_name,
                i.type,
                i.source,
                i.date,
                i.location,
                k.address,
                i.province,
                k.city as kabupaten,
                i.food_type,
                i.affected_count,
                i.deaths,
                0 as hospitalized,
                i.cause,
                i.severity,
                i.status,
                i.description,
                i.reported_by,
                NULL as source_url,
                CASE WHEN i.status = 'resolved' THEN i.updated_at ELSE NULL END as resolved_at,
                i.map_coordinates,
                i.created_at,
                i.updated_at
            FROM incidents i
            LEFT JOIN kitchens k ON i.kitchen_id = k.id
            WHERE i.id = $1
            "#,
        )
        .bind(incident_id)
        .fetch_optional(&self.base.pool)
        .await?;

        Ok(incident)
    }

    /// Get timeline events for an incident
    pub async fn get_timeline_events(
        &self,
        incident_id: &Uuid,
    ) -> Result<Vec<IncidentTimelineEvent>, DatabaseError> {
        // For now, generate timeline from incident data
        // In a real implementation, this would query a separate timeline table
        let events = sqlx::query_as::<_, IncidentTimelineEvent>(
            r#"
            SELECT 
                uuid_generate_v4() as id,
                id as incident_id,
                date as event_date,
                'Incident reported' as event_title,
                description,
                created_at
            FROM incidents
            WHERE id = $1
            UNION ALL
            SELECT 
                uuid_generate_v4() as id,
                id as incident_id,
                updated_at as event_date,
                CASE 
                    WHEN status = 'resolved' THEN 'Incident resolved'
                    ELSE 'Status updated'
                END as event_title,
                NULL as description,
                updated_at as created_at
            FROM incidents
            WHERE id = $1 AND updated_at != date
            ORDER BY event_date ASC
            "#,
        )
        .bind(incident_id)
        .fetch_all(&self.base.pool)
        .await?;

        Ok(events)
    }

    /// Get lab results for an incident
    pub async fn get_lab_results(
        &self,
        incident_id: &Uuid,
    ) -> Result<Option<IncidentLabResult>, DatabaseError> {
        // Mock lab result based on incident cause
        let result = sqlx::query_as::<_, IncidentLabResult>(
            r#"
            SELECT 
                uuid_generate_v4() as id,
                id as incident_id,
                COALESCE(cause, 'Unknown pathogen') as pathogen,
                CURRENT_DATE - INTERVAL '3 days' as test_date,
                'Balai Laboratorium Kesehatan' as confirmed_by,
                created_at
            FROM incidents
            WHERE id = $1 AND cause IS NOT NULL
            "#,
        )
        .bind(incident_id)
        .fetch_optional(&self.base.pool)
        .await?;

        Ok(result)
    }

    /// Get affected institutions for an incident
    pub async fn get_affected_institutions(
        &self,
        _incident_id: &Uuid,
    ) -> Result<Vec<String>, DatabaseError> {
        // For now, return empty list - would query a separate table in production
        let institutions: Vec<String> = vec![];
        Ok(institutions)
    }

    /// Get corrective actions for an incident
    pub async fn get_corrective_actions(
        &self,
        incident_id: &Uuid,
    ) -> Result<Vec<String>, DatabaseError> {
        // Generate corrective actions based on incident type and severity
        let actions = sqlx::query_scalar::<_, String>(
            r#"
            SELECT 
                CASE 
                    WHEN severity = 'critical' THEN 'Kitchen suspended operations pending investigation'
                    WHEN severity = 'major' THEN 'Enhanced monitoring implemented'
                    ELSE 'Standard corrective measures applied'
                END as action
            FROM incidents
            WHERE id = $1
            UNION ALL
            SELECT 'Staff retraining on food safety protocols' as action
            FROM incidents
            WHERE id = $1 AND severity IN ('major', 'critical')
            UNION ALL
            SELECT 'Equipment sanitization and maintenance' as action
            FROM incidents
            WHERE id = $1
            "#,
        )
        .bind(incident_id)
        .fetch_all(&self.base.pool)
        .await?;

        Ok(actions)
    }

    /// List incidents with filtering
    pub async fn list_incidents(
        &self,
        status: Option<&str>,
        province: Option<&str>,
        min_victims: Option<i32>,
        limit: i64,
        offset: i64,
    ) -> Result<(Vec<Incident>, i64), DatabaseError> {
        let mut conditions = vec!["1=1"];

        if status.is_some() {
            conditions.push("status::text = $1");
        }
        if province.is_some() {
            conditions.push("province ILIKE $2");
        }
        if min_victims.is_some() {
            conditions.push("COALESCE(affected_count, 0) >= $3");
        }

        let where_clause = conditions.join(" AND ");

        // Count query
        let count_sql = format!(r#"SELECT COUNT(*) FROM incidents WHERE {}"#, where_clause);

        let mut count_query = sqlx::query_scalar::<_, i64>(&count_sql);

        if let Some(s) = status {
            count_query = count_query.bind(s);
        }
        if let Some(p) = province {
            count_query = count_query.bind(format!("%{}%", p));
        }
        if let Some(mv) = min_victims {
            count_query = count_query.bind(mv);
        }

        let total = count_query.fetch_one(&self.base.pool).await?;

        // Data query
        let data_sql = format!(
            r#"
            SELECT * FROM incidents
            WHERE {}
            ORDER BY date DESC
            LIMIT $4 OFFSET $5
            "#,
            where_clause
        );

        let mut data_query = sqlx::query_as::<_, Incident>(&data_sql);

        if let Some(s) = status {
            data_query = data_query.bind(s);
        }
        if let Some(p) = province {
            data_query = data_query.bind(format!("%{}%", p));
        }
        if let Some(mv) = min_victims {
            data_query = data_query.bind(mv);
        }

        let incidents = data_query
            .bind(limit)
            .bind(offset)
            .fetch_all(&self.base.pool)
            .await?;

        Ok((incidents, total))
    }
}

/// National statistics
#[derive(Debug, Clone, sqlx::FromRow)]
pub struct NationalStats {
    pub total_kitchens: i64,
    pub active_kitchens: i64,
    pub certified_kitchens: i64,
    pub total_reviews: i64,
    pub verified_reviews: i64,
    pub average_rating: Option<rust_decimal::Decimal>,
    pub total_incidents: i64,
    pub active_incidents: i64,
    pub resolved_incidents: i64,
    pub critical_incidents: i64,
    pub total_victims: i64,
    pub total_deaths: i64,
}

/// Province statistics
#[derive(Debug, Clone, sqlx::FromRow)]
pub struct ProvinceStat {
    pub province: String,
    pub total_kitchens: i64,
    pub avg_rating: Option<rust_decimal::Decimal>,
    pub incidents: i64,
}

/// Regional statistics
#[derive(Debug, Clone, sqlx::FromRow)]
pub struct RegionalStats {
    pub total_kitchens: i64,
    pub active_kitchens: i64,
    pub certified_kitchens: i64,
    pub total_reviews: i64,
    pub total_incidents: i64,
    pub resolved_incidents: i64,
    pub active_incidents: i64,
    pub average_rating: Option<rust_decimal::Decimal>,
}

/// Top performing kitchen
#[derive(Debug, Clone, sqlx::FromRow)]
pub struct TopKitchen {
    pub id: Uuid,
    pub name: String,
    pub rating: Option<rust_decimal::Decimal>,
    pub compliance_score: Option<rust_decimal::Decimal>,
}

/// Monthly incident trend
#[derive(Debug, Clone, sqlx::FromRow)]
pub struct IncidentTrend {
    pub month: String,
    pub total_incidents: i64,
    pub total_victims: i64,
    pub deaths: i64,
    pub top_cause: Option<String>,
}

/// Statistics query methods
pub struct StatsQueries {
    pool: PgPool,
}

impl StatsQueries {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    /// Get national statistics
    pub async fn get_national_stats(&self) -> Result<NationalStats, DatabaseError> {
        let stats = sqlx::query_as::<_, NationalStats>(
            r#"
            SELECT 
                (SELECT COUNT(*) FROM kitchens) as total_kitchens,
                (SELECT COUNT(*) FROM kitchens) as active_kitchens,
                (SELECT COUNT(*) FROM kitchens WHERE certifications IS NOT NULL) as certified_kitchens,
                (SELECT COUNT(*) FROM reviews) as total_reviews,
                (SELECT COUNT(*) FROM reviews WHERE verified = true) as verified_reviews,
                (SELECT AVG((taste_rating + hygiene_rating + freshness_rating + temperature_rating + packaging_rating + handling_rating) / 6) FROM reviews) as average_rating,
                (SELECT COUNT(*) FROM incidents) as total_incidents,
                (SELECT COUNT(*) FROM incidents WHERE status != 'resolved') as active_incidents,
                (SELECT COUNT(*) FROM incidents WHERE status = 'resolved') as resolved_incidents,
                (SELECT COUNT(*) FROM incidents WHERE severity = 'critical') as critical_incidents,
                (SELECT COALESCE(SUM(affected_count), 0) FROM incidents) as total_victims,
                (SELECT COALESCE(SUM(deaths), 0) FROM incidents) as total_deaths
            "#,
        )
        .fetch_one(&self.pool)
        .await?;

        Ok(stats)
    }

    /// Get province statistics
    pub async fn get_province_stats(&self) -> Result<Vec<ProvinceStat>, DatabaseError> {
        let stats = sqlx::query_as::<_, ProvinceStat>(
            r#"
            SELECT 
                COALESCE(k.province, 'Unknown') as province,
                COUNT(DISTINCT k.id) as total_kitchens,
                AVG((r.taste_rating + r.hygiene_rating + r.freshness_rating + r.temperature_rating + r.packaging_rating + r.handling_rating) / 6) as avg_rating,
                COUNT(DISTINCT i.id) as incidents
            FROM kitchens k
            LEFT JOIN reviews r ON k.id = r.kitchen_id
            LEFT JOIN incidents i ON k.id = i.kitchen_id
            GROUP BY k.province
            ORDER BY total_kitchens DESC
            "#,
        )
        .fetch_all(&self.pool)
        .await?;

        Ok(stats)
    }

    /// Get regional statistics
    pub async fn get_regional_stats(
        &self,
        province: Option<&str>,
        kabupaten: Option<&str>,
    ) -> Result<RegionalStats, DatabaseError> {
        let mut conditions = vec!["1=1"];

        if province.is_some() {
            conditions.push("province = $1");
        }
        if kabupaten.is_some() {
            conditions.push("city = $2");
        }

        let where_clause = conditions.join(" AND ");

        let sql = format!(
            r#"
            SELECT 
                COUNT(DISTINCT k.id) as total_kitchens,
                COUNT(DISTINCT k.id) as active_kitchens,
                COUNT(DISTINCT CASE WHEN k.certifications IS NOT NULL THEN k.id END) as certified_kitchens,
                COUNT(DISTINCT r.id) as total_reviews,
                COUNT(DISTINCT i.id) as total_incidents,
                COUNT(DISTINCT CASE WHEN i.status = 'resolved' THEN i.id END) as resolved_incidents,
                COUNT(DISTINCT CASE WHEN i.status != 'resolved' THEN i.id END) as active_incidents,
                AVG((r.taste_rating + r.hygiene_rating + r.freshness_rating + r.temperature_rating + r.packaging_rating + r.handling_rating) / 6) as average_rating
            FROM kitchens k
            LEFT JOIN reviews r ON k.id = r.kitchen_id
            LEFT JOIN incidents i ON k.id = i.kitchen_id
            WHERE {}
            "#,
            where_clause
        );

        let mut query = sqlx::query_as::<_, RegionalStats>(&sql);

        if let Some(p) = province {
            query = query.bind(p);
        }
        if let Some(k) = kabupaten {
            query = query.bind(k);
        }

        let stats = query.fetch_one(&self.pool).await?;

        Ok(stats)
    }

    /// Get top performing kitchens
    pub async fn get_top_kitchens(
        &self,
        province: Option<&str>,
        kabupaten: Option<&str>,
        limit: i64,
    ) -> Result<Vec<TopKitchen>, DatabaseError> {
        let mut conditions = vec!["1=1"];

        if province.is_some() {
            conditions.push("k.province = $1");
        }
        if kabupaten.is_some() {
            conditions.push("k.city = $2");
        }

        let where_clause = conditions.join(" AND ");

        let sql = format!(
            r#"
            SELECT 
                k.id,
                k.name,
                AVG((r.taste_rating + r.hygiene_rating + r.freshness_rating + r.temperature_rating + r.packaging_rating + r.handling_rating) / 6) as rating,
                AVG((r.taste_rating + r.hygiene_rating + r.freshness_rating + r.temperature_rating + r.packaging_rating + r.handling_rating) / 6) as compliance_score
            FROM kitchens k
            LEFT JOIN reviews r ON k.id = r.kitchen_id
            WHERE {}
            GROUP BY k.id, k.name
            HAVING COUNT(r.id) > 0
            ORDER BY rating DESC NULLS LAST
            LIMIT $3
            "#,
            where_clause
        );

        let mut query = sqlx::query_as::<_, TopKitchen>(&sql);

        if let Some(p) = province {
            query = query.bind(p);
        }
        if let Some(k) = kabupaten {
            query = query.bind(k);
        }

        let kitchens = query.bind(limit).fetch_all(&self.pool).await?;

        Ok(kitchens)
    }

    /// Get compliance trends
    pub async fn get_compliance_trends(
        &self,
        province: Option<&str>,
        kabupaten: Option<&str>,
        kitchen_id: Option<Uuid>,
        months: i32,
    ) -> Result<Vec<ComplianceTrendData>, DatabaseError> {
        let mut conditions =
            vec!["r.created_at >= DATE_TRUNC('month', CURRENT_DATE - INTERVAL '1 month' * $1)"];

        if province.is_some() {
            conditions.push("k.province = $2");
        }
        if kabupaten.is_some() {
            conditions.push("k.city = $3");
        }
        if kitchen_id.is_some() {
            conditions.push("r.kitchen_id = $4");
        }

        let where_clause = conditions.join(" AND ");

        let sql = format!(
            r#"
            SELECT 
                TO_CHAR(DATE_TRUNC('month', r.created_at), 'YYYY-MM') as month,
                AVG((r.taste_rating + r.hygiene_rating + r.freshness_rating + r.temperature_rating + r.packaging_rating + r.handling_rating) / 6) as average_score,
                COUNT(DISTINCT i.id) as incidents,
                COUNT(*) as reviews,
                AVG((r.taste_rating + r.hygiene_rating + r.freshness_rating + r.temperature_rating + r.packaging_rating + r.handling_rating) / 6) as average_rating
            FROM reviews r
            LEFT JOIN kitchens k ON r.kitchen_id = k.id
            LEFT JOIN incidents i ON k.id = i.kitchen_id 
                AND i.date >= DATE_TRUNC('month', r.created_at)
                AND i.date < DATE_TRUNC('month', r.created_at) + INTERVAL '1 month'
            WHERE {}
            GROUP BY DATE_TRUNC('month', r.created_at)
            ORDER BY month DESC
            "#,
            where_clause
        );

        let mut query = sqlx::query_as::<_, ComplianceTrendData>(&sql);

        query = query.bind(months);

        if let Some(p) = province {
            query = query.bind(p);
        } else {
            query = query.bind("");
        }

        if let Some(k) = kabupaten {
            query = query.bind(k);
        } else {
            query = query.bind("");
        }

        if let Some(id) = kitchen_id {
            query = query.bind(id);
        } else {
            query = query.bind(Uuid::nil());
        }

        let trends = query.fetch_all(&self.pool).await?;

        Ok(trends)
    }

    /// Get incident trends
    pub async fn get_incident_trends(
        &self,
        province: Option<&str>,
        months: i32,
    ) -> Result<Vec<IncidentTrend>, DatabaseError> {
        let mut conditions =
            vec!["date >= DATE_TRUNC('month', CURRENT_DATE - INTERVAL '1 month' * $1)"];

        if province.is_some() {
            conditions.push("province = $2");
        }

        let where_clause = conditions.join(" AND ");

        let sql = format!(
            r#"
            SELECT 
                TO_CHAR(DATE_TRUNC('month', date), 'YYYY-MM') as month,
                COUNT(*) as total_incidents,
                COALESCE(SUM(affected_count), 0) as total_victims,
                COALESCE(SUM(deaths), 0) as deaths,
                MODE() WITHIN GROUP (ORDER BY cause) as top_cause
            FROM incidents
            WHERE {}
            GROUP BY DATE_TRUNC('month', date)
            ORDER BY month DESC
            "#,
            where_clause
        );

        let mut query = sqlx::query_as::<_, IncidentTrend>(&sql);

        query = query.bind(months);

        if let Some(p) = province {
            query = query.bind(p);
        }

        let trends = query.fetch_all(&self.pool).await?;

        Ok(trends)
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
