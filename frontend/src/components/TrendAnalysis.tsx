import { TrendingUp, TrendingDown, Minus, AlertCircle } from 'lucide-react';

interface ComplianceTrendData {
  month: string;
  score: number;
  incidents: number;
}

interface TrendAnalysisProps {
  trendData: ComplianceTrendData[];
}

export function TrendAnalysis({ trendData }: TrendAnalysisProps) {
  if (!trendData || trendData.length === 0) {
    return (
      <div className="bg-white rounded-3xl p-8 border border-emerald-100">
        <h3 className="text-emerald-900 mb-4">Analisis Tren Performa</h3>
        <p className="text-gray-600">Data tren belum tersedia</p>
      </div>
    );
  }

  const latest = trendData[0];
  const previous = trendData[1];
  
  const scoreDiff = latest && previous ? latest.score - previous.score : 0;
  const incidentDiff = latest && previous ? previous.incidents - latest.incidents : 0;
  
  const getTrendIcon = (diff: number) => {
    if (diff > 0) return TrendingUp;
    if (diff < 0) return TrendingDown;
    return Minus;
  };

  const getTrendColor = (diff: number, isPositiveGood: boolean = true) => {
    if (diff === 0) return 'text-gray-600';
    const isGood = isPositiveGood ? diff > 0 : diff < 0;
    return isGood ? 'text-green-600' : 'text-red-600';
  };

  const getTrendBg = (diff: number, isPositiveGood: boolean = true) => {
    if (diff === 0) return 'bg-gray-100';
    const isGood = isPositiveGood ? diff > 0 : diff < 0;
    return isGood ? 'bg-green-100' : 'bg-red-100';
  };

  const getInsight = () => {
    if (scoreDiff > 5 && incidentDiff > 0) {
      return {
        type: 'success',
        message: 'Perbaikan signifikan di bulan terakhir! Score meningkat dan insiden berkurang.',
        icon: '🎉'
      };
    } else if (scoreDiff > 0) {
      return {
        type: 'info',
        message: 'Tren positif terdeteksi. Pertahankan standar HACCP yang baik.',
        icon: '✅'
      };
    } else if (scoreDiff < -5) {
      return {
        type: 'warning',
        message: 'Perhatian diperlukan! Score menurun signifikan bulan ini.',
        icon: '⚠️'
      };
    }
    return {
      type: 'neutral',
      message: 'Performa stabil. Terus monitor compliance HACCP.',
      icon: '📊'
    };
  };

  const insight = getInsight();
  const ScoreTrendIcon = getTrendIcon(scoreDiff);
  const IncidentTrendIcon = getTrendIcon(incidentDiff);

  const maxScore = Math.max(...trendData.map(d => d.score));
  const maxIncidents = Math.max(...trendData.map(d => d.incidents));

  return (
    <div className="bg-white rounded-3xl p-8 border border-emerald-100">
      <div className="flex items-center justify-between mb-6">
        <h3 className="text-emerald-900">Analisis Tren Performa</h3>
        <div className="text-xs text-gray-500">
          {trendData.length} bulan terakhir
        </div>
      </div>

      {/* Insight Alert */}
      <div className={`mb-6 p-4 rounded-xl border-2 ${
        insight.type === 'success' ? 'bg-green-50 border-green-200' :
        insight.type === 'warning' ? 'bg-orange-50 border-orange-200' :
        insight.type === 'info' ? 'bg-blue-50 border-blue-200' :
        'bg-gray-50 border-gray-200'
      }`}>
        <div className="flex items-start gap-3">
          <span className="text-2xl">{insight.icon}</span>
          <div className="flex-1">
            <p className={`${
              insight.type === 'success' ? 'text-green-900' :
              insight.type === 'warning' ? 'text-orange-900' :
              insight.type === 'info' ? 'text-blue-900' :
              'text-gray-900'
            }`}>
              <strong>Insight Otomatis:</strong> {insight.message}
            </p>
          </div>
        </div>
      </div>

      {/* Metrics */}
      <div className="grid grid-cols-1 md:grid-cols-2 gap-6 mb-8">
        {/* Compliance Score */}
        <div className="space-y-3">
          <div className="flex items-center justify-between">
            <h4 className="text-gray-900">Compliance Score</h4>
            <div className={`flex items-center gap-1 px-3 py-1 rounded-lg ${getTrendBg(scoreDiff)}`}>
              <ScoreTrendIcon className={`size-4 ${getTrendColor(scoreDiff)}`} />
              <span className={`text-sm ${getTrendColor(scoreDiff)}`}>
                {scoreDiff > 0 ? '+' : ''}{scoreDiff}%
              </span>
            </div>
          </div>
          <div className="text-3xl text-emerald-600">{latest.score}%</div>
          <div className="text-sm text-gray-600">Bulan {latest.month}</div>
        </div>

        {/* Incidents */}
        <div className="space-y-3">
          <div className="flex items-center justify-between">
            <h4 className="text-gray-900">Jumlah Insiden</h4>
            <div className={`flex items-center gap-1 px-3 py-1 rounded-lg ${getTrendBg(incidentDiff, false)}`}>
              <IncidentTrendIcon className={`size-4 ${getTrendColor(incidentDiff, false)}`} />
              <span className={`text-sm ${getTrendColor(incidentDiff, false)}`}>
                {incidentDiff > 0 ? '-' : '+'}{Math.abs(incidentDiff)}
              </span>
            </div>
          </div>
          <div className="text-3xl text-gray-900">{latest.incidents}</div>
          <div className="text-sm text-gray-600">Bulan {latest.month}</div>
        </div>
      </div>

      {/* Trend Chart */}
      <div className="space-y-4">
        <h4 className="text-gray-900">Grafik Tren Bulanan</h4>
        
        {/* Compliance Score Chart */}
        <div>
          <p className="text-sm text-gray-600 mb-2">Compliance Score (%)</p>
          <div className="space-y-2">
            {trendData.slice().reverse().map((data, idx) => (
              <div key={idx} className="flex items-center gap-3">
                <span className="text-xs text-gray-600 w-16">{data.month}</span>
                <div className="flex-1 bg-gray-100 rounded-full h-8 overflow-hidden">
                  <div
                    className="h-full bg-gradient-to-r from-emerald-500 to-emerald-600 rounded-full flex items-center justify-end pr-3 transition-all"
                    style={{ width: `${data.score}%` }}
                  >
                    <span className="text-white text-xs">{data.score}%</span>
                  </div>
                </div>
              </div>
            ))}
          </div>
        </div>

        {/* Incidents Chart */}
        <div>
          <p className="text-sm text-gray-600 mb-2">Jumlah Insiden</p>
          <div className="space-y-2">
            {trendData.slice().reverse().map((data, idx) => {
              const width = maxIncidents > 0 ? (data.incidents / maxIncidents) * 100 : 0;
              const color = data.incidents === 0 ? 'bg-green-500' : 
                           data.incidents <= 1 ? 'bg-yellow-500' : 
                           'bg-red-500';
              
              return (
                <div key={idx} className="flex items-center gap-3">
                  <span className="text-xs text-gray-600 w-16">{data.month}</span>
                  <div className="flex-1 bg-gray-100 rounded-full h-8 overflow-hidden">
                    {data.incidents > 0 ? (
                      <div
                        className={`h-full ${color} rounded-full flex items-center justify-end pr-3 transition-all`}
                        style={{ width: `${Math.max(width, 15)}%` }}
                      >
                        <span className="text-white text-xs">{data.incidents}</span>
                      </div>
                    ) : (
                      <div className="h-full flex items-center px-3">
                        <span className="text-xs text-green-700">0 insiden ✓</span>
                      </div>
                    )}
                  </div>
                </div>
              );
            })}
          </div>
        </div>
      </div>
    </div>
  );
}
