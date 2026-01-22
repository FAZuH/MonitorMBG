import { Award, TrendingUp, Star, Shield } from 'lucide-react';
import { PerformanceBadge } from '../App';

interface PerformanceBadgesProps {
  badges: PerformanceBadge[];
}

export function PerformanceBadges({ badges }: PerformanceBadgesProps) {
  if (!badges || badges.length === 0) {
    return (
      <div className="bg-gray-50 border border-gray-200 rounded-2xl p-6">
        <p className="text-gray-600 text-center">Belum ada badge performa</p>
      </div>
    );
  }

  const getBadgeConfig = (type: 'gold' | 'silver' | 'improvement') => {
    switch (type) {
      case 'gold':
        return {
          icon: Award,
          gradient: 'from-yellow-400 to-yellow-600',
          bg: 'bg-yellow-50',
          border: 'border-yellow-300',
          text: 'text-yellow-900',
          glow: 'shadow-yellow-200'
        };
      case 'silver':
        return {
          icon: Shield,
          gradient: 'from-gray-300 to-gray-500',
          bg: 'bg-gray-50',
          border: 'border-gray-300',
          text: 'text-gray-900',
          glow: 'shadow-gray-200'
        };
      case 'improvement':
        return {
          icon: TrendingUp,
          gradient: 'from-emerald-400 to-emerald-600',
          bg: 'bg-emerald-50',
          border: 'border-emerald-300',
          text: 'text-emerald-900',
          glow: 'shadow-emerald-200'
        };
    }
  };

  return (
    <div className="bg-white rounded-3xl p-8 border border-emerald-100">
      <div className="flex items-center gap-3 mb-6">
        <div className="p-2 bg-yellow-100 rounded-xl">
          <Star className="size-6 text-yellow-600" />
        </div>
        <div>
          <h3 className="text-emerald-900">Performance Badges</h3>
          <p className="text-sm text-emerald-600">Apresiasi atas kinerja HACCP yang baik</p>
        </div>
      </div>

      <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-4">
        {badges.map((badge, idx) => {
          const config = getBadgeConfig(badge.type);
          const Icon = config.icon;

          return (
            <div
              key={idx}
              className={`relative overflow-hidden rounded-2xl border-2 ${config.border} ${config.bg} p-6 hover:scale-105 transition-transform ${config.glow} shadow-lg`}
            >
              {/* Badge Icon */}
              <div className="flex items-center justify-center mb-4">
                <div className={`p-4 bg-gradient-to-br ${config.gradient} rounded-2xl shadow-lg`}>
                  <Icon className="size-8 text-white" />
                </div>
              </div>

              {/* Badge Info */}
              <div className="text-center space-y-2">
                <h4 className={`${config.text}`}>{badge.title}</h4>
                <p className="text-sm text-gray-600">{badge.description}</p>
                <div className="pt-3 border-t border-gray-200">
                  <p className="text-xs text-gray-500">
                    Diraih: {new Date(badge.earnedDate).toLocaleDateString('id-ID', {
                      day: 'numeric',
                      month: 'long',
                      year: 'numeric'
                    })}
                  </p>
                </div>
              </div>

              {/* Decorative Elements */}
              <div className="absolute top-0 right-0 w-20 h-20 bg-white/20 rounded-full -mr-10 -mt-10"></div>
              <div className="absolute bottom-0 left-0 w-16 h-16 bg-white/20 rounded-full -ml-8 -mb-8"></div>
            </div>
          );
        })}
      </div>

      {/* Info Box */}
      <div className="mt-6 bg-blue-50 border border-blue-200 rounded-xl p-4">
        <div className="flex items-start gap-3">
          <Star className="size-5 text-blue-600 flex-shrink-0 mt-0.5" />
          <div>
            <h4 className="text-blue-900 mb-1">Tentang Badge System</h4>
            <ul className="text-sm text-blue-800 space-y-1">
              <li><strong>🥇 Gold Badge:</strong> 3 bulan berturut-turut tanpa insiden</li>
              <li><strong>🥈 Silver Badge:</strong> Audit higienitas dengan hasil memuaskan</li>
              <li><strong>📈 Improvement Badge:</strong> Tren membaik secara konsisten</li>
            </ul>
            <p className="text-xs text-blue-700 mt-2 italic">
              Badge ditampilkan untuk menyeimbangkan narasi dan memberikan apresiasi atas upaya perbaikan berkelanjutan.
            </p>
          </div>
        </div>
      </div>
    </div>
  );
}
