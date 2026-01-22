import { Code, Database, Lock, BarChart3, MapPin, TrendingUp } from 'lucide-react';

export function OpenDataAPI() {
  return (
    <div className="bg-white rounded-3xl p-8 border border-emerald-100">
      <div className="flex items-center gap-3 mb-6">
        <div className="p-2 bg-indigo-100 rounded-xl">
          <Database className="size-6 text-indigo-600" />
        </div>
        <div>
          <h2 className="text-emerald-900">Open Data API</h2>
          <p className="text-sm text-emerald-600">Akses data agregat dan statistik publik MBG</p>
        </div>
      </div>

      <div className="bg-indigo-50 border border-indigo-200 rounded-xl p-6 mb-6">
        <div className="flex items-start gap-3">
          <Lock className="size-5 text-indigo-600 flex-shrink-0 mt-0.5" />
          <div>
            <h3 className="text-indigo-900 mb-2">Akses Terbatas untuk Transparansi</h3>
            <p className="text-sm text-indigo-800 leading-relaxed">
              API ini menyediakan data agregat dan statistik anonim untuk peneliti, jurnalis, dan publik. 
              Data pribadi dan informasi sensitif tidak tersedia melalui API ini.
            </p>
          </div>
        </div>
      </div>

      <div className="space-y-6">
        {/* Endpoint 1 */}
        <div className="border border-gray-200 rounded-2xl overflow-hidden">
          <div className="bg-gradient-to-r from-blue-50 to-indigo-50 p-4 border-b border-gray-200">
            <div className="flex items-center gap-3">
              <BarChart3 className="size-5 text-blue-600" />
              <div className="flex-1">
                <h4 className="text-gray-900">Statistik Agregat per Kabupaten</h4>
                <p className="text-sm text-gray-600 mt-1">Dapatkan data insiden dan compliance agregat berdasarkan wilayah</p>
              </div>
            </div>
          </div>
          <div className="p-4 bg-gray-50">
            <div className="mb-4">
              <p className="text-xs text-gray-600 mb-2">GET Request</p>
              <div className="bg-gray-900 text-green-400 p-3 rounded-lg text-sm overflow-x-auto">
                <code>https://api.mbg.go.id/v1/stats/kabupaten?region=Jakarta</code>
              </div>
            </div>
            <div className="mb-4">
              <p className="text-xs text-gray-600 mb-2">Response Example</p>
              <div className="bg-gray-900 text-gray-300 p-3 rounded-lg text-sm overflow-x-auto">
                <pre className="text-xs">{`{
  "region": "Jakarta",
  "total_kitchens": 45,
  "avg_compliance_score": 92.5,
  "total_incidents": 3,
  "month": "2025-12"
}`}</pre>
              </div>
            </div>
          </div>
        </div>

        {/* Endpoint 2 */}
        <div className="border border-gray-200 rounded-2xl overflow-hidden">
          <div className="bg-gradient-to-r from-green-50 to-emerald-50 p-4 border-b border-gray-200">
            <div className="flex items-center gap-3">
              <TrendingUp className="size-5 text-green-600" />
              <div className="flex-1">
                <h4 className="text-gray-900">Tren Compliance per Wilayah</h4>
                <p className="text-sm text-gray-600 mt-1">Tren compliance bulanan untuk analisis pola</p>
              </div>
            </div>
          </div>
          <div className="p-4 bg-gray-50">
            <div className="mb-4">
              <p className="text-xs text-gray-600 mb-2">GET Request</p>
              <div className="bg-gray-900 text-green-400 p-3 rounded-lg text-sm overflow-x-auto">
                <code>https://api.mbg.go.id/v1/trends/compliance?region=Jakarta&months=6</code>
              </div>
            </div>
            <div className="mb-4">
              <p className="text-xs text-gray-600 mb-2">Response Example</p>
              <div className="bg-gray-900 text-gray-300 p-3 rounded-lg text-sm overflow-x-auto">
                <pre className="text-xs">{`{
  "region": "Jakarta",
  "data": [
    {"month": "2025-07", "score": 88.5, "incidents": 5},
    {"month": "2025-08", "score": 90.2, "incidents": 3},
    {"month": "2025-09", "score": 91.8, "incidents": 2},
    {"month": "2025-10", "score": 92.1, "incidents": 2},
    {"month": "2025-11", "score": 93.5, "incidents": 1},
    {"month": "2025-12", "score": 92.5, "incidents": 3}
  ]
}`}</pre>
              </div>
            </div>
          </div>
        </div>

        {/* Endpoint 3 */}
        <div className="border border-gray-200 rounded-2xl overflow-hidden">
          <div className="bg-gradient-to-r from-purple-50 to-pink-50 p-4 border-b border-gray-200">
            <div className="flex items-center gap-3">
              <MapPin className="size-5 text-purple-600" />
              <div className="flex-1">
                <h4 className="text-gray-900">Data Insiden Anonim</h4>
                <p className="text-sm text-gray-600 mt-1">Insiden keracunan makanan (data anonymized)</p>
              </div>
            </div>
          </div>
          <div className="p-4 bg-gray-50">
            <div className="mb-4">
              <p className="text-xs text-gray-600 mb-2">GET Request</p>
              <div className="bg-gray-900 text-green-400 p-3 rounded-lg text-sm overflow-x-auto">
                <code>https://api.mbg.go.id/v1/incidents?region=Jakarta&severity=critical</code>
              </div>
            </div>
            <div className="mb-4">
              <p className="text-xs text-gray-600 mb-2">Response Example</p>
              <div className="bg-gray-900 text-gray-300 p-3 rounded-lg text-sm overflow-x-auto">
                <pre className="text-xs">{`{
  "total": 3,
  "incidents": [
    {
      "id": "INC-2025-12-001",
      "region": "Jakarta",
      "severity": "critical",
      "root_cause": ["temperature_storage"],
      "date": "2025-12-01",
      "status": "resolved"
    },
    ...
  ]
}`}</pre>
              </div>
            </div>
          </div>
        </div>
      </div>

      {/* API Access Info */}
      <div className="mt-6 bg-gradient-to-br from-amber-50 to-orange-50 border border-amber-200 rounded-xl p-6">
        <div className="flex items-start gap-3">
          <Code className="size-5 text-amber-600 flex-shrink-0 mt-0.5" />
          <div>
            <h4 className="text-amber-900 mb-2">Cara Mendapatkan API Key</h4>
            <ol className="text-sm text-amber-800 space-y-2">
              <li>
                <strong>1. Daftar sebagai Peneliti/Organisasi</strong><br/>
                <span className="text-xs">Kirim email ke <code className="bg-amber-100 px-2 py-1 rounded">api@mbg.go.id</code> dengan identitas dan tujuan penggunaan</span>
              </li>
              <li>
                <strong>2. Verifikasi Identitas</strong><br/>
                <span className="text-xs">Tim kami akan memverifikasi organisasi Anda dalam 3-5 hari kerja</span>
              </li>
              <li>
                <strong>3. Terima API Key</strong><br/>
                <span className="text-xs">API key akan dikirim via email dengan dokumentasi lengkap</span>
              </li>
              <li>
                <strong>4. Gunakan Secara Bertanggung Jawab</strong><br/>
                <span className="text-xs">
                  Rate limit: 1000 requests/jam. Tidak boleh digunakan untuk komersial tanpa izin.
                </span>
              </li>
            </ol>
          </div>
        </div>
      </div>

      {/* Use Cases */}
      <div className="mt-6 grid grid-cols-1 md:grid-cols-3 gap-4">
        <div className="bg-blue-50 border border-blue-200 rounded-xl p-4">
          <h4 className="text-blue-900 mb-2">📊 Peneliti</h4>
          <p className="text-sm text-blue-800">
            Analisis pola keracunan makanan untuk penelitian kesehatan masyarakat
          </p>
        </div>
        <div className="bg-green-50 border border-green-200 rounded-xl p-4">
          <h4 className="text-green-900 mb-2">📰 Jurnalis</h4>
          <p className="text-sm text-green-800">
            Investigasi dan liputan mendalam tentang keamanan pangan program MBG
          </p>
        </div>
        <div className="bg-purple-50 border border-purple-200 rounded-xl p-4">
          <h4 className="text-purple-900 mb-2">🏛️ Pemerintah Daerah</h4>
          <p className="text-sm text-purple-800">
            Monitoring performa dapur MBG di wilayah untuk evaluasi kebijakan
          </p>
        </div>
      </div>
    </div>
  );
}
