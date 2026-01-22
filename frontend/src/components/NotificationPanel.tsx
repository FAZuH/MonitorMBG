import { useState, useEffect, useRef } from 'react';
import { Bell, X, CheckCircle2, Eye, ArrowRight, AlertCircle, Flame, Droplet, ThermometerSnowflake, Package } from 'lucide-react';
import { User } from '../types';

export interface Notification {
  id: string;
  title: string;
  description: string;
  category: 'hygiene' | 'temperature' | 'cross_contamination' | 'freshness' | 'packaging' | 'handling';
  priority: 'critical' | 'medium' | 'minor';
  date: string;
  kitchenCode: string;
  schoolCode?: string;
  reviewId: string;
  status: 'new' | 'viewed' | 'resolved';
  targetRole: 'all' | 'kitchen' | 'supplier' | 'consumer';
  createdBy: string;
  auditTrail: {
    timestamp: string;
    action: string;
    user: string;
  }[];
}

interface NotificationPanelProps {
  user: User | null;
  notifications: Notification[];
  onMarkAsRead: (id: string) => void;
  onMarkAsResolved: (id: string) => void;
  onViewDetail: (reviewId: string) => void;
  onFollowUp: (id: string) => void;
}

export function NotificationPanel({
  user,
  notifications,
  onMarkAsRead,
  onMarkAsResolved,
  onViewDetail,
  onFollowUp
}: NotificationPanelProps) {
  const [isOpen, setIsOpen] = useState(false);
  const panelRef = useRef<HTMLDivElement>(null);

  // Filter notifications based on user role
  const filteredNotifications = notifications.filter(notif => {
    if (!user) return false;
    
    // Admin sees all
    if (user.role === 'admin') return true;
    
    // Consumer only sees their own reports
    if (user.role === 'school') {
      return notif.createdBy === user.uniqueCode || notif.schoolCode === user.uniqueCode;
    }
    
    // Kitchen sees kitchen-related notifications
    if (user.role === 'kitchen') {
      return notif.targetRole === 'kitchen' || notif.targetRole === 'all';
    }
    
    // Supplier sees supplier-related notifications
    if (user.role === 'supplier') {
      return notif.targetRole === 'supplier' || notif.targetRole === 'all';
    }
    
    return false;
  });

  const unreadCount = filteredNotifications.filter(n => n.status === 'new').length;

  // Close panel when clicking outside
  useEffect(() => {
    const handleClickOutside = (event: MouseEvent) => {
      if (panelRef.current && !panelRef.current.contains(event.target as Node)) {
        setIsOpen(false);
      }
    };

    if (isOpen) {
      document.addEventListener('mousedown', handleClickOutside);
    }

    return () => {
      document.removeEventListener('mousedown', handleClickOutside);
    };
  }, [isOpen]);

  const getCategoryIcon = (category: string) => {
    switch (category) {
      case 'hygiene': return <Droplet className="size-4" />;
      case 'temperature': return <ThermometerSnowflake className="size-4" />;
      case 'cross_contamination': return <AlertCircle className="size-4" />;
      case 'freshness': return <Flame className="size-4" />;
      case 'packaging': return <Package className="size-4" />;
      default: return <AlertCircle className="size-4" />;
    }
  };

  const getCategoryLabel = (category: string) => {
    const labels: Record<string, string> = {
      hygiene: 'Kebersihan',
      temperature: 'Kontrol Suhu',
      cross_contamination: 'Kontaminasi Silang',
      freshness: 'Kesegaran',
      packaging: 'Kemasan',
      handling: 'Penanganan'
    };
    return labels[category] || category;
  };

  const getPriorityColor = (priority: string) => {
    switch (priority) {
      case 'critical': return 'bg-red-100 border-red-300 text-red-900';
      case 'medium': return 'bg-orange-100 border-orange-300 text-orange-900';
      case 'minor': return 'bg-green-100 border-green-300 text-green-900';
      default: return 'bg-gray-100 border-gray-300 text-gray-900';
    }
  };

  const getPriorityBadge = (priority: string) => {
    switch (priority) {
      case 'critical': return 'bg-red-500 text-white';
      case 'medium': return 'bg-orange-500 text-white';
      case 'minor': return 'bg-green-500 text-white';
      default: return 'bg-gray-500 text-white';
    }
  };

  return (
    <div className="relative" ref={panelRef}>
      {/* Notification Bell Icon */}
      <button
        onClick={() => setIsOpen(!isOpen)}
        className="relative p-2 hover:bg-green-50 rounded-xl transition-colors"
        aria-label="Notifications"
      >
        <Bell className="size-6 text-green-700" />
        {unreadCount > 0 && (
          <span className="absolute -top-1 -right-1 size-5 bg-red-500 text-white text-xs rounded-full flex items-center justify-center animate-pulse">
            {unreadCount > 9 ? '9+' : unreadCount}
          </span>
        )}
      </button>

      {/* Notification Dropdown Panel */}
      {isOpen && (
        <div className="absolute right-0 top-full mt-2 w-96 max-h-[600px] bg-white rounded-2xl shadow-2xl border border-gray-200 overflow-hidden z-50">
          {/* Header */}
          <div className="bg-gradient-to-r from-green-600 to-green-700 p-4 text-white">
            <div className="flex items-center justify-between">
              <div className="flex items-center gap-2">
                <Bell className="size-5" />
                <h3 className="text-white font-bold">Notifikasi</h3>
              </div>
              <button
                onClick={() => setIsOpen(false)}
                className="p-1 hover:bg-white/20 rounded-lg transition-colors font-bold"
              >
                <X className="size-5" />
              </button>
            </div>
            <p className="text-sm text-green-100 mt-1">
              {unreadCount} notifikasi baru
            </p>
          </div>

          {/* Notifications List */}
          <div className="overflow-y-auto max-h-[500px]">
            {filteredNotifications.length === 0 ? (
              <div className="p-8 text-center">
                <Bell className="size-12 text-gray-300 mx-auto mb-3" />
                <p className="text-gray-500">Tidak ada notifikasi</p>
              </div>
            ) : (
              <div className="divide-y divide-gray-100">
                {filteredNotifications.map((notif) => (
                  <div
                    key={notif.id}
                    className={`p-4 hover:bg-gray-50 transition-colors ${
                      notif.status === 'new' ? 'bg-green-50/50' : ''
                    }`}
                  >
                    {/* Priority Badge */}
                    <div className="flex items-start gap-3 mb-2">
                      <div className={`p-2 rounded-lg ${getPriorityColor(notif.priority)}`}>
                        {getCategoryIcon(notif.category)}
                      </div>
                      <div className="flex-1">
                        <div className="flex items-start justify-between gap-2 mb-1">
                          <h4 className="text-sm text-gray-900 flex-1 font-bold">{notif.title}</h4>
                          <span className={`px-2 py-0.5 rounded-full text-xs ${getPriorityBadge(notif.priority)}`}>
                            {notif.priority === 'critical' ? 'Kritis' : notif.priority === 'medium' ? 'Sedang' : 'Minor'}
                          </span>
                        </div>
                        <p className="text-sm text-gray-600 mb-2">{notif.description}</p>
                        
                        {/* Meta Info */}
                        <div className="flex flex-wrap gap-2 text-xs text-gray-500 mb-3">
                          <span className="bg-gray-100 px-2 py-1 rounded">
                            {getCategoryLabel(notif.category)}
                          </span>
                          <span className="bg-gray-100 px-2 py-1 rounded">
                            📅 {new Date(notif.date).toLocaleDateString('id-ID')}
                          </span>
                          <span className="bg-gray-100 px-2 py-1 rounded">
                            🏢 {notif.kitchenCode}
                          </span>
                          {notif.schoolCode && (
                            <span className="bg-gray-100 px-2 py-1 rounded">
                              🏫 {notif.schoolCode}
                            </span>
                          )}
                        </div>

                        {/* Actions */}
                        <div className="flex flex-wrap gap-2">
                          <button
                            onClick={() => {
                              onViewDetail(notif.reviewId);
                              if (notif.status === 'new') onMarkAsRead(notif.id);
                              setIsOpen(false);
                            }}
                            className="flex items-center gap-1 px-3 py-1.5 bg-blue-600 text-white rounded-lg hover:bg-blue-700 transition-colors text-xs"
                          >
                            <Eye className="size-3" />
                            Lihat Detail
                          </button>
                          
                          {(user?.role === 'kitchen' || user?.role === 'supplier') && notif.status !== 'resolved' && (
                            <button
                              onClick={() => {
                                onFollowUp(notif.id);
                                if (notif.status === 'new') onMarkAsRead(notif.id);
                              }}
                              className="flex items-center gap-1 px-3 py-1.5 bg-orange-600 text-white rounded-lg hover:bg-orange-700 transition-colors text-xs"
                            >
                              <ArrowRight className="size-3" />
                              Tindak Lanjuti
                            </button>
                          )}
                          
                          {notif.status !== 'resolved' && (
                            <button
                              onClick={() => onMarkAsResolved(notif.id)}
                              className="flex items-center gap-1 px-3 py-1.5 bg-green-600 text-white rounded-lg hover:bg-green-700 transition-colors text-xs"
                            >
                              <CheckCircle2 className="size-3" />
                              Tandai Selesai
                            </button>
                          )}
                        </div>

                        {/* Status Badge */}
                        {notif.status === 'resolved' && (
                          <div className="mt-2 inline-flex items-center gap-1 text-xs text-green-700 bg-green-100 px-2 py-1 rounded">
                            <CheckCircle2 className="size-3" />
                            Sudah Diselesaikan
                          </div>
                        )}
                      </div>
                    </div>
                  </div>
                ))}
              </div>
            )}
          </div>

          {/* Footer */}
          {filteredNotifications.length > 0 && (
            <div className="p-3 bg-gray-50 border-t border-gray-200 text-center">
              <button className="text-sm text-green-700 hover:text-green-800">
                Lihat Semua Notifikasi
              </button>
            </div>
          )}
        </div>
      )}
    </div>
  );
}
