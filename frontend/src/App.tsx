import { useState } from 'react';
import { LoginPage } from './components/auth/LoginPage';
import { RegistrationPage } from './components/auth/RegistrationPage';
import { DashboardHeader } from './components/dashboard/DashboardHeader';
import { HACCPEducation } from './components/dashboard/HACCPEducation';
import { QuickTutorial } from './components/dashboard/QuickTutorial';
import { KitchenList } from './components/KitchenList';
import { KitchenDetail } from './components/KitchenDetail';
import { ReviewForm } from './components/ReviewForm';
import { IncidentMap } from './components/IncidentMap';
import { PublicDashboard } from './components/PublicDashboard';
import { NotificationPanel, Notification } from './components/NotificationPanel';
import { LocationFilter } from './components/LocationFilter';
import { BottomNavigation } from './components/BottomNavigation';
import { MobileMapModal } from './components/MobileMapModal';
import { ChefHat, Award, Users, MapIcon, BarChart3, AlertTriangle, ClipboardList, MessageSquare } from 'lucide-react';
import { User } from './types';
import mealPhoto1 from 'figma:asset/f6c1a2b02ee187084390c4ac8f63de849e7855a5.png';
import mealPhoto2 from 'figma:asset/74c03ecdc0c56a8c574c5aef5ecdfa704770a75e.png';
import mealPhoto3 from 'figma:asset/2d792d4c86c4ad1bd0b76d5123476c6975538c42.png';
import backArrowIcon from 'figma:asset/aff6363f5686293c593cc0511601c06031634575.png';

export interface HACCPRating {
  taste: number;
  hygiene: number;
  freshness: number;
  temperature: number;
  packaging: number;
  handling: number;
}

export interface Review {
  id: string;
  kitchenId: string;
  reviewerName: string;
  reviewerType: 'consumer' | 'supplier' | 'kitchen';
  ratings: HACCPRating;
  comment: string;
  date: string;
  verified: boolean;
  photos?: string[];
  // New fields for enhanced validation
  verificationStatus: 'unverified' | 'in_progress' | 'verified';
  reportSource: 'public' | 'official_inspector' | 'health_worker';
  confidenceLevel: 'low' | 'medium' | 'high';
  rootCauses?: ('temperature_storage' | 'cross_contamination' | 'equipment_sanitation' | 'worker_hygiene' | 'supply_chain')[];
  evidence?: {
    photoTimestamp?: string;
    menuCode?: string;
    schoolLocation?: string;
    consumptionTime?: string;
    symptoms?: string[];
  };
  isDraft?: boolean;
  disputeStatus?: 'none' | 'disputed' | 'under_review' | 'resolved';
  disputeHistory?: {
    timestamp: string;
    action: string;
    by: string;
    notes: string;
  }[];
}

export interface PerformanceBadge {
  type: 'gold' | 'silver' | 'improvement';
  title: string;
  description: string;
  earnedDate: string;
}

export interface Kitchen {
  id: string;
  name: string;
  location: string;
  type: string;
  mealsServed: number;
  certifications: string[];
  image: string;
  reviews: Review[];
  performanceBadges?: PerformanceBadge[];
  complianceTrend?: {
    month: string;
    score: number;
    incidents: number;
  }[];
}

