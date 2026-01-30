use backend::database::model::*;
use backend::database::table::Table;
use rust_decimal::Decimal;

mod common;

// --- Test Harness Macro ---
macro_rules! db_test {
    ($name:ident, |$db:ident| $body:block) => {
        #[tokio::test]
        async fn $name() {
            let ($db, db_name) = common::setup_db().await;
            $body
            common::teardown_db($db, db_name).await;
        }
    };
}

mod institution_table_tests {
    use super::*;

    db_test!(insert_and_select, |db| {
        let institution = Institution {
            name: "Test School".to_string(),
            r#type: "school".to_string(),
            address: Some("123 Test St".to_string()),
            city: Some("Test City".to_string()),
            province: Some("Test Province".to_string()),
            phone: Some("1234567890".to_string()),
            email: Some("test@school.com".to_string()),
            registration_number: Some("REG123".to_string()),
            verified: Some(true),
            ..Default::default()
        };

        let id = db
            .institution_table
            .insert(&institution)
            .await
            .expect("Failed to insert institution");

        let fetched = db
            .institution_table
            .select(&id)
            .await
            .expect("Failed to select")
            .unwrap();
        assert_eq!(fetched.name, "Test School");
        assert_eq!(fetched.r#type, "school");
    });

    db_test!(update, |db| {
        let institution = Institution {
            name: "Original Name".to_string(),
            r#type: "school".to_string(),
            registration_number: Some("REG456".to_string()),
            verified: Some(false),
            ..Default::default()
        };

        let id = db
            .institution_table
            .insert(&institution)
            .await
            .expect("Failed to insert");

        let mut to_update = db
            .institution_table
            .select(&id)
            .await
            .expect("Failed to select")
            .unwrap();
        to_update.name = "Updated Name".to_string();

        db.institution_table
            .update(&to_update)
            .await
            .expect("Failed to update");

        let fetched = db
            .institution_table
            .select(&id)
            .await
            .expect("Failed to select")
            .unwrap();
        assert_eq!(fetched.name, "Updated Name");
    });

    db_test!(delete, |db| {
        let institution = Institution {
            name: "To Delete".to_string(),
            r#type: "school".to_string(),
            registration_number: Some("REG789".to_string()),
            verified: Some(false),
            ..Default::default()
        };

        let id = db
            .institution_table
            .insert(&institution)
            .await
            .expect("Failed to insert");

        db.institution_table
            .delete(&id)
            .await
            .expect("Failed to delete");

        let fetched = db
            .institution_table
            .select(&id)
            .await
            .expect("Failed to select");
        assert!(fetched.is_none());
    });
}

mod user_table_tests {
    use super::*;

    db_test!(insert_and_select, |db| {
        let user = User {
            name: "Test User".to_string(),
            role: UserRole::Admin,
            unique_code: "USER123".to_string(),
            phone: Some("08123456789".to_string()),
            verified: Some(true),
            ..Default::default()
        };

        let id = db
            .user_table
            .insert(&user)
            .await
            .expect("Failed to insert user");

        let fetched = db
            .user_table
            .select(&id)
            .await
            .expect("Failed to select")
            .unwrap();
        assert_eq!(fetched.name, "Test User");
        assert_eq!(fetched.role, UserRole::Admin);
    });
}

mod kitchen_table_tests {
    use super::*;

    db_test!(insert_and_select, |db| {
        let kitchen = Kitchen {
            name: "Central Kitchen 1".to_string(),
            address: Some("Kitchen Address".to_string()),
            city: Some("Jakarta".to_string()),
            province: Some("DKI Jakarta".to_string()),
            r#type: Some(KitchenType::CentralKitchen),
            meals_served: Some(1000),
            ..Default::default()
        };

        let id = db
            .kitchen_table
            .insert(&kitchen)
            .await
            .expect("Failed to insert kitchen");

        let fetched = db
            .kitchen_table
            .select(&id)
            .await
            .expect("Failed to select")
            .unwrap();
        assert_eq!(fetched.name, "Central Kitchen 1");
    });
}

mod compliance_metric_tests {
    use super::*;

    db_test!(insert_and_select, |db| {
        let kitchen = Kitchen {
            name: "Kitchen for Metric".to_string(),
            ..Default::default()
        };
        let kitchen_id = db
            .kitchen_table
            .insert(&kitchen)
            .await
            .expect("Failed to insert kitchen");

        let metric = ComplianceMetric {
            kitchen_id,
            hygiene_score: Some(Decimal::from_f64_retain(95.5).unwrap()),
            portion_compliance: Some(Decimal::from_f64_retain(98.0).unwrap()),
            nutrition_compliance: Some(Decimal::from_f64_retain(100.0).unwrap()),
            temperature_control: Some(Decimal::from_f64_retain(92.0).unwrap()),
            trend: Some(ComplianceTrend::Improving),
            ..Default::default()
        };

        let id = db
            .compliance_metric_table
            .insert(&metric)
            .await
            .expect("Failed to insert metric");

        let fetched = db
            .compliance_metric_table
            .select(&id)
            .await
            .expect("Failed to select")
            .unwrap();
        assert_eq!(
            fetched.hygiene_score,
            Some(Decimal::from_f64_retain(95.5).unwrap())
        );
        assert_eq!(fetched.trend, Some(ComplianceTrend::Improving));
    });
}

mod checklist_item_tests {
    use super::*;

    db_test!(insert_and_select, |db| {
        let kitchen = Kitchen {
            name: "Kitchen for Checklist".to_string(),
            ..Default::default()
        };
        let kitchen_id = db
            .kitchen_table
            .insert(&kitchen)
            .await
            .expect("Failed to insert kitchen");

        let metric = ComplianceMetric {
            kitchen_id,
            ..Default::default()
        };
        let metric_id = db
            .compliance_metric_table
            .insert(&metric)
            .await
            .expect("Failed to insert metric");

        let item = ChecklistItem {
            compliance_metric_id: metric_id,
            category: Some("Hygiene".to_string()),
            item: Some("Clean floor".to_string()),
            status: Some(ChecklistStatus::Pass),
            ..Default::default()
        };

        let id = db
            .checklist_item_table
            .insert(&item)
            .await
            .expect("Failed to insert checklist item");

        let fetched = db
            .checklist_item_table
            .select(&id)
            .await
            .expect("Failed to select")
            .unwrap();
        assert_eq!(fetched.item, Some("Clean floor".to_string()));
        assert_eq!(fetched.status, Some(ChecklistStatus::Pass));
    });
}

mod incident_tests {
    use super::*;

    db_test!(insert_and_select, |db| {
        let kitchen = Kitchen {
            name: "Kitchen for Incident".to_string(),
            ..Default::default()
        };
        let kitchen_id = db
            .kitchen_table
            .insert(&kitchen)
            .await
            .expect("Failed to insert kitchen");

        let incident = Incident {
            kitchen_id,
            r#type: IncidentType::Poisoning,
            source: IncidentSource::Consumer,
            severity: IncidentSeverity::Critical,
            description: Some("Food poisoning reported".to_string()),
            ..Default::default()
        };

        let id = db
            .incident_table
            .insert(&incident)
            .await
            .expect("Failed to insert incident");

        let fetched = db
            .incident_table
            .select(&id)
            .await
            .expect("Failed to select")
            .unwrap();
        assert_eq!(fetched.r#type, IncidentType::Poisoning);
        assert_eq!(fetched.severity, IncidentSeverity::Critical);
    });
}

mod inspection_tests {
    use super::*;

    db_test!(insert_and_select, |db| {
        let kitchen = Kitchen {
            name: "Kitchen for Inspection".to_string(),
            ..Default::default()
        };
        let kitchen_id = db
            .kitchen_table
            .insert(&kitchen)
            .await
            .expect("Failed to insert kitchen");

        let inspection = Inspection {
            kitchen_id,
            inspector_name: Some("Inspector Gadget".to_string()),
            overall_score: Some(Decimal::from_f64_retain(85.0).unwrap()),
            ..Default::default()
        };

        let id = db
            .inspection_table
            .insert(&inspection)
            .await
            .expect("Failed to insert inspection");

        let fetched = db
            .inspection_table
            .select(&id)
            .await
            .expect("Failed to select")
            .unwrap();
        assert_eq!(fetched.inspector_name, Some("Inspector Gadget".to_string()));
    });
}

mod inspection_finding_tests {
    use super::*;

    db_test!(insert_and_select, |db| {
        let kitchen = Kitchen {
            name: "Kitchen for Finding".to_string(),
            ..Default::default()
        };
        let kitchen_id = db
            .kitchen_table
            .insert(&kitchen)
            .await
            .expect("Failed to insert kitchen");

        let inspection = Inspection {
            kitchen_id,
            ..Default::default()
        };
        let inspection_id = db
            .inspection_table
            .insert(&inspection)
            .await
            .expect("Failed to insert inspection");

        let finding = InspectionFinding {
            inspection_id,
            category: FindingCategory::Major,
            description: "Dirty surfaces".to_string(),
            ..Default::default()
        };

        let id = db
            .inspection_finding_table
            .insert(&finding)
            .await
            .expect("Failed to insert finding");

        let fetched = db
            .inspection_finding_table
            .select(&id)
            .await
            .expect("Failed to select")
            .unwrap();
        assert_eq!(fetched.description, "Dirty surfaces");
        assert_eq!(fetched.category, FindingCategory::Major);
    });
}

mod complaint_tests {
    use super::*;

    db_test!(insert_and_select, |db| {
        let kitchen = Kitchen {
            name: "Kitchen for Complaint".to_string(),
            ..Default::default()
        };
        let kitchen_id = db
            .kitchen_table
            .insert(&kitchen)
            .await
            .expect("Failed to insert kitchen");

        let complaint = Complaint {
            kitchen_id,
            category: ComplaintCategory::Taste,
            description: "Too salty".to_string(),
            status: Some(ComplaintStatus::Pending),
            ..Default::default()
        };

        let id = db
            .complaint_table
            .insert(&complaint)
            .await
            .expect("Failed to insert complaint");

        let fetched = db
            .complaint_table
            .select(&id)
            .await
            .expect("Failed to select")
            .unwrap();
        assert_eq!(fetched.description, "Too salty");
        assert_eq!(fetched.category, ComplaintCategory::Taste);
    });
}

mod complaint_evidence_tests {
    use super::*;

    db_test!(insert_and_select, |db| {
        let kitchen = Kitchen {
            name: "Kitchen for Evidence".to_string(),
            ..Default::default()
        };
        let kitchen_id = db
            .kitchen_table
            .insert(&kitchen)
            .await
            .expect("Failed to insert kitchen");

        let complaint = Complaint {
            kitchen_id,
            ..Default::default()
        };
        let complaint_id = db
            .complaint_table
            .insert(&complaint)
            .await
            .expect("Failed to insert complaint");

        let evidence = ComplaintEvidence {
            complaint_id,
            url: "http://example.com/photo.jpg".to_string(),
            metadata_status: Some(EvidenceMetadataStatus::Verified),
            ..Default::default()
        };

        let id = db
            .complaint_evidence_table
            .insert(&evidence)
            .await
            .expect("Failed to insert evidence");

        let fetched = db
            .complaint_evidence_table
            .select(&id)
            .await
            .expect("Failed to select")
            .unwrap();
        assert_eq!(fetched.url, "http://example.com/photo.jpg");
        assert_eq!(
            fetched.metadata_status,
            Some(EvidenceMetadataStatus::Verified)
        );
    });
}

mod complaint_comment_tests {
    use super::*;

    db_test!(insert_and_select, |db| {
        let kitchen = Kitchen {
            name: "Kitchen for Comment".to_string(),
            ..Default::default()
        };
        let kitchen_id = db
            .kitchen_table
            .insert(&kitchen)
            .await
            .expect("Failed to insert kitchen");

        let complaint = Complaint {
            kitchen_id,
            ..Default::default()
        };
        let complaint_id = db
            .complaint_table
            .insert(&complaint)
            .await
            .expect("Failed to insert complaint");

        let comment = ComplaintComment {
            complaint_id,
            message: "Investigating this issue".to_string(),
            author_name: Some("Admin".to_string()),
            ..Default::default()
        };

        let id = db
            .complaint_comment_table
            .insert(&comment)
            .await
            .expect("Failed to insert comment");

        let fetched = db
            .complaint_comment_table
            .select(&id)
            .await
            .expect("Failed to select")
            .unwrap();
        assert_eq!(fetched.message, "Investigating this issue");
        assert_eq!(fetched.author_name, Some("Admin".to_string()));
    });
}

mod review_tests {
    use super::*;

    db_test!(insert_and_select, |db| {
        let kitchen = Kitchen {
            name: "Kitchen for Review".to_string(),
            ..Default::default()
        };
        let kitchen_id = db
            .kitchen_table
            .insert(&kitchen)
            .await
            .expect("Failed to insert kitchen");

        let user = User {
            name: "Reviewer User".to_string(),
            unique_code: "REV001".to_string(),
            ..Default::default()
        };
        let reviewer_id = db
            .user_table
            .insert(&user)
            .await
            .expect("Failed to insert user");

        let review = Review {
            kitchen_id,
            reviewer_id,
            reviewer_name: "Student A".to_string(),
            taste_rating: Decimal::from_f64_retain(4.5).unwrap(),
            comment: "Great food!".to_string(),
            ..Default::default()
        };

        let id = db
            .review_table
            .insert(&review)
            .await
            .expect("Failed to insert review");

        let fetched = db
            .review_table
            .select(&id)
            .await
            .expect("Failed to select")
            .unwrap();
        assert_eq!(fetched.reviewer_name, "Student A");
        assert_eq!(fetched.comment, "Great food!");
    });
}

mod review_dispute_history_tests {
    use super::*;

