import { useState } from 'react';
import { Database, MapPin, TrendingUp, BarChart3, Download, Filter } from 'lucide-react';
import { AreaChart, Area, BarChart, Bar, XAxis, YAxis, CartesianGrid, Tooltip, Legend, ResponsiveContainer, LineChart, Line } from 'recharts';

// Mock data untuk statistik
const regionStats = [
  { region: 'Jakarta', kitchens: 45, avgCompliance: 92.5, incidents: 3 },
  { region: 'Jawa Barat', kitchens: 78, avgCompliance: 89.2, incidents: 8 },
  { region: 'Jawa Tengah', kitchens: 62, avgCompliance: 91.8, incidents: 4 },
  { region: 'Jawa Timur', kitchens: 71, avgCompliance: 88.5, incidents: 9 },
  { region: 'Sumatra Utara', kitchens: 52, avgCompliance: 90.1, incidents: 5 },
  { region: 'Bali', kitchens: 34, avgCompliance: 94.2, incidents: 2 },
];

const complianceTrend = [
  { month: 'Jul 2025', score: 88.5, incidents: 45 },
  { month: 'Agu 2025', score: 90.2, incidents: 38 },
  { month: 'Sep 2025', score: 91.8, incidents: 31 },
  { month: 'Okt 2025', score: 92.1, incidents: 28 },
  { month: 'Nov 2025', score: 93.5, incidents: 22 },
  { month: 'Des 2025', score: 92.8, incidents: 25 },
];

const incidentsByCategory = [
  { category: 'Suhu Penyimpanan', count: 42, percentage: 28 },
  { category: 'Kontaminasi Silang', count: 35, percentage: 23 },
  { category: 'Sanitasi Peralatan', count: 31, percentage: 21 },
  { category: 'Higiene Pekerja', count: 25, percentage: 17 },
  { category: 'Supply Chain', count: 17, percentage: 11 },
];

type ViewMode = 'regional' | 'trend' | 'category';

export function OpenDataStats() {
  const [viewMode, setViewMode] = useState<ViewMode>('regional');
  const [selectedRegion, setSelectedRegion] = useState<string>('Semua Wilayah');

  return (
    <div className="bg-white rounded-3xl p-8 border border-emerald-100">
      <div className="flex items-center justify-between mb-6">
        <div className="flex items-center gap-3">
          <div className="p-2 bg-indigo-100 rounded-xl">
            <Database className="size-6 text-indigo-600" />
          </div>
          <div>
            <h2 className="text-emerald-900 font-bold">Data Publik MBG</h2>
            <p className="text-sm text-emerald-600">Statistik agregat dan tren keamanan pangan</p>
          </div>
        </div>
        <button className="flex items-center gap-2 px-4 py-2 bg-emerald-600 text-white rounded-xl hover:bg-emerald-700 transition-colors">
          <Download className="size-4" />
          <span className="text-sm font-[Abril_Fatface]">Ekspor Data</span>
        </button>
      </div>

      {/* View Mode Tabs */}
      <div className="flex gap-2 mb-6 bg-gray-100 p-1 rounded-xl">
        <button
          onClick={() => setViewMode('regional')}
          className={`flex-1 flex items-center justify-center gap-2 px-4 py-3 rounded-lg transition-all ${
            viewMode === 'regional'
              ? 'bg-white text-emerald-600 shadow-sm'
              : 'text-gray-600 hover:text-gray-900'
          }`}
        >
          <MapPin className="size-4" />
          <span className="text-sm font-[Abril_Fatface]">Per Wilayah</span>
        </button>
        <button
          onClick={() => setViewMode('trend')}
          className={`flex-1 flex items-center justify-center gap-2 px-4 py-3 rounded-lg transition-all ${
            viewMode === 'trend'
              ? 'bg-white text-emerald-600 shadow-sm'
              : 'text-gray-600 hover:text-gray-900'
          }`}
        >
          <TrendingUp className="size-4" />
          <span className="text-sm font-[Abril_Fatface]">Tren Bulanan</span>
        </button>
        <button
          onClick={() => setViewMode('category')}
          className={`flex-1 flex items-center justify-center gap-2 px-4 py-3 rounded-lg transition-all ${
            viewMode === 'category'
              ? 'bg-white text-emerald-600 shadow-sm'
              : 'text-gray-600 hover:text-gray-900'
          }`}
        >
          <BarChart3 className="size-4" />
          <span className="text-sm font-[Abril_Fatface]">Per Kategori</span>
        </button>
      </div>

      {/* Regional Stats View */}
      {viewMode === 'regional' && (
        <div className="space-y-6">
          <div className="flex items-center justify-between">
            <h3 className="text-gray-900">Statistik per Wilayah</h3>
            <select
              value={selectedRegion}
              onChange={(e) => setSelectedRegion(e.target.value)}
              className="px-4 py-2 border border-gray-300 rounded-xl focus:outline-none focus:ring-2 focus:ring-emerald-500 bg-white"
            >
              <option>Semua Wilayah</option>
              {regionStats.map((r) => (
                <option key={r.region}>{r.region}</option>
              ))}
            </select>
          </div>

          {/* Chart */}
          <div className="w-full" style={{ height: '320px' }}>
            <BarChart data={regionStats} width={800} height={320}>
              <CartesianGrid strokeDasharray="3 3" stroke="#f0f0f0" />
              <XAxis dataKey="region" tick={{ fontSize: 12 }} />
              <YAxis yAxisId="left" tick={{ fontSize: 12 }} />
              <YAxis yAxisId="right" orientation="right" tick={{ fontSize: 12 }} />
              <Tooltip 
                contentStyle={{ 
                  backgroundColor: 'white', 
                  border: '1px solid #e5e7eb',
                  borderRadius: '12px',
                  padding: '12px'
                }}
              />
              <Legend />
              <Bar yAxisId="left" dataKey="avgCompliance" fill="#10b981" name="Compliance Score (%)" />
              <Bar yAxisId="right" dataKey="incidents" fill="#ef4444" name="Jumlah Insiden" />
            </BarChart>
          </div>

          {/* Stats Cards */}
          <div className="grid grid-cols-1 md:grid-cols-3 gap-4">
            <div className="bg-emerald-50 border border-emerald-200 rounded-xl p-4">
              <p className="text-emerald-600 text-sm mb-1">Total Dapur</p>
              <p className="text-2xl text-emerald-900">{regionStats.reduce((sum, r) => sum + r.kitchens, 0)}</p>
            </div>
            <div className="bg-blue-50 border border-blue-200 rounded-xl p-4">
              <p className="text-blue-600 text-sm mb-1">Rata-rata Compliance</p>
              <p className="text-2xl text-blue-900">
                {(regionStats.reduce((sum, r) => sum + r.avgCompliance, 0) / regionStats.length).toFixed(1)}%
              </p>
            </div>
            <div className="bg-red-50 border border-red-200 rounded-xl p-4">
              <p className="text-red-600 text-sm mb-1">Total Insiden</p>
              <p className="text-2xl text-red-900">{regionStats.reduce((sum, r) => sum + r.incidents, 0)}</p>
            </div>
          </div>
        </div>
      )}

      {/* Trend View */}
      {viewMode === 'trend' && (
        <div className="space-y-6">
          <h3 className="text-gray-900">Tren Compliance 6 Bulan Terakhir</h3>
          
          {/* Chart */}
          <div className="w-full" style={{ height: '320px' }}>
            <LineChart data={complianceTrend} width={800} height={320}>
              <CartesianGrid strokeDasharray="3 3" stroke="#f0f0f0" />
              <XAxis dataKey="month" tick={{ fontSize: 12 }} />
              <YAxis yAxisId="left" tick={{ fontSize: 12 }} domain={[85, 95]} />
              <YAxis yAxisId="right" orientation="right" tick={{ fontSize: 12 }} />
              <Tooltip 
                contentStyle={{ 
                  backgroundColor: 'white', 
                  border: '1px solid #e5e7eb',
                  borderRadius: '12px',
                  padding: '12px'
                }}
              />
              <Legend />
              <Line 
                yAxisId="left" 
                type="monotone" 
                dataKey="score" 
                stroke="#10b981" 
                strokeWidth={3}
                name="Compliance Score (%)"
                dot={{ fill: '#10b981', r: 5 }}
              />
              <Line 
                yAxisId="right" 
                type="monotone" 
                dataKey="incidents" 
                stroke="#ef4444" 
                strokeWidth={3}
                name="Jumlah Insiden"
                dot={{ fill: '#ef4444', r: 5 }}
              />
            </LineChart>
          </div>

          {/* Insight */}
          <div className="bg-green-50 border border-green-200 rounded-xl p-4">
            <p className="text-green-900">
              <strong>📈 Insight:</strong> Tren compliance meningkat 4.3% dalam 6 bulan terakhir, 
              dengan penurunan insiden sebesar 44% dari Juli ke November 2025.
            </p>
          </div>
        </div>
      )}

      {/* Category View */}
      {viewMode === 'category' && (
        <div className="space-y-6">
          <h3 className="text-gray-900">Insiden Berdasarkan Kategori Root Cause</h3>
          
          {/* Chart */}
          <div className="w-full overflow-x-auto" style={{ height: '320px' }}>
            <BarChart data={incidentsByCategory} layout="vertical" width={800} height={320}>
              <CartesianGrid strokeDasharray="3 3" stroke="#f0f0f0" />
              <XAxis type="number" tick={{ fontSize: 12 }} />
              <YAxis dataKey="category" type="category" width={150} tick={{ fontSize: 12 }} />
              <Tooltip 
                contentStyle={{ 
                  backgroundColor: 'white', 
                  border: '1px solid #e5e7eb',
                  borderRadius: '12px',
                  padding: '12px'
                }}
              />
              <Bar dataKey="count" fill="#f59e0b" name="Jumlah Insiden" />
            </BarChart>
          </div>

          {/* Percentage Breakdown */}
          <div className="space-y-3">
            {incidentsByCategory.map((item, idx) => (
              <div key={idx} className="flex items-center gap-3">
                <span className="text-sm text-gray-700 w-40">{item.category}</span>
                <div className="flex-1 bg-gray-100 rounded-full h-8 overflow-hidden">
                  <div
                    className="h-full bg-gradient-to-r from-orange-400 to-orange-600 flex items-center justify-end pr-3"
                    style={{ width: `${item.percentage}%` }}
                  >
                    <span className="text-white text-xs">{item.percentage}%</span>
                  </div>
                </div>
                <span className="text-sm text-gray-600 w-12">{item.count}</span>
              </div>
            ))}
          </div>
        </div>
      )}

      {/* Info Box */}
      <div className="mt-6 bg-blue-50 border border-blue-200 rounded-xl p-4">
        <div className="flex items-start gap-3">
          <Database className="size-5 text-blue-600 flex-shrink-0 mt-0.5" />
          <div>
            <h4 className="text-blue-900 mb-1">Tentang Data Publik</h4>
            <p className="text-sm text-blue-800 leading-relaxed">
              Data ini merupakan statistik agregat dan anonim dari seluruh dapur MBG. 
              Tidak ada informasi pribadi atau data sensitif yang ditampilkan. 
              Data diperbarui setiap bulan untuk transparansi publik.
            </p>
          </div>
        </div>
      </div>
    </div>
  );
}