// Mock data
const mockKitchens: Kitchen[] = [
  {
    id: '1',
    name: 'Dapur Sehat Jakarta Pusat',
    location: 'Jakarta Pusat, DKI Jakarta',
    type: 'Central Kitchen',
    mealsServed: 5000,
    certifications: ['HACCP Certified', 'Halal', 'ISO 22000'],
    image: 'commercial kitchen clean',
    reviews: [
      {
        id: 'r1',
        kitchenId: '1',
        reviewerName: 'SD Negeri 01',
        reviewerType: 'consumer',
        ratings: { taste: 4.5, hygiene: 5, freshness: 4.8, temperature: 5, packaging: 4.7, handling: 5 },
        comment: 'Makanan tiba dengan kondisi sangat baik, suhu terjaga, dan anak-anak sangat menyukai rasanya.',
        date: '2025-11-25',
        verified: true,
        photos: [
          mealPhoto1,
          mealPhoto2
        ],
        verificationStatus: 'verified',
        reportSource: 'public',
        confidenceLevel: 'high',
        rootCauses: ['temperature_storage'],
        evidence: {
          photoTimestamp: '2025-11-25T10:00:00Z',
          menuCode: 'MNU-001',
          schoolLocation: 'Jakarta Pusat',
          consumptionTime: '2025-11-25T12:00:00Z',
          symptoms: ['sakit kepala', 'muntah']
        },
        isDraft: false,
        disputeStatus: 'none',
        disputeHistory: []
      },
      {
        id: 'r2',
        kitchenId: '1',
        reviewerName: 'CV Sayur Segar',
        reviewerType: 'supplier',
        ratings: { taste: 4, hygiene: 5, freshness: 5, temperature: 5, packaging: 4.5, handling: 5 },
        comment: 'Penanganan bahan baku sangat profesional, cold chain terjaga dengan baik.',
        date: '2025-11-24',
        verified: true,
        photos: [
          mealPhoto3
        ],
        verificationStatus: 'verified',
        reportSource: 'official_inspector',
        confidenceLevel: 'high',
        rootCauses: ['equipment_sanitation'],
        evidence: {
          photoTimestamp: '2025-11-24T14:00:00Z',
          menuCode: 'MNU-002',
          schoolLocation: 'Jakarta Pusat',
          consumptionTime: '2025-11-24T16:00:00Z',
          symptoms: ['sakit perut']
        },
        isDraft: false,
        disputeStatus: 'none',
        disputeHistory: []
      }
    ],
    performanceBadges: [
      {
        type: 'gold',
        title: 'Best Practice',
        description: 'Kepatuhan HACCP yang sangat baik',
        earnedDate: '2025-11-01'
      }
    ],
    complianceTrend: [
      {
        month: '2025-11',
        score: 95,
        incidents: 0
      },
      {
        month: '2025-10',
        score: 90,
        incidents: 1
      }
    ]
  },
  {
    id: '2',
    name: 'Dapur Gizi Surabaya',
    location: 'Surabaya, Jawa Timur',
    type: 'Regional Kitchen',
    mealsServed: 3500,
    certifications: ['HACCP Certified', 'Halal'],
    image: 'industrial kitchen food',
    reviews: [
      {
        id: 'r3',
        kitchenId: '2',
        reviewerName: 'SMP Negeri 15',
        reviewerType: 'consumer',
        ratings: { taste: 4.8, hygiene: 4.9, freshness: 4.7, temperature: 4.8, packaging: 4.6, handling: 4.8 },
        comment: 'Menu bervariasi dan bergizi, packaging rapi dan higienis.',
        date: '2025-11-23',
        verified: true,
        photos: [
          'https://images.unsplash.com/photo-1691388280849-ff87c0c024f3?crop=entropy&cs=tinysrgb&fit=max&fm=jpg&ixid=M3w3Nzg4Nzd8MHwxfHNlYXJjaHwxfHxjYWZldGVyaWElMjBmb29kJTIwdHJheXxlbnwxfHx8fDE3NjQ0MTg1NzZ8MA&ixlib=rb-4.1.0&q=80&w=1080'
        ],
        verificationStatus: 'verified',
        reportSource: 'health_worker',
        confidenceLevel: 'medium',
        rootCauses: ['cross_contamination'],
        evidence: {
          photoTimestamp: '2025-11-23T09:00:00Z',
          menuCode: 'MNU-003',
          schoolLocation: 'Surabaya',
          consumptionTime: '2025-11-23T11:00:00Z',
          symptoms: ['diare']
        },
        isDraft: false,
        disputeStatus: 'none',
        disputeHistory: []
      }
    ],
    performanceBadges: [
      {
        type: 'silver',
        title: 'Good Practice',
        description: 'Kepatuhan HACCP yang baik',
        earnedDate: '2025-11-01'
      }
    ],
    complianceTrend: [
      {
        month: '2025-11',
        score: 85,
        incidents: 1
      },
      {
        month: '2025-10',
        score: 80,
        incidents: 2
      }
    ]
  },
  {
    id: '3',
    name: 'Dapur Bergizi Bandung',
    location: 'Bandung, Jawa Barat',
    type: 'Central Kitchen',
    mealsServed: 4200,
    certifications: ['HACCP Certified', 'Halal', 'BPOM'],
    image: 'professional kitchen staff',
    reviews: [],
    performanceBadges: [
      {
        type: 'improvement',
        title: 'Needs Improvement',
        description: 'Kepatuhan HACCP yang perlu ditingkatkan',
        earnedDate: '2025-11-01'
      }
    ],
    complianceTrend: [
      {
        month: '2025-11',
        score: 75,
        incidents: 2
      },
      {
        month: '2025-10',
        score: 70,
        incidents: 3
      }
    ]
  }
];

