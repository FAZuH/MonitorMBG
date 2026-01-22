-- Enable UUID extension
CREATE EXTENSION IF NOT EXISTS "uuid-ossp";

-- Create ENUMs
CREATE TYPE user_role_enum AS ENUM ('kitchen', 'supplier', 'school', 'admin');
CREATE TYPE compliance_trend_enum AS ENUM ('improving', 'stable', 'declining');
CREATE TYPE checklist_status_enum AS ENUM ('pass', 'fail', 'warning');
CREATE TYPE incident_type_enum AS ENUM ('poisoning', 'nutrition', 'sanitation', 'other');
CREATE TYPE incident_source_enum AS ENUM ('consumer', 'inspector', 'public');
CREATE TYPE incident_severity_enum AS ENUM ('minor', 'major', 'critical');
CREATE TYPE incident_status_enum AS ENUM ('investigating', 'resolved', 'escalated');
CREATE TYPE inspection_follow_up_status_enum AS ENUM ('pending', 'in-progress', 'completed');
CREATE TYPE finding_category_enum AS ENUM ('major', 'minor', 'observation');
CREATE TYPE complaint_category_enum AS ENUM ('hygiene', 'taste', 'portion', 'temperature', 'packaging', 'other');
CREATE TYPE complaint_status_enum AS ENUM ('pending', 'in-progress', 'resolved');
CREATE TYPE evidence_metadata_status_enum AS ENUM ('verified', 'mismatch', 'unverified');
CREATE TYPE evidence_capture_method_enum AS ENUM ('camera', 'fallback');
CREATE TYPE alert_type_enum AS ENUM ('compliance', 'incident', 'inspection', 'complaint');
CREATE TYPE alert_severity_enum AS ENUM ('low', 'medium', 'high', 'critical');
CREATE TYPE kitchen_type_enum AS ENUM ('Central Kitchen', 'Regional Kitchen', 'Satellite Kitchen');

-- Institutions Table (for schools, suppliers, etc.)
CREATE TABLE institutions (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    name VARCHAR(255) NOT NULL,
    type VARCHAR(50) NOT NULL CHECK (type IN ('school', 'supplier', 'kitchen', 'government')),
    address TEXT,
    city VARCHAR(100),
    province VARCHAR(100),
    phone VARCHAR(20),
    email VARCHAR(255),
    registration_number VARCHAR(100) UNIQUE,
    verified BOOLEAN DEFAULT FALSE,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);

-- Users Table
CREATE TABLE users (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    name VARCHAR(255) NOT NULL,
    role user_role_enum NOT NULL,
    unique_code VARCHAR(50) UNIQUE NOT NULL,
    phone VARCHAR(20),
    verified BOOLEAN DEFAULT FALSE,
    institution_name VARCHAR(255),
    institution_id UUID,
    ktp_photo_hash VARCHAR(255),
    last_login TIMESTAMP,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    CONSTRAINT FK_users_institutions FOREIGN KEY(institution_id) REFERENCES institutions(id) ON DELETE SET NULL
);

-- Kitchens Table
CREATE TABLE kitchens (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    name VARCHAR(255) NOT NULL,
    address TEXT,
    
    -- Structured location (for filtering/search)
    city VARCHAR(100),
    province VARCHAR(100),
    location VARCHAR(255) GENERATED ALWAYS AS (
        CASE 
            WHEN city IS NOT NULL AND province IS NOT NULL 
            THEN city || ', ' || province
            ELSE COALESCE(city, province, '')
        END
    ) STORED,  -- Auto-computed display field
    
    type kitchen_type_enum DEFAULT 'Central Kitchen',
    meals_served INTEGER DEFAULT 0,
    certifications JSONB,  -- Array: ["HACCP Certified", "Halal", "ISO 22000"]
    image_url VARCHAR(500),
    owner_id UUID,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    CONSTRAINT FK_kitchens_users FOREIGN KEY(owner_id) REFERENCES users(id) ON DELETE SET NULL
);

-- Compliance Metrics Table (For inspection-based metrics, NOT for computed trends)
CREATE TABLE compliance_metrics (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    kitchen_id UUID NOT NULL,
    
    -- Manual inspection scores (not derived from reviews)
    hygiene_score DECIMAL(5, 2),
    portion_compliance DECIMAL(5, 2),
    nutrition_compliance DECIMAL(5, 2),
    temperature_control DECIMAL(5, 2),
    sla_performance JSONB,
    
    last_inspection_date TIMESTAMP,
    trend compliance_trend_enum,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    
    CONSTRAINT FK_compliance_metrics_kitchens FOREIGN KEY(kitchen_id) REFERENCES kitchens(id) ON DELETE CASCADE
);

-- Checklist Items Table
CREATE TABLE checklist_items (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    compliance_metric_id UUID NOT NULL,
    category VARCHAR(100),
    item VARCHAR(255),
    status checklist_status_enum,
    notes TEXT,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    CONSTRAINT FK_checklist_items_compliance_metrics FOREIGN KEY(compliance_metric_id) REFERENCES compliance_metrics(id) ON DELETE CASCADE
);

-- Incidents Table
CREATE TABLE incidents (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    kitchen_id UUID NOT NULL,
    type incident_type_enum NOT NULL,
    source incident_source_enum NOT NULL,
    date TIMESTAMP NOT NULL,
    location VARCHAR(255),
    province VARCHAR(100),  -- For frontend map filtering by region
    food_type VARCHAR(100),
    affected_count INTEGER DEFAULT 0,
    deaths INTEGER DEFAULT 0,  -- Frontend FoodPoisoningIncident.deaths
    cause TEXT,  -- Frontend FoodPoisoningIncident.cause
    severity incident_severity_enum NOT NULL,
    status incident_status_enum DEFAULT 'investigating',
    description TEXT,
    reported_by VARCHAR(255),
    
    -- Map coordinates (percentage-based for frontend map: 0-100)
    -- {x: 30.5, y: 66.5} means 30.5% from left, 66.5% from top
    map_coordinates JSONB,  -- {x: number, y: number}
    
    -- GPS coordinates (if available)
    gps_coordinates JSONB,  -- {lat: number, lng: number}
    
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    CONSTRAINT FK_incidents_kitchens FOREIGN KEY(kitchen_id) REFERENCES kitchens(id) ON DELETE CASCADE
);

-- Inspections Table
CREATE TABLE inspections (
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
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    CONSTRAINT FK_inspections_kitchens FOREIGN KEY(kitchen_id) REFERENCES kitchens(id) ON DELETE CASCADE
);

-- Inspection Findings Table
CREATE TABLE inspection_findings (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    inspection_id UUID NOT NULL,
    category finding_category_enum NOT NULL,
    description TEXT NOT NULL,
    evidence VARCHAR(255),
    correction_required BOOLEAN DEFAULT FALSE,
    deadline TIMESTAMP,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    CONSTRAINT FK_inspection_findings_inspections FOREIGN KEY(inspection_id) REFERENCES inspections(id) ON DELETE CASCADE
);

-- Complaints Table
CREATE TABLE complaints (
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
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    CONSTRAINT FK_complaints_kitchens FOREIGN KEY(kitchen_id) REFERENCES kitchens(id) ON DELETE CASCADE,
    CONSTRAINT FK_complaints_users FOREIGN KEY(assigned_to) REFERENCES users(id) ON DELETE SET NULL
);

-- Complaint Evidence Table
CREATE TABLE complaint_evidence (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    complaint_id UUID NOT NULL,
    url VARCHAR(255) NOT NULL,
    timestamp TIMESTAMP NOT NULL,
    metadata_status evidence_metadata_status_enum,
    capture_method evidence_capture_method_enum,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    CONSTRAINT FK_complaint_evidence_complaints FOREIGN KEY(complaint_id) REFERENCES complaints(id) ON DELETE CASCADE
);

-- Complaint Comments Table
CREATE TABLE complaint_comments (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    complaint_id UUID NOT NULL,
    author_id UUID,
    author_name VARCHAR(255),
    role VARCHAR(50),
    message TEXT NOT NULL,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    CONSTRAINT FK_complaint_comments_complaints FOREIGN KEY(complaint_id) REFERENCES complaints(id) ON DELETE CASCADE,
    CONSTRAINT FK_complaint_comments_users FOREIGN KEY(author_id) REFERENCES users(id) ON DELETE SET NULL
);

-- Reviews Table (MATCHES FRONTEND Review interface)
CREATE TABLE reviews (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    kitchen_id UUID NOT NULL,
    reviewer_id UUID NOT NULL,
    reviewer_name VARCHAR(255) NOT NULL,  -- Denormalized for easy display
    reviewer_type user_role_enum NOT NULL,
    
    -- HACCP ratings (0-5 scale, DECIMAL(2,1) for values like 4.5)
    taste_rating DECIMAL(2,1) NOT NULL CHECK (taste_rating BETWEEN 0 AND 5),
    hygiene_rating DECIMAL(2,1) NOT NULL CHECK (hygiene_rating BETWEEN 0 AND 5),
    freshness_rating DECIMAL(2,1) NOT NULL CHECK (freshness_rating BETWEEN 0 AND 5),
    temperature_rating DECIMAL(2,1) NOT NULL CHECK (temperature_rating BETWEEN 0 AND 5),
    packaging_rating DECIMAL(2,1) NOT NULL CHECK (packaging_rating BETWEEN 0 AND 5),
    handling_rating DECIMAL(2,1) NOT NULL CHECK (handling_rating BETWEEN 0 AND 5),
    
    comment TEXT NOT NULL,
    photos JSONB,  -- Array of photo URLs: ["url1", "url2"]
    
    -- Verification fields
    verification_status VARCHAR(20) DEFAULT 'unverified' 
        CHECK (verification_status IN ('unverified', 'in_progress', 'verified')),
    report_source VARCHAR(20) NOT NULL
        CHECK (report_source IN ('public', 'official_inspector', 'health_worker')),
    confidence_level VARCHAR(10) NOT NULL
        CHECK (confidence_level IN ('low', 'medium', 'high')),
    
    -- Root causes: ["temperature_storage", "cross_contamination", etc.]
    root_causes JSONB,
    
    -- Evidence object: {photoTimestamp, menuCode, schoolLocation, consumptionTime, symptoms[]}
    evidence JSONB,
    
    -- Dispute
    dispute_status VARCHAR(20) DEFAULT 'none'
        CHECK (dispute_status IN ('none', 'disputed', 'under_review', 'resolved')),
    
    verified BOOLEAN DEFAULT FALSE,
    is_draft BOOLEAN DEFAULT FALSE,
    
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    
    CONSTRAINT FK_reviews_kitchens FOREIGN KEY(kitchen_id) 
        REFERENCES kitchens(id) ON DELETE CASCADE,
    CONSTRAINT FK_reviews_users FOREIGN KEY(reviewer_id) 
        REFERENCES users(id) ON DELETE CASCADE
);

-- Review Dispute History Table
CREATE TABLE review_dispute_history (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    review_id UUID NOT NULL,
    timestamp TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    action VARCHAR(50) NOT NULL,
    by_user_id UUID,
    by_user_code VARCHAR(50),
    notes TEXT,
    
    CONSTRAINT FK_review_disputes_reviews FOREIGN KEY(review_id) 
        REFERENCES reviews(id) ON DELETE CASCADE
);

-- Performance Badges Table (MATCHES FRONTEND PerformanceBadge interface)
CREATE TABLE performance_badges (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    kitchen_id UUID NOT NULL,
    type VARCHAR(20) NOT NULL CHECK (type IN ('gold', 'silver', 'improvement')),
    title VARCHAR(255) NOT NULL,
    description TEXT NOT NULL,
    earned_date DATE NOT NULL,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    
    CONSTRAINT FK_performance_badges_kitchens FOREIGN KEY(kitchen_id) 
        REFERENCES kitchens(id) ON DELETE CASCADE
);

-- Audit Logs Table
CREATE TABLE audit_logs (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    user_id UUID,
    user_name VARCHAR(255),
    action VARCHAR(100) NOT NULL,
    entity_type VARCHAR(100) NOT NULL,
    entity_id VARCHAR(100) NOT NULL,
    timestamp TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    ip_address VARCHAR(45),
    metadata JSONB,
    CONSTRAINT FK_audit_logs_users FOREIGN KEY(user_id) REFERENCES users(id) ON DELETE SET NULL
);

-- Alerts Table
CREATE TABLE alerts (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    kitchen_id UUID,
    type alert_type_enum NOT NULL,
    severity alert_severity_enum NOT NULL,
    title VARCHAR(255) NOT NULL,
    message TEXT NOT NULL,
    timestamp TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    acknowledged BOOLEAN DEFAULT FALSE,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    CONSTRAINT FK_alerts_kitchens FOREIGN KEY(kitchen_id) REFERENCES kitchens(id) ON DELETE CASCADE
);

-- Notifications Table (Frontend Notification interface)
CREATE TABLE notifications (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    title VARCHAR(255) NOT NULL,
    description TEXT NOT NULL,
    
    -- HACCP category
    category VARCHAR(50) NOT NULL CHECK (category IN (
        'hygiene', 'temperature', 'cross_contamination', 
        'freshness', 'packaging', 'handling'
    )),
    
    priority VARCHAR(20) NOT NULL CHECK (priority IN ('critical', 'medium', 'minor')),
    
    -- References
    kitchen_code VARCHAR(50),  -- Kitchen unique code
    school_code VARCHAR(50),   -- School unique code (if applicable)
    review_id UUID,            -- Link to specific review
    
    -- Status tracking
    status VARCHAR(20) DEFAULT 'new' CHECK (status IN ('new', 'viewed', 'resolved')),
    
    -- Role targeting (who should see this notification)
    target_role VARCHAR(20) NOT NULL CHECK (target_role IN ('all', 'kitchen', 'supplier', 'consumer')),
    
    -- Creator
    created_by VARCHAR(50) NOT NULL,  -- User unique code who created notification
    
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    
    CONSTRAINT FK_notifications_reviews FOREIGN KEY(review_id) 
        REFERENCES reviews(id) ON DELETE CASCADE
);

-- Notification Audit Trail
CREATE TABLE notification_audit_trail (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    notification_id UUID NOT NULL,
    timestamp TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    action VARCHAR(50) NOT NULL,  -- 'Created', 'Viewed', 'Resolved', etc.
    user_code VARCHAR(50) NOT NULL,    -- User unique code
    
    CONSTRAINT FK_notification_audit_notifications 
        FOREIGN KEY(notification_id) REFERENCES notifications(id) ON DELETE CASCADE
);

-- Videos Table
CREATE TABLE videos (
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
);

-- ============================================
-- INDEXES
-- ============================================
CREATE INDEX idx_institutions_type ON institutions(type);
CREATE INDEX idx_institutions_registration ON institutions(registration_number);

CREATE INDEX idx_users_unique_code ON users(unique_code);
CREATE INDEX idx_users_role ON users(role);
CREATE INDEX idx_users_institution_id ON users(institution_id);

CREATE INDEX idx_kitchens_owner_id ON kitchens(owner_id);
CREATE INDEX idx_kitchens_type ON kitchens(type);
CREATE INDEX idx_kitchens_city ON kitchens(city);
CREATE INDEX idx_kitchens_province ON kitchens(province);

CREATE INDEX idx_compliance_kitchen_id ON compliance_metrics(kitchen_id);

CREATE INDEX idx_incidents_kitchen_id ON incidents(kitchen_id);
CREATE INDEX idx_incidents_status ON incidents(status);
CREATE INDEX idx_incidents_date ON incidents(date);
CREATE INDEX idx_incidents_province ON incidents(province);

CREATE INDEX idx_inspections_kitchen_id ON inspections(kitchen_id);
CREATE INDEX idx_inspections_date ON inspections(date);

CREATE INDEX idx_complaints_kitchen_id ON complaints(kitchen_id);
CREATE INDEX idx_complaints_status ON complaints(status);

CREATE INDEX idx_reviews_kitchen_id ON reviews(kitchen_id);
CREATE INDEX idx_reviews_reviewer_id ON reviews(reviewer_id);
CREATE INDEX idx_reviews_verification_status ON reviews(verification_status);
CREATE INDEX idx_reviews_created_at ON reviews(created_at);

CREATE INDEX idx_review_disputes_review_id ON review_dispute_history(review_id);

CREATE INDEX idx_performance_badges_kitchen_id ON performance_badges(kitchen_id);

CREATE INDEX idx_alerts_kitchen_id ON alerts(kitchen_id);
CREATE INDEX idx_alerts_acknowledged ON alerts(acknowledged);

CREATE INDEX idx_notifications_status ON notifications(status);
CREATE INDEX idx_notifications_target_role ON notifications(target_role);
CREATE INDEX idx_notifications_review_id ON notifications(review_id);
CREATE INDEX idx_notifications_kitchen_code ON notifications(kitchen_code);
CREATE INDEX idx_notifications_created_by ON notifications(created_by);
CREATE INDEX idx_notifications_created_at ON notifications(created_at);

CREATE INDEX idx_notification_audit_notification_id ON notification_audit_trail(notification_id);
CREATE INDEX idx_notification_audit_timestamp ON notification_audit_trail(timestamp);

