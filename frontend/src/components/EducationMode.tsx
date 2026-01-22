import { Book, AlertTriangle, Lightbulb, Shield, CheckCircle2, ChevronDown, X, Info } from 'lucide-react';
import { useState } from 'react';

export function EducationMode() {
  const [expandedSection, setExpandedSection] = useState<string | null>(null);

  const toggleSection = (section: string) => {
    setExpandedSection(expandedSection === section ? null : section);
  };

  return (
    <div className="bg-white rounded-2xl p-6 md:p-8 border border-emerald-200 shadow-sm">
      <div className="flex items-center gap-3 mb-6">
        <div className="p-2 bg-emerald-100 rounded-xl">
          <Book className="size-6 text-emerald-700" />
        </div>
        <div>
          <h2 className="text-emerald-900 font-bold">Panduan Keamanan Pangan</h2>
          <p className="text-sm text-emerald-600">Pahami standar HACCP berbasis sains</p>
        </div>
      </div>

      <div className="space-y-3">
        {/* HACCP Definition - Accordion */}
        <div className="border border-gray-300 rounded-xl overflow-hidden transition-all duration-300">
          <button
            onClick={() => toggleSection('haccp')}
            className="w-full flex items-center justify-between p-4 bg-slate-100 hover:bg-slate-200 transition-colors"
            aria-expanded={expandedSection === 'haccp'}
            aria-controls="haccp-content"
          >
            <div className="flex items-center gap-3">
              <div className="p-2 bg-slate-600 rounded-lg">
                <Shield className="size-5 text-white" />
              </div>
              <div className="text-left">
                <h3 className="text-slate-800 font-bold">Definisi HACCP</h3>
                <p className="text-sm text-slate-600">Apa itu sistem keamanan pangan HACCP?</p>
              </div>
            </div>
            <ChevronDown 
              className={`size-6 text-slate-600 transition-transform duration-300 ${
                expandedSection === 'haccp' ? 'rotate-180' : ''
              }`}
            />
          </button>
          
          <div 
            id="haccp-content"
            className={`transition-all duration-300 ease-in-out overflow-hidden ${
              expandedSection === 'haccp' ? 'max-h-[2000px] opacity-100' : 'max-h-0 opacity-0'
            }`}
            role="region"
            aria-labelledby="haccp-button"
          >
            <div className="p-6 bg-gradient-to-br from-slate-50 to-gray-100 border-t border-gray-300">
              <div className="flex items-start justify-between mb-4">
                <div className="flex-1">
                  <p className="text-slate-800 mb-4 leading-relaxed">
                    <strong>Hazard Analysis and Critical Control Points (HACCP)</strong> adalah sistem manajemen keamanan pangan 
                    yang mengidentifikasi, mengevaluasi, dan mengendalikan bahaya yang signifikan bagi keamanan pangan.
                  </p>
                </div>
                <button
                  onClick={() => setExpandedSection(null)}
                  className="ml-4 p-1 hover:bg-slate-200 rounded-lg transition-colors"
                  aria-label="Close"
                >
                  <X className="size-5 text-slate-600" />
                </button>
              </div>
              <div className="grid grid-cols-1 md:grid-cols-2 gap-4">
                <div className="bg-white rounded-xl p-4 border border-gray-200">
                  <h4 className="text-slate-800 font-bold mb-2">6 Kriteria Utama</h4>
                  <ul className="text-sm text-slate-700 space-y-1">
                    <li>✓ Rasa (Taste)</li>
                    <li>✓ Kebersihan (Hygiene)</li>
                    <li>✓ Kesegaran (Freshness)</li>
                    <li>✓ Kontrol Suhu (Temperature)</li>
                    <li>✓ Kemasan (Packaging)</li>
                    <li>✓ Penanganan (Handling)</li>
                  </ul>
                </div>
                <div className="bg-white rounded-xl p-4 border border-gray-200">
                  <h4 className="text-slate-800 font-bold mb-2">Manfaat HACCP</h4>
                  <ul className="text-sm text-slate-700 space-y-1">
                    <li>• Mencegah keracunan makanan</li>
                    <li>• Meningkatkan kualitas produk</li>
                    <li>• Meminimalkan risiko kesehatan</li>
                    <li>• Meningkatkan kepercayaan publik</li>
                  </ul>
                </div>
              </div>
            </div>
          </div>
        </div>

        {/* Myths vs Facts - Accordion */}
        <div className="border border-gray-300 rounded-xl overflow-hidden transition-all duration-300">
          <button
            onClick={() => toggleSection('myths')}
            className="w-full flex items-center justify-between p-4 bg-orange-100 hover:bg-orange-200 transition-colors"
            aria-expanded={expandedSection === 'myths'}
            aria-controls="myths-content"
          >
            <div className="flex items-center gap-3">
              <div className="p-2 bg-orange-600 rounded-lg">
                <AlertTriangle className="size-5 text-white" />
              </div>
              <div className="text-left">
                <h3 className="text-slate-800 font-bold">Mitos atau Fakta?</h3>
                <p className="text-sm text-slate-600">Pelajari kesalahpahaman umum tentang keamanan pangan</p>
              </div>
            </div>
            <ChevronDown 
              className={`size-6 text-slate-600 transition-transform duration-300 ${
                expandedSection === 'myths' ? 'rotate-180' : ''
              }`}
            />
          </button>
          
          <div 
            id="myths-content"
            className={`transition-all duration-300 ease-in-out overflow-hidden ${
              expandedSection === 'myths' ? 'max-h-[2000px] opacity-100' : 'max-h-0 opacity-0'
            }`}
            role="region"
            aria-labelledby="myths-button"
          >
            <div className="p-6 bg-gradient-to-br from-orange-50 to-amber-50 border-t border-gray-300">
              <div className="flex items-start justify-between mb-4">
                <div className="flex-1 space-y-4">
                  <div className="bg-white rounded-xl p-4 border border-gray-200">
                    <p className="text-slate-800 mb-2">
                      <strong>❌ Mitos:</strong> "Makanan yang rasanya enak pasti aman"
                    </p>
                    <p className="text-sm text-slate-700">
                      <strong>✓ Fakta:</strong> Bakteri berbahaya tidak selalu mengubah rasa, bau, atau penampilan makanan. 
                      Pemeriksaan suhu dan higiene tetap diperlukan.
                    </p>
                  </div>
                  <div className="bg-white rounded-xl p-4 border border-gray-200">
                    <p className="text-slate-800 mb-2">
                      <strong>❌ Mitos:</strong> "Keracunan makanan selalu langsung terasa"
                    </p>
                    <p className="text-sm text-slate-700">
                      <strong>✓ Fakta:</strong> Gejala bisa muncul dari 30 menit hingga beberapa hari setelah konsumsi, 
                      tergantung jenis kontaminan.
                    </p>
                  </div>
                  <div className="bg-white rounded-xl p-4 border border-gray-200">
                    <p className="text-slate-800 mb-2">
                      <strong>❌ Mitos:</strong> "Memanaskan kembali makanan membunuh semua bakteri"
                    </p>
                    <p className="text-sm text-slate-700">
                      <strong>✓ Fakta:</strong> Beberapa bakteri menghasilkan toksin yang tahan panas. 
                      Penyimpanan yang benar lebih penting daripada pemanasan ulang.
                    </p>
                  </div>
                </div>
                <button
                  onClick={() => setExpandedSection(null)}
                  className="ml-4 p-1 hover:bg-orange-200 rounded-lg transition-colors"
                  aria-label="Close"
                >
                  <X className="size-5 text-slate-600" />
                </button>
              </div>
            </div>
          </div>
        </div>

        {/* Opinion vs Evidence - Accordion */}
        <div className="border border-gray-300 rounded-xl overflow-hidden transition-all duration-300">
          <button
            onClick={() => toggleSection('evidence')}
            className="w-full flex items-center justify-between p-4 bg-blue-100 hover:bg-blue-200 transition-colors"
            aria-expanded={expandedSection === 'evidence'}
            aria-controls="evidence-content"
          >
            <div className="flex items-center gap-3">
              <div className="p-2 bg-blue-600 rounded-lg">
                <Lightbulb className="size-5 text-white" />
              </div>
              <div className="text-left">
                <h3 className="text-slate-800 font-bold">Perbedaan Opini vs Temuan Berbukti</h3>
                <p className="text-sm text-slate-600">Pahami perbedaan laporan subjektif dan objektif</p>
              </div>
            </div>
            <ChevronDown 
              className={`size-6 text-slate-600 transition-transform duration-300 ${
                expandedSection === 'evidence' ? 'rotate-180' : ''
              }`}
            />
          </button>
          
          <div 
            id="evidence-content"
            className={`transition-all duration-300 ease-in-out overflow-hidden ${
              expandedSection === 'evidence' ? 'max-h-[2000px] opacity-100' : 'max-h-0 opacity-0'
            }`}
            role="region"
            aria-labelledby="evidence-button"
          >
            <div className="p-6 bg-gradient-to-br from-blue-50 to-cyan-50 border-t border-gray-300">
              <div className="flex items-start justify-between mb-4">
                <div className="flex-1">
                  <div className="grid grid-cols-1 md:grid-cols-2 gap-4 mb-4">
                    <div className="bg-white rounded-xl p-4 border-l-4 border-red-400">
                      <h4 className="text-slate-800 font-bold mb-2 flex items-center gap-2">
                        <span className="text-xl">💬</span>
                        Opini Subjektif
                      </h4>
                      <ul className="text-sm text-slate-700 space-y-1">
                        <li>• "Makanannya tidak enak"</li>
                        <li>• "Sepertinya tidak bersih"</li>
                        <li>• "Saya merasa kurang puas"</li>
                        <li>• "Porsi terlalu sedikit"</li>
                      </ul>
                      <p className="text-xs text-slate-600 mt-3 italic">
                        ⚠️ Opini pribadi tanpa bukti objektif
                      </p>
                    </div>
                    <div className="bg-white rounded-xl p-4 border-l-4 border-green-500">
                      <h4 className="text-slate-800 font-bold mb-2 flex items-center gap-2">
                        <span className="text-xl">📊</span>
                        Temuan Berbukti
                      </h4>
                      <ul className="text-sm text-slate-700 space-y-1">
                        <li>• "Suhu makanan 35°C (seharusnya &gt;60°C)"</li>
                        <li>• "Ditemukan rambut dalam makanan"</li>
                        <li>• "Kemasan bocor dan rusak"</li>
                        <li>• "Foto menunjukkan makanan berjamur"</li>
                      </ul>
                      <p className="text-xs text-slate-600 mt-3 italic">
                        ✅ Observasi objektif dengan bukti fisik
                      </p>
                    </div>
                  </div>
                  
                  <div className="bg-amber-50 rounded-xl p-4 border border-amber-200">
                    <p className="text-sm text-slate-800">
                      <strong>💡 Sistem Review MBG:</strong> Kami mewajibkan foto, timestamp, dan identifikasi root cause 
                      untuk memastikan semua laporan berbasis bukti, bukan asumsi. Ini melindungi dapur dari tuduhan tidak berdasar 
                      sekaligus memastikan masalah nyata ditangani dengan serius.
                    </p>
                  </div>
                </div>
                <button
                  onClick={() => setExpandedSection(null)}
                  className="ml-4 p-1 hover:bg-blue-200 rounded-lg transition-colors"
                  aria-label="Close"
                >
                  <X className="size-5 text-slate-600" />
                </button>
              </div>
            </div>
          </div>
        </div>

        {/* Note about reporting */}
        <div className="bg-gradient-to-r from-slate-700 to-slate-800 rounded-xl p-6 text-white">
          <div className="flex items-center gap-3 mb-3">
            <Info className="size-6" />
            <h4 className="font-bold">Temukan Masalah Keamanan Pangan?</h4>
          </div>
          <p className="text-sm text-slate-200 mb-4">
            Gunakan tombol accordion di atas untuk memahami cara melaporkan dengan benar. 
            Pastikan laporan Anda berbasis bukti objektif, bukan opini subjektif.
          </p>
          <p className="text-xs text-slate-300">
            💡 Tip: Ambil foto dengan timestamp, catat detail spesifik, dan identifikasi root cause HACCP
          </p>
        </div>
      </div>
    </div>
  );
}