export default function App() {
  // Authentication state
  const [authState, setAuthState] = useState<'public' | 'login' | 'register' | 'authenticated' | 'publicLocation' | 'publicIncidentMap' | 'publicKitchenDetail'>('public');
  const [currentUser, setCurrentUser] = useState<User | null>(null);
  
  const [selectedKitchen, setSelectedKitchen] = useState<Kitchen | null>(null);
  const [showReviewForm, setShowReviewForm] = useState(false);
  const [kitchens, setKitchens] = useState<Kitchen[]>(mockKitchens);
  const [activeTab, setActiveTab] = useState<'dashboard' | 'review' | 'map' | 'education' | 'profile'>('dashboard');
  const [mobileTab, setMobileTab] = useState<'dashboard' | 'review' | 'map' | 'education' | 'profile'>('dashboard');
  
  // Mobile map modal state
  const [showMobileMapModal, setShowMobileMapModal] = useState(false);

  // Mock notifications
  const [notifications, setNotifications] = useState<Notification[]>([
    {
      id: 'n1',
      title: 'Makanan kurang matang',
      description: 'Laporan dari SD Negeri 05 tentang ayam yang kurang matang',
      category: 'temperature',
      priority: 'critical',
      date: '2025-12-01',
      kitchenCode: 'KTCH-001',
      schoolCode: 'SCHL-005',
      reviewId: 'r1',
      status: 'new',
      targetRole: 'kitchen',
      createdBy: 'SCHL-005',
      auditTrail: [
        {
          timestamp: '2025-12-01T10:30:00Z',
          action: 'Created',
          user: 'SCHL-005'
        }
      ]
    },
    {
      id: 'n2',
      title: 'Kemasan rusak',
      description: 'Beberapa kemasan ditemukan bocor saat pengiriman',
      category: 'packaging',
      priority: 'medium',
      date: '2025-11-30',
      kitchenCode: 'KTCH-002',
      schoolCode: 'SCHL-012',
      reviewId: 'r2',
      status: 'new',
      targetRole: 'kitchen',
      createdBy: 'SCHL-012',
      auditTrail: [
        {
          timestamp: '2025-11-30T14:20:00Z',
          action: 'Created',
          user: 'SCHL-012'
        }
      ]
    },
    {
      id: 'n3',
      title: 'Kualitas sayuran baik',
      description: 'Sayuran segar dan berkualitas tinggi',
      category: 'freshness',
      priority: 'minor',
      date: '2025-11-29',
      kitchenCode: 'KTCH-001',
      reviewId: 'r3',
      status: 'viewed',
      targetRole: 'all',
      createdBy: 'SUPL-008',
      auditTrail: [
        {
          timestamp: '2025-11-29T09:00:00Z',
          action: 'Created',
          user: 'SUPL-008'
        },
        {
          timestamp: '2025-11-29T11:30:00Z',
          action: 'Viewed',
          user: 'KTCH-001'
        }
      ]
    }
  ]);

  const handleMarkAsRead = (id: string) => {
    setNotifications(prev => prev.map(n => 
      n.id === id ? {
        ...n,
        status: 'viewed' as const,
        auditTrail: [
          ...n.auditTrail,
          {
            timestamp: new Date().toISOString(),
            action: 'Viewed',
            user: currentUser?.uniqueCode || 'unknown'
          }
        ]
      } : n
    ));
  };

  const handleMarkAsResolved = (id: string) => {
    setNotifications(prev => prev.map(n => 
      n.id === id ? {
        ...n,
        status: 'resolved' as const,
        auditTrail: [
          ...n.auditTrail,
          {
            timestamp: new Date().toISOString(),
            action: 'Resolved',
            user: currentUser?.uniqueCode || 'unknown'
          }
        ]
      } : n
    ));
  };

  const handleViewDetail = (reviewId: string) => {
    // Find kitchen with this review
    const kitchen = kitchens.find(k => k.reviews.some(r => r.id === reviewId));
    if (kitchen) {
      setSelectedKitchen(kitchen);
      setActiveTab('review');
      setMobileTab('review');
    }
  };

  const handleFollowUp = (id: string) => {
    // Handle follow up action
    console.log('Following up notification:', id);
    // In real app, this would open a follow-up form
  };

  const handleLogin = (uniqueCode: string) => {
    // Mock user creation based on code
    const role = uniqueCode.startsWith('KTCH') ? 'kitchen' : 
                 uniqueCode.startsWith('SUPL') ? 'supplier' : 'school';
    
    const user: User = {
      id: `user-${Date.now()}`,
      name: 'Demo User',
      role: role as 'kitchen' | 'supplier' | 'school',
      uniqueCode,
      phone: '08123456789',
      verified: true,
      institutionName: role === 'kitchen' ? 'Dapur Sehat Jakarta Pusat' : 
                       role === 'supplier' ? 'CV Sayur Segar' : 'SD Negeri 01',
      institutionId: '1',
      createdAt: new Date().toISOString(),
      lastLogin: new Date().toISOString()
    };
    
    setCurrentUser(user);
    setAuthState('authenticated');
  };

  const handleRegistration = (data: any) => {
    // Mock registration completion
    handleLogin(data.uniqueCode);
  };

  const handleLogout = () => {
    setCurrentUser(null);
    setAuthState('login');
    setSelectedKitchen(null);
    setShowReviewForm(false);
  };

  // Show authentication pages if not logged in
  if (authState === 'public') {
    return (
      <PublicDashboard
        onLoginClick={() => setAuthState('login')}
        onViewIncidentMap={() => setAuthState('publicIncidentMap')}
        onViewKitchenLocation={() => setAuthState('publicLocation')}
        onViewKitchenDetail={(kitchenId) => {
          const kitchen = kitchens.find(k => k.id === kitchenId);
          if (kitchen) {
            setSelectedKitchen(kitchen);
            setAuthState('publicKitchenDetail');
          }
        }}
      />
    );
  }

  // Public location filter view
  if (authState === 'publicLocation') {
    return (
      <div className="min-h-screen bg-gradient-to-br from-green-50 via-white to-emerald-50">
        <header className="bg-white border-b border-green-100 sticky top-0 z-40 shadow-sm">
          <div className="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8">
            <div className="flex items-center justify-between h-20">
              <button
                onClick={() => setAuthState('public')}
                className="flex items-center gap-2 text-green-700 hover:text-green-800 transition-colors"
              >
                <img 
                  src={backArrowIcon} 
                  alt="Back" 
                  className="size-5 transform scale-x-[-1]" 
                />
                Kembali ke Beranda
              </button>
              <button
                onClick={() => setAuthState('login')}
                className="px-4 py-2 bg-green-600 text-white rounded-xl hover:bg-green-700 transition-colors"
              >
                Login
              </button>
            </div>
          </div>
        </header>
        <main className="max-w-7xl mx-auto px-4 py-8">
          <div className="mb-8">
            <h1 className="text-green-900 mb-2">Cari Dapur MBG per Lokasi</h1>
            <p className="text-gray-600">Temukan dapur makan bergizi gratis di seluruh Indonesia</p>
          </div>
          <LocationFilter
            onSelectKitchen={(locationKitchen) => {
              // Convert LocationFilter kitchen to App kitchen type
              const appKitchen = kitchens.find(k => k.id === locationKitchen.id);
              if (appKitchen) {
                setSelectedKitchen(appKitchen);
              } else {
                // Create a new kitchen object with empty reviews
                setSelectedKitchen({
                  id: locationKitchen.id,
                  name: locationKitchen.name,
                  location: `${locationKitchen.city}, ${locationKitchen.province}`,
                  type: 'Central Kitchen',
                  mealsServed: locationKitchen.capacity,
                  certifications: locationKitchen.certifications,
                  image: 'commercial kitchen clean',
                  reviews: []
                });
              }
              setAuthState('publicKitchenDetail');
            }}
          />
        </main>
      </div>
    );
  }

  // Public incident map view
  if (authState === 'publicIncidentMap') {
    return (
      <div className="min-h-screen bg-gradient-to-br from-green-50 via-white to-emerald-50">
        <header className="bg-white border-b border-green-100 sticky top-0 z-40 shadow-sm">
          <div className="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8">
            <div className="flex items-center justify-between h-20">
              <button
                onClick={() => setAuthState('public')}
                className="flex items-center gap-2 text-green-700 hover:text-green-800 transition-colors"
              >
                <img src={backArrowIcon} alt="Back" className="size-5" />
                Kembali ke Beranda
              </button>
              <button
                onClick={() => setAuthState('login')}
                className="px-4 py-2 bg-green-600 text-white rounded-xl hover:bg-green-700 transition-colors"
              >
                Login
              </button>
            </div>
          </div>
        </header>
        <main className="max-w-7xl mx-auto px-4 py-8">
          <div className="mb-8">
            <h1 className="text-green-900 mb-2 font-bold">Peta Insiden Keamanan Pangan MBG</h1>
            <p className="text-gray-600">Visualisasi insiden keracunan makanan di seluruh Indonesia</p>
          </div>
          <IncidentMap />
        </main>
      </div>
    );
  }

  // Public kitchen detail view
  if (authState === 'publicKitchenDetail' && selectedKitchen) {
    return (
      <div className="min-h-screen bg-gradient-to-br from-green-50 via-white to-emerald-50">
        <header className="bg-white border-b border-green-100 sticky top-0 z-40 shadow-sm">
          <div className="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8">
            <div className="flex items-center justify-between h-20">
              <button
                onClick={() => {
                  setSelectedKitchen(null);
                  setAuthState('publicLocation');
                }}
                className="flex items-center gap-2 text-green-700 hover:text-green-800 transition-colors"
              >
                <img src={backArrowIcon} alt="Back" className="size-5" />
                Kembali ke Daftar Dapur
              </button>
              <button
                onClick={() => setAuthState('login')}
                className="px-4 py-2 bg-green-600 text-white rounded-xl hover:bg-green-700 transition-colors"
              >
                Login untuk Review
              </button>
            </div>
          </div>
        </header>
        <main className="max-w-7xl mx-auto px-4 py-8">
          <KitchenDetail
            kitchen={selectedKitchen}
            onBack={() => {
              setSelectedKitchen(null);
              setAuthState('publicLocation');
            }}
            onAddReview={() => {
              // Redirect to login for review
              alert('Silakan login terlebih dahulu untuk memberikan review');
              setAuthState('login');
            }}
          />
        </main>
      </div>
    );
  }

  if (authState === 'login') {
    return <LoginPage 
      onLogin={handleLogin} 
      onRegister={() => setAuthState('register')}
      onBackToDashboard={() => setAuthState('public')}
    />;
  }

  if (authState === 'register') {
    return <RegistrationPage onBack={() => setAuthState('login')} onComplete={handleRegistration} />;
  }

  // Main application (authenticated users only)
  const handleAddReview = (review: Omit<Review, 'id' | 'date'>) => {
    if (!selectedKitchen) return;

    const newReview: Review = {
      ...review,
      id: `r${Date.now()}`,
      date: new Date().toISOString().split('T')[0]
    };

    const updatedKitchens = kitchens.map(k => 
      k.id === selectedKitchen.id 
        ? { ...k, reviews: [...k.reviews, newReview] }
        : k
    );

    setKitchens(updatedKitchens);
    setSelectedKitchen({ ...selectedKitchen, reviews: [...selectedKitchen.reviews, newReview] });
    setShowReviewForm(false);
  };

  return (
    <div className="min-h-screen bg-gradient-to-br from-green-50 via-white to-emerald-50">
      {/* Dashboard Header */}
      <DashboardHeader 
        user={currentUser!} 
        notifications={notifications}
        onMarkAsRead={handleMarkAsRead}
        onMarkAsResolved={handleMarkAsResolved}
        onViewDetail={handleViewDetail}
        onFollowUp={handleFollowUp}
        onLogout={handleLogout} 
      />

      {/* Main Content */}
      <main className="max-w-7xl mx-auto px-4 py-8 space-y-8 pb-24 md:pb-8">
        {/* HACCP Education Section */}
        <HACCPEducation />

        {/* Quick Actions */}
        <div className="flex items-center justify-between">
          <div>
            <h2 className="text-emerald-900 font-bold">Dashboard</h2>
            <p className="text-emerald-600">Selamat datang, {currentUser?.name}</p>
          </div>
          <QuickTutorial role={currentUser!.role} />
        </div>

        {/* Navigation Tabs */}
        <div className="bg-white rounded-2xl shadow-sm p-2 inline-flex gap-2 flex-wrap">
          <button
            onClick={() => {
              setActiveTab('reviews');
              setSelectedKitchen(null);
              setShowReviewForm(false);
            }}
            className={`flex items-center gap-2 px-6 py-3 rounded-xl transition-all ${
              activeTab === 'reviews'
                ? 'bg-emerald-600 text-white shadow-md'
                : 'text-gray-600 hover:bg-gray-50'
            }`}
          >
            <ChefHat className="size-5" />
            <span className="font-bold">Review Dapur</span>
          </button>
          <button
            onClick={() => {
              setActiveTab('incidents');
              setSelectedKitchen(null);
              setShowReviewForm(false);
            }}
            className={`flex items-center gap-2 px-6 py-3 rounded-xl transition-all ${
              activeTab === 'incidents'
                ? 'bg-emerald-600 text-white shadow-md'
                : 'text-gray-600 hover:bg-gray-50'
            }`}
          >
            <MapIcon className="size-5" />
            <span className="font-bold">Peta Insiden</span>
          </button>
        </div>

        {/* Content */}
        {activeTab === 'reviews' ? (
          selectedKitchen ? (
            <KitchenDetail
              kitchen={selectedKitchen}
              onBack={() => {
                setSelectedKitchen(null);
                setShowReviewForm(false);
              }}
              onAddReview={() => setShowReviewForm(true)}
            />
          ) : (
            <KitchenList
              kitchens={kitchens}
              onSelectKitchen={setSelectedKitchen}
            />
          )
        ) : (
          <IncidentMap />
        )}

        {/* Review Form Modal */}
        {showReviewForm && selectedKitchen && (
          <ReviewForm
            kitchenId={selectedKitchen.id}
            onSubmit={handleAddReview}
            onClose={() => setShowReviewForm(false)}
          />
        )}
      </main>

      {/* Footer */}
      <footer className="bg-emerald-900 text-emerald-50 mt-16 py-8">
        <div className="max-w-7xl mx-auto px-4 text-center">
          <p className="text-sm">
            Program Makan Bergizi Gratis - Sistem Review Berbasis HACCP
          </p>
          <p className="text-xs text-emerald-300 mt-2">
            Memastikan kualitas, keamanan, dan gizi makanan untuk generasi Indonesia
          </p>
        </div>
      </footer>
    </div>
  );
}