-- ============================================
-- TRIGGERS FOR updated_at
-- ============================================

-- Function to update updated_at timestamp
CREATE OR REPLACE FUNCTION update_updated_at_column()
RETURNS TRIGGER AS $$
BEGIN
    NEW.updated_at = CURRENT_TIMESTAMP;
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

-- Apply trigger to all tables with updated_at
CREATE TRIGGER update_institutions_updated_at BEFORE UPDATE ON institutions
    FOR EACH ROW EXECUTE FUNCTION update_updated_at_column();

CREATE TRIGGER update_users_updated_at BEFORE UPDATE ON users
    FOR EACH ROW EXECUTE FUNCTION update_updated_at_column();

CREATE TRIGGER update_kitchens_updated_at BEFORE UPDATE ON kitchens
    FOR EACH ROW EXECUTE FUNCTION update_updated_at_column();

CREATE TRIGGER update_compliance_metrics_updated_at BEFORE UPDATE ON compliance_metrics
    FOR EACH ROW EXECUTE FUNCTION update_updated_at_column();

CREATE TRIGGER update_incidents_updated_at BEFORE UPDATE ON incidents
    FOR EACH ROW EXECUTE FUNCTION update_updated_at_column();

CREATE TRIGGER update_inspections_updated_at BEFORE UPDATE ON inspections
    FOR EACH ROW EXECUTE FUNCTION update_updated_at_column();

CREATE TRIGGER update_complaints_updated_at BEFORE UPDATE ON complaints
    FOR EACH ROW EXECUTE FUNCTION update_updated_at_column();

CREATE TRIGGER update_reviews_updated_at BEFORE UPDATE ON reviews
    FOR EACH ROW EXECUTE FUNCTION update_updated_at_column();

CREATE TRIGGER update_notifications_updated_at BEFORE UPDATE ON notifications
    FOR EACH ROW EXECUTE FUNCTION update_updated_at_column();

-- ============================================
-- VIEWS AND FUNCTIONS FOR COMPUTED DATA
-- ============================================

-- View: Kitchen compliance trend (computed from reviews and incidents)
CREATE VIEW kitchen_compliance_trend AS
SELECT 
    k.id as kitchen_id,
    TO_CHAR(DATE_TRUNC('month', r.created_at), 'YYYY-MM') as month,
    ROUND(AVG((r.taste_rating + r.hygiene_rating + r.freshness_rating + 
               r.temperature_rating + r.packaging_rating + r.handling_rating) / 6.0)::NUMERIC, 1) as score,
    COUNT(DISTINCT i.id) as incidents
FROM kitchens k
LEFT JOIN reviews r ON r.kitchen_id = k.id AND r.verified = TRUE
LEFT JOIN incidents i ON i.kitchen_id = k.id 
    AND DATE_TRUNC('month', i.date) = DATE_TRUNC('month', r.created_at)
WHERE r.created_at >= CURRENT_DATE - INTERVAL '6 months'
GROUP BY k.id, DATE_TRUNC('month', r.created_at)
ORDER BY month DESC;

-- Function: Get compliance trend for specific kitchen
CREATE OR REPLACE FUNCTION get_kitchen_compliance_trend(
    p_kitchen_id UUID, 
    p_months INTEGER DEFAULT 6
)
RETURNS TABLE(
    month TEXT, 
    score DECIMAL, 
    incidents BIGINT
) AS $$
BEGIN
    RETURN QUERY
    SELECT 
        TO_CHAR(DATE_TRUNC('month', r.created_at), 'YYYY-MM') as period,
        ROUND(AVG((r.taste_rating + r.hygiene_rating + r.freshness_rating + 
                   r.temperature_rating + r.packaging_rating + r.handling_rating) / 6.0)::NUMERIC, 1)::DECIMAL as avg_score,
        COUNT(DISTINCT i.id) as incident_count
    FROM reviews r
    LEFT JOIN incidents i ON i.kitchen_id = r.kitchen_id 
        AND DATE_TRUNC('month', i.date) = DATE_TRUNC('month', r.created_at)
    WHERE r.kitchen_id = p_kitchen_id
        AND r.verified = TRUE
        AND r.created_at >= CURRENT_DATE - INTERVAL '1 month' * p_months
    GROUP BY DATE_TRUNC('month', r.created_at)
    ORDER BY period DESC;
END;
$$ LANGUAGE plpgsql;