    db_test!(insert_and_select, |db| {
        let kitchen = Kitchen {
            name: "Kitchen for Dispute".to_string(),
            ..Default::default()
        };
        let kitchen_id = db
            .kitchen_table
            .insert(&kitchen)
            .await
            .expect("Failed to insert kitchen");

        let user = User {
            name: "Reviewer User Dispute".to_string(),
            unique_code: "REV002".to_string(),
            ..Default::default()
        };
        let reviewer_id = db
            .user_table
            .insert(&user)
            .await
            .expect("Failed to insert user");

        let review = Review {
            kitchen_id,
            reviewer_id,
            ..Default::default()
        };
        let review_id = db
            .review_table
            .insert(&review)
            .await
            .expect("Failed to insert review");

        let history = ReviewDisputeHistory {
            review_id,
            action: "Dispute Opened".to_string(),
            notes: Some("Unfair rating".to_string()),
            ..Default::default()
        };

        let id = db
            .review_dispute_history_table
            .insert(&history)
            .await
            .expect("Failed to insert history");

        let fetched = db
            .review_dispute_history_table
            .select(&id)
            .await
            .expect("Failed to select")
            .unwrap();
        assert_eq!(fetched.action, "Dispute Opened");
        assert_eq!(fetched.notes, Some("Unfair rating".to_string()));
    });
}

mod performance_badge_tests {
    use super::*;

    db_test!(insert_and_select, |db| {
        let kitchen = Kitchen {
            name: "Kitchen for Badge".to_string(),
            ..Default::default()
        };
        let kitchen_id = db
            .kitchen_table
            .insert(&kitchen)
            .await
            .expect("Failed to insert kitchen");

        let badge = PerformanceBadge {
            kitchen_id,
            title: "Top Chef".to_string(),
            description: "Best food quality".to_string(),
            ..Default::default()
        };

        let id = db
            .performance_badge_table
            .insert(&badge)
            .await
            .expect("Failed to insert badge");

        let fetched = db
            .performance_badge_table
            .select(&id)
            .await
            .expect("Failed to select")
            .unwrap();
        assert_eq!(fetched.title, "Top Chef");
    });
}

mod audit_log_tests {
    use super::*;

    db_test!(insert_and_select, |db| {
        let log = AuditLog {
            action: "LOGIN".to_string(),
            entity_type: "USER".to_string(),
            entity_id: "123".to_string(),
            ..Default::default()
        };

        let id = db
            .audit_log_table
            .insert(&log)
            .await
            .expect("Failed to insert log");

        let fetched = db
            .audit_log_table
            .select(&id)
            .await
            .expect("Failed to select")
            .unwrap();
        assert_eq!(fetched.action, "LOGIN");
    });
}

mod alert_tests {
    use super::*;

    db_test!(insert_and_select, |db| {
        let alert = Alert {
            title: "High Temp".to_string(),
            message: "Freezer temp too high".to_string(),
            severity: AlertSeverity::High,
            ..Default::default()
        };

        let id = db
            .alert_table
            .insert(&alert)
            .await
            .expect("Failed to insert alert");

        let fetched = db
            .alert_table
            .select(&id)
            .await
            .expect("Failed to select")
            .unwrap();
        assert_eq!(fetched.title, "High Temp");
        assert_eq!(fetched.severity, AlertSeverity::High);
    });
}

mod notification_tests {
    use super::*;

    db_test!(insert_and_select, |db| {
        let notification = Notification {
            title: "New Review".to_string(),
            description: "You have a new review".to_string(),
            target_role: "kitchen".to_string(),
            ..Default::default()
        };

        let id = db
            .notification_table
            .insert(&notification)
            .await
            .expect("Failed to insert notification");

        let fetched = db
            .notification_table
            .select(&id)
            .await
            .expect("Failed to select")
            .unwrap();
        assert_eq!(fetched.title, "New Review");
    });
}

mod notification_audit_trail_tests {
    use super::*;

    db_test!(insert_and_select, |db| {
        let notification = Notification {
            title: "Audit Test".to_string(),
            ..Default::default()
        };
        let notification_id = db
            .notification_table
            .insert(&notification)
            .await
            .expect("Failed to insert notification");

        let trail = NotificationAuditTrail {
            notification_id,
            action: "READ".to_string(),
            user_code: "USER1".to_string(),
            ..Default::default()
        };

        let id = db
            .notification_audit_trail_table
            .insert(&trail)
            .await
            .expect("Failed to insert trail");

        let fetched = db
            .notification_audit_trail_table
            .select(&id)
            .await
            .expect("Failed to select")
            .unwrap();
        assert_eq!(fetched.action, "READ");
    });
}

mod video_tests {
    use super::*;

    db_test!(insert_and_select, |db| {
        let video = Video {
            youtube_id: "dQw4w9WgXcQ".to_string(),
            title: "Training Video".to_string(),
            ..Default::default()
        };

        let id = db
            .video_table
            .insert(&video)
            .await
            .expect("Failed to insert video");

        let fetched = db
            .video_table
            .select(&id)
            .await
            .expect("Failed to select")
            .unwrap();
        assert_eq!(fetched.youtube_id, "dQw4w9WgXcQ");
        assert_eq!(fetched.title, "Training Video");
    });
}
