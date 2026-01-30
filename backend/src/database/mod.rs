//! Database management and connection pooling.
//!
//! This module provides the central [`Database`] struct which manages the connection pool
//! and all table-specific data access objects.

use std::str::FromStr;

use log::debug;
use log::info;
use sqlx::migrate::MigrateDatabase;
use sqlx::postgres::PgConnectOptions as ConnOpt;
use sqlx::postgres::PgPool as Pool;
use sqlx::postgres::Postgres;

pub mod error;
pub mod model;
pub mod table;

use table::*;

pub use table::StatsQueries;

/// Central database manager.
///
/// Holds the connection pool and all table-specific data access objects.
pub struct Database {
    /// The underlying SQLx connection pool.
    pool: Pool,
    /// Table for institution data.
    pub institution_table: InstitutionTable,
    /// Table for user data.
    pub user_table: UserTable,
    /// Table for kitchen data.
    pub kitchen_table: KitchenTable,
    /// Table for compliance metrics.
    pub compliance_metric_table: ComplianceMetricTable,
    /// Table for checklist items.
    pub checklist_item_table: ChecklistItemTable,
    /// Table for incident reports.
    pub incident_table: IncidentTable,
    /// Table for inspection records.
    pub inspection_table: InspectionTable,
    /// Table for inspection findings.
    pub inspection_finding_table: InspectionFindingTable,
    /// Table for complaints.
    pub complaint_table: ComplaintTable,
    /// Table for complaint evidence.
    pub complaint_evidence_table: ComplaintEvidenceTable,
    /// Table for complaint comments.
    pub complaint_comment_table: ComplaintCommentTable,
    /// Table for reviews.
    pub review_table: ReviewTable,
    /// Table for review dispute history.
    pub review_dispute_history_table: ReviewDisputeHistoryTable,
    /// Table for performance badges.
    pub performance_badge_table: PerformanceBadgeTable,
    /// Table for audit logs.
    pub audit_log_table: AuditLogTable,
    /// Table for alerts.
    pub alert_table: AlertTable,
    /// Table for notifications.
    pub notification_table: NotificationTable,
    /// Table for notification audit trails.
    pub notification_audit_trail_table: NotificationAuditTrailTable,
    /// Table for video data.
    pub video_table: VideoTable,
    /// Statistics queries
    pub stats_queries: StatsQueries,
}

impl Database {
    /// Close the connection pool
    /// Used primarily for testing
    pub async fn close(&self) {
        self.pool.close().await;
    }
}

impl Database {
    /// Creates a new `Database` instance and connects to the database.
    ///
    /// # Errors
    ///
    /// Returns an error if the connection fails or if the database cannot be created.
    pub async fn new(db_url: &str) -> anyhow::Result<Self> {
        debug!("Connecting to db...");
        Self::create_db(db_url).await?;

        let opts = ConnOpt::from_str(db_url)?;
        let pool = Pool::connect_with(opts).await?;
        info!("Connected to db.");

        let institution_table = InstitutionTable::new(pool.clone());
        let user_table = UserTable::new(pool.clone());
        let kitchen_table = KitchenTable::new(pool.clone());
        let compliance_metric_table = ComplianceMetricTable::new(pool.clone());
        let checklist_item_table = ChecklistItemTable::new(pool.clone());
        let incident_table = IncidentTable::new(pool.clone());
        let inspection_table = InspectionTable::new(pool.clone());
        let inspection_finding_table = InspectionFindingTable::new(pool.clone());
        let complaint_table = ComplaintTable::new(pool.clone());
        let complaint_evidence_table = ComplaintEvidenceTable::new(pool.clone());
        let complaint_comment_table = ComplaintCommentTable::new(pool.clone());
        let review_table = ReviewTable::new(pool.clone());
        let review_dispute_history_table = ReviewDisputeHistoryTable::new(pool.clone());
        let performance_badge_table = PerformanceBadgeTable::new(pool.clone());
        let audit_log_table = AuditLogTable::new(pool.clone());
        let alert_table = AlertTable::new(pool.clone());
        let notification_table = NotificationTable::new(pool.clone());
        let notification_audit_trail_table = NotificationAuditTrailTable::new(pool.clone());
        let video_table = VideoTable::new(pool.clone());
        let stats_queries = StatsQueries::new(pool.clone());

        Ok(Self {
            pool,
            institution_table,
            user_table,
            kitchen_table,
            compliance_metric_table,
            checklist_item_table,
            incident_table,
            inspection_table,
            inspection_finding_table,
            complaint_table,
            complaint_evidence_table,
            complaint_comment_table,
            review_table,
            review_dispute_history_table,
            performance_badge_table,
            audit_log_table,
            alert_table,
            notification_table,
            notification_audit_trail_table,
            video_table,
            stats_queries,
        })
    }

    /// Runs database migrations.
    ///
    /// # Errors
    ///
    /// Returns an error if migrations fail.
    pub async fn run_migrations(&self) -> anyhow::Result<()> {
        sqlx::migrate!("./migrations").run(&self.pool).await?;
        Ok(())
    }

    pub async fn drop_all_tables(&self) -> anyhow::Result<()> {
        // Drop in reverse order of dependencies (roughly)
        self.notification_audit_trail_table.drop_table().await?;
        self.notification_table.drop_table().await?;
        self.alert_table.drop_table().await?;
        self.audit_log_table.drop_table().await?;
        self.performance_badge_table.drop_table().await?;
        self.review_dispute_history_table.drop_table().await?;
        self.review_table.drop_table().await?;
        self.complaint_comment_table.drop_table().await?;
        self.complaint_evidence_table.drop_table().await?;
        self.complaint_table.drop_table().await?;
        self.inspection_finding_table.drop_table().await?;
        self.inspection_table.drop_table().await?;
        self.incident_table.drop_table().await?;
        self.checklist_item_table.drop_table().await?;
        self.compliance_metric_table.drop_table().await?;
        self.kitchen_table.drop_table().await?;
        self.user_table.drop_table().await?;
        self.institution_table.drop_table().await?;
        self.video_table.drop_table().await?;
        Ok(())
    }

    pub async fn delete_all_tables(&self) -> anyhow::Result<()> {
        self.notification_audit_trail_table.delete_all().await?;
        self.notification_table.delete_all().await?;
        self.alert_table.delete_all().await?;
        self.audit_log_table.delete_all().await?;
        self.performance_badge_table.delete_all().await?;
        self.review_dispute_history_table.delete_all().await?;
        self.review_table.delete_all().await?;
        self.complaint_comment_table.delete_all().await?;
        self.complaint_evidence_table.delete_all().await?;
        self.complaint_table.delete_all().await?;
        self.inspection_finding_table.delete_all().await?;
        self.inspection_table.delete_all().await?;
        self.incident_table.delete_all().await?;
        self.checklist_item_table.delete_all().await?;
        self.compliance_metric_table.delete_all().await?;
        self.kitchen_table.delete_all().await?;
        self.user_table.delete_all().await?;
        self.institution_table.delete_all().await?;
        self.video_table.delete_all().await?;
        Ok(())
    }

    /// Create database if not exists.
    pub async fn create_db(db_url: &str) -> anyhow::Result<()> {
        if !Postgres::database_exists(db_url).await? {
            info!("Database does not exist. Creating...");
            Postgres::create_database(db_url).await?;
            info!("Database created successfully.");
        }
        Ok(())
    }
}
