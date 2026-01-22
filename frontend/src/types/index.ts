// User and Authentication Types
export interface User {
  id: string;
  name: string;
  role: 'kitchen' | 'supplier' | 'school';
  uniqueCode: string;
  phone: string;
  verified: boolean;
  institutionName: string;
  institutionId: string;
  ktpPhotoHash?: string;
  createdAt: string;
  lastLogin?: string;
}

export interface RegistrationData {
  name: string;
  phone: string;
  uniqueCode: string;
  ktpPhoto: string;
  consentGiven: boolean;
  institutionName: string;
}

// Compliance Types
export interface ComplianceMetrics {
  kitchenId: string;
  kitchenName: string;
  hygieneScore: number;
  portionCompliance: number;
  nutritionCompliance: number;
  temperatureControl: number;
  cleanlinessChecklist: ChecklistItem[];
  slaPerformance: {
    standardTime: number;
    actualTime: number;
    onTimeRate: number;
  };
  lastInspection: string;
  trend: 'improving' | 'stable' | 'declining';
}

export interface ChecklistItem {
  id: string;
  category: string;
  item: string;
  status: 'pass' | 'fail' | 'warning';
  notes?: string;
}

// Incident Types
export interface Incident {
  id: string;
  type: 'poisoning' | 'nutrition' | 'sanitation' | 'other';
  source: 'consumer' | 'inspector' | 'public';
  date: string;
  location: string;
  kitchenId: string;
  kitchenName: string;
  foodType: string;
  affectedCount: number;
  severity: 'minor' | 'major' | 'critical';
  status: 'investigating' | 'resolved' | 'escalated';
  description: string;
  reportedBy: string;
  coordinates?: { lat: number; lng: number };
}

// Inspection Types
export interface Inspection {
  id: string;
  kitchenId: string;
  inspectorName: string;
  date: string;
  findings: Finding[];
  overallScore: number;
  recommendations: string[];
  followUpStatus: 'pending' | 'in-progress' | 'completed';
  attachments: string[];
  nextInspectionDate?: string;
}

export interface Finding {
  id: string;
  category: 'major' | 'minor' | 'observation';
  description: string;
  evidence?: string;
  correctionRequired: boolean;
  deadline?: string;
}

// Complaint Types
export interface Complaint {
  id: string;
  kitchenId: string;
  category: 'hygiene' | 'taste' | 'portion' | 'temperature' | 'packaging' | 'other';
  description: string;
  evidence: ComplaintEvidence[];
  status: 'pending' | 'in-progress' | 'resolved';
  reportedBy: string;
  reportedAt: string;
  slaDeadline: string;
  assignedTo?: string;
  resolution?: string;
  satisfactionRating?: number;
  comments: ComplaintComment[];
}

export interface ComplaintEvidence {
  url: string;
  timestamp: string;
  metadataStatus: 'verified' | 'mismatch' | 'unverified';
  captureMethod: 'camera' | 'fallback';
}

export interface ComplaintComment {
  id: string;
  author: string;
  role: string;
  message: string;
  timestamp: string;
}

// Photo Metadata Types
export interface PhotoMetadata {
  timestamp: string;
  deviceTime: string;
  serverTime: string;
  location?: { lat: number; lng: number };
  verified: boolean;
  watermark: {
    institutionName: string;
    reportCode: string;
    dateTime: string;
    location?: string;
  };
  hash: string;
}

// Audit Types
export interface AuditLog {
  id: string;
  userId: string;
  userName: string;
  action: string;
  entityType: string;
  entityId: string;
  timestamp: string;
  ipAddress?: string;
  metadata?: Record<string, any>;
}

// Dashboard Types
export interface DashboardStats {
  totalKitchens: number;
  averageCompliance: number;
  activeIncidents: number;
  pendingComplaints: number;
  recentInspections: number;
  criticalAlerts: number;
}

export interface Alert {
  id: string;
  type: 'compliance' | 'incident' | 'inspection' | 'complaint';
  severity: 'low' | 'medium' | 'high' | 'critical';
  title: string;
  message: string;
  kitchenId?: string;
  timestamp: string;
  acknowledged: boolean;
}
