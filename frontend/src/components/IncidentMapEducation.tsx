import { Info, Shield, AlertTriangle, CheckCircle2, MapPin, Phone, FileText, Eye, ChevronDown, X } from 'lucide-react';
import { useState } from 'react';

export function IncidentMapEducation() {
  const [expandedSection, setExpandedSection] = useState<string | null>(null);

  const toggleSection = (sectionId: string) => {
    setExpandedSection(expandedSection === sectionId ? null : sectionId);
  };

  return (
    <div className="bg-white rounded-3xl border border-blue-200 overflow-hidden shadow-lg mb-8">
      {/* Header */}
      <div className="bg-gradient-to-r from-blue-600 to-blue-700 p-8 text-white">
        <div className="flex items-start gap-4">
          <div className="p-3 bg-white/20 rounded-xl">
            <Info className="size-8 text-white" />
          </div>
          <div className="flex-1">
            <h2 className="text-white mb-3 font-bold">Tentang Peta Insiden Keamanan Pangan MBG</h2>
            <p className="text-blue-100 leading-relaxed">
              Peta Insiden MBG adalah sistem pemantauan keamanan pangan real-time untuk Program Makan Bergizi Gratis (MBG) 
              yang mencatat dan melacak kejadian dugaan keracunan makanan di seluruh wilayah Indonesia. Sistem ini dirancang 
              untuk transparansi, perlindungan konsumen, dan percepatan investigasi serta perbaikan layanan pangan sekolah.
            </p>
          </div>
        </div>
      </div>

      <div className="p-8 space-y-8">
        {/* Tujuan */}
        <section>
          <div className="flex items-center gap-3 mb-4">
            <div className="p-2 bg-emerald-100 rounded-lg">
              <Shield className="size-6 text-emerald-600" />
            </div>
            <h3 className="text-emerald-900 font-bold">Tujuan Visualisasi Peta Insiden</h3>
          </div>
          <div className="bg-emerald-50 rounded-2xl p-6 border border-emerald-200">
            <ul className="space-y-3 text-emerald-800">
              <li className="flex gap-3">
                <span className="text-emerald-600 flex-shrink-0">•</span>
                <span><strong>Transparansi:</strong> Memberikan akses informasi terbuka kepada publik tentang kejadian keamanan pangan</span>
              </li>
              <li className="flex gap-3">
                <span className="text-emerald-600 flex-shrink-0">•</span>
                <span><strong>Perlindungan Konsumen:</strong> Membantu sekolah dan orang tua mengidentifikasi dan menghindari risiko keamanan pangan</span>
              </li>
              <li className="flex gap-3">
                <span className="text-emerald-600 flex-shrink-0">•</span>
                <span><strong>Percepatan Investigasi:</strong> Memudahkan tim pengawas untuk merespons insiden dengan cepat dan tepat</span>
              </li>
              <li className="flex gap-3">
                <span className="text-emerald-600 flex-shrink-0">•</span>
                <span><strong>Perbaikan Berkelanjutan:</strong> Mengidentifikasi pola dan area yang memerlukan peningkatan sistem keamanan pangan</span>
              </li>
            </ul>
          </div>
        </section>

        {/* Legenda Warna */}
        <section>
          <div className="flex items-center gap-3 mb-4">
            <div className="p-2 bg-purple-100 rounded-lg">
              <MapPin className="size-6 text-purple-600" />
            </div>
            <h3 className="text-gray-900 font-bold">Makna Indikator pada Peta</h3>
          </div>
          <div className="grid md:grid-cols-3 gap-4">
            <div className="bg-red-50 border-2 border-red-300 rounded-2xl p-6">
              <div className="flex items-center gap-2 mb-3">
                <div className="size-4 rounded-full bg-red-600 animate-pulse" />
                <span className="text-red-900 font-bold font-[Abril_Fatface]">Merah - Kritis</span>
              </div>
              <p className="text-sm text-red-700">
                Kasus yang memerlukan penanganan cepat dan perhatian khusus. Biasanya melibatkan jumlah korban yang signifikan 
                atau gejala yang serius.
              </p>
            </div>
            <div className="bg-orange-50 border-2 border-orange-300 rounded-2xl p-6">
              <div className="flex items-center gap-2 mb-3">
                <div className="size-4 rounded-full bg-orange-500" />
                <span className="text-orange-900 font-[Abril_Fatface] font-bold">Oranye - Investigasi</span>
              </div>
              <p className="text-sm text-orange-700">
                Kasus dalam status investigasi aktif. Tim sedang mengumpulkan bukti, memeriksa dapur, dan mengidentifikasi 
                sumber masalah.
              </p>
            </div>
            <div className="bg-green-50 border-2 border-green-300 rounded-2xl p-6">
              <div className="flex items-center gap-2 mb-3">
                <div className="size-4 rounded-full bg-green-600" />
                <span className="text-green-900 font-bold font-[Abril_Fatface]">Hijau - Selesai</span>
              </div>
              <p className="text-sm text-green-700">
                Kasus yang sudah diselesaikan dan ditindaklanjuti dengan perbaikan sistem. Dapur telah memenuhi standar 
                keamanan kembali.
              </p>
            </div>
          </div>
          <div className="mt-4 bg-gray-50 rounded-2xl p-5 border border-gray-200">
            <p className="text-sm text-gray-700">
              <strong>Interpretasi Angka:</strong> Total insiden menunjukkan kumulatif laporan yang masuk ke sistem. 
              Total korban terpapar adalah jumlah individu yang mengalami gejala terkait kejadian keracunan makanan. 
              Angka ini diperbarui secara real-time berdasarkan laporan terverifikasi.
            </p>
          </div>
        </section>

        {/* ACCORDION SECTIONS */}
        <div className="space-y-4">
          {/* Alur Tindak Lanjut Insiden - Accordion */}
          <div className="border border-gray-200 rounded-2xl overflow-hidden bg-white shadow-sm">
            <button
              onClick={() => toggleSection('alur')}
              onKeyDown={(e) => {
                if (e.key === 'Enter' || e.key === ' ') {
                  e.preventDefault();
                  toggleSection('alur');
                }
              }}
              className="w-full flex items-center justify-between p-5 hover:bg-gray-50 transition-colors focus:outline-none focus:ring-2 focus:ring-green-500 focus:ring-inset"
              aria-expanded={expandedSection === 'alur'}
              aria-controls="alur-content"
              role="button"
              tabIndex={0}
            >
              <div className="flex items-center gap-3">
                <div className="p-2 bg-blue-100 rounded-lg">
                  <FileText className="size-5 text-blue-600" />
                </div>
                <h3 className="text-gray-900 font-bold">Alur Tindak Lanjut Insiden</h3>
              </div>
              <div className="flex items-center gap-2">
                <Info className="size-5 text-gray-400" />
                <ChevronDown
                  className={`size-5 text-gray-600 transition-transform duration-300 ${
                    expandedSection === 'alur' ? 'rotate-180' : ''
                  }`}
                />
              </div>
            </button>

            {expandedSection === 'alur' && (
              <div
                id="alur-content"
                role="region"
                aria-labelledby="alur-button"
                className="border-t border-gray-200 animate-in slide-in-from-top-2 duration-300"
              >
                <div className="p-6 relative">
                  <button
                    onClick={() => setExpandedSection(null)}
                    className="absolute top-4 right-4 p-2 hover:bg-gray-100 rounded-lg transition-colors"
                    aria-label="Tutup"
                  >
                    <X className="size-4 text-gray-500" />
                  </button>

                  <div className="bg-gradient-to-br from-blue-50 to-cyan-50 rounded-2xl p-6 border border-blue-200">
                    <div className="space-y-4">
                      {[
                        {
                          step: '1',
                          title: 'Konfirmasi Laporan',
                          desc: 'Verifikasi informasi dari pelapor dan pengumpulan data awal'
                        },
                        {
                          step: '2',
                          title: 'Pemeriksaan Sumber',
                          desc: 'Inspeksi dapur, supplier, dan rantai distribusi makanan'
                        },
                        {
                          step: '3',
                          title: 'Intervensi Dapur/Supplier',
                          desc: 'Penghentian sementara operasi jika diperlukan, perbaikan proses HACCP'
                        },
                        {
                          step: '4',
                          title: 'Rekomendasi Perbaikan',
                          desc: 'Penyusunan action plan dan implementasi standar keamanan baru'
                        },
                        {
                          step: '5',
                          title: 'Pemantauan Ulang',
                          desc: 'Follow-up berkala untuk memastikan perbaikan berkelanjutan'
                        }
                      ].map((item, idx) => (
                        <div key={idx} className="flex gap-4 items-start">
                          <div className="flex items-center justify-center size-10 bg-blue-600 text-white rounded-full flex-shrink-0">
                            {item.step}
                          </div>
                          <div className="flex-1 pt-1">
                            <p className="text-blue-900 mb-1">{item.title}</p>
                            <p className="text-sm text-blue-700">{item.desc}</p>
                          </div>
                        </div>
                      ))}
                    </div>
                  </div>
                </div>
              </div>
            )}
          </div>

          {/* Privasi & Akurasi Data - Accordion */}
          <div className="border border-gray-200 rounded-2xl overflow-hidden bg-white shadow-sm">
            <button
              onClick={() => toggleSection('privasi')}
              onKeyDown={(e) => {
                if (e.key === 'Enter' || e.key === ' ') {
                  e.preventDefault();
                  toggleSection('privasi');
                }
              }}
              className="w-full flex items-center justify-between p-5 hover:bg-gray-50 transition-colors focus:outline-none focus:ring-2 focus:ring-green-500 focus:ring-inset"
              aria-expanded={expandedSection === 'privasi'}
              aria-controls="privasi-content"
              role="button"
              tabIndex={0}
            >
              <div className="flex items-center gap-3">
                <div className="p-2 bg-amber-100 rounded-lg">
                  <Eye className="size-5 text-amber-600" />
                </div>
                <h3 className="text-gray-900 font-bold">Privasi & Akurasi Data</h3>
              </div>
              <div className="flex items-center gap-2">
                <Info className="size-5 text-gray-400" />
                <ChevronDown
                  className={`size-5 text-gray-600 transition-transform duration-300 ${
                    expandedSection === 'privasi' ? 'rotate-180' : ''
                  }`}
                />
              </div>
            </button>

            {expandedSection === 'privasi' && (
              <div
                id="privasi-content"
                role="region"
                aria-labelledby="privasi-button"
                className="border-t border-gray-200 animate-in slide-in-from-top-2 duration-300"
              >
                <div className="p-6 relative">
                  <button
                    onClick={() => setExpandedSection(null)}
                    className="absolute top-4 right-4 p-2 hover:bg-gray-100 rounded-lg transition-colors"
                    aria-label="Tutup"
                  >
                    <X className="size-4 text-gray-500" />
                  </button>

                  <div className="bg-amber-50 rounded-2xl p-6 border border-amber-200">
                    <div className="space-y-3 text-amber-900">
                      <p className="flex gap-2">
                        <AlertTriangle className="size-5 text-amber-600 flex-shrink-0 mt-0.5" />
                        <span>
                          <strong>Verifikasi Data:</strong> Semua laporan diverifikasi oleh tim pengawas resmi sebelum ditampilkan
                          di peta untuk memastikan akurasi dan menghindari informasi yang menyesatkan.
                        </span>
                      </p>
                      <p className="flex gap-2">
                        <Shield className="size-5 text-amber-600 flex-shrink-0 mt-0.5" />
                        <span>
                          <strong>Perlindungan Identitas:</strong> Identitas individu yang terdampak dirahasiakan sesuai dengan
                          etika privasi dan untuk menghindari stigma sosial. Hanya data agregat dan lokasi umum yang ditampilkan.
                        </span>
                      </p>
                      <p className="flex gap-2">
                        <CheckCircle2 className="size-5 text-amber-600 flex-shrink-0 mt-0.5" />
                        <span>
                          <strong>Hindari Kesimpulan Prematur:</strong> Data peta tidak boleh digunakan untuk menyimpulkan
                          kesalahan dapur tanpa investigasi lengkap. Insiden dapat disebabkan berbagai faktor di luar kontrol dapur.
                        </span>
                      </p>
                    </div>
                  </div>
                </div>
              </div>
            )}
          </div>

          {/* Cara Publik Berpartisipasi - Accordion */}
          <div className="border border-gray-200 rounded-2xl overflow-hidden bg-white shadow-sm">
            <button
              onClick={() => toggleSection('partisipasi')}
              onKeyDown={(e) => {
                if (e.key === 'Enter' || e.key === ' ') {
                  e.preventDefault();
                  toggleSection('partisipasi');
                }
              }}
              className="w-full flex items-center justify-between p-5 hover:bg-gray-50 transition-colors focus:outline-none focus:ring-2 focus:ring-green-500 focus:ring-inset"
              aria-expanded={expandedSection === 'partisipasi'}
              aria-controls="partisipasi-content"
              role="button"
              tabIndex={0}
            >
              <div className="flex items-center gap-3">
                <div className="p-2 bg-green-100 rounded-lg">
                  <Phone className="size-5 text-green-600" />
                </div>
                <h3 className="text-gray-900 font-bold">Cara Publik Berpartisipasi dengan Aman</h3>
              </div>
              <div className="flex items-center gap-2">
                <Info className="size-5 text-gray-400" />
                <ChevronDown
                  className={`size-5 text-gray-600 transition-transform duration-300 ${
                    expandedSection === 'partisipasi' ? 'rotate-180' : ''
                  }`}
                />
              </div>
            </button>

            {expandedSection === 'partisipasi' && (
              <div
                id="partisipasi-content"
                role="region"
                aria-labelledby="partisipasi-button"
                className="border-t border-gray-200 animate-in slide-in-from-top-2 duration-300"
              >
                <div className="p-6 relative">
                  <button
                    onClick={() => setExpandedSection(null)}
                    className="absolute top-4 right-4 p-2 hover:bg-gray-100 rounded-lg transition-colors"
                    aria-label="Tutup"
                  >
                    <X className="size-4 text-gray-500" />
                  </button>

                  <div className="bg-green-50 rounded-2xl p-6 border border-green-200">
                    <div className="space-y-4">
                      <div>
                        <p className="text-green-900 mb-2"><strong>Saluran Pelaporan Resmi:</strong></p>
                        <ul className="space-y-2 text-sm text-green-800 ml-4">
                          <li>• Hotline MBG: <strong>0800-123-4567</strong> (24/7)</li>
                          <li>• Email: <strong>lapor@mbg.go.id</strong></li>
                          <li>• Form online di dashboard sistem (untuk pengguna terdaftar)</li>
                          <li>• WhatsApp pengawas wilayah (dapat diakses di halaman kontak)</li>
                        </ul>
                      </div>
                      <div>
                        <p className="text-green-900 mb-2"><strong>Gejala yang Harus Dilaporkan:</strong></p>
                        <ul className="space-y-2 text-sm text-green-800 ml-4">
                          <li>• Mual, muntah, atau diare dalam 2-6 jam setelah makan</li>
                          <li>• Demam, sakit perut, atau lemas yang tidak wajar</li>
                          <li>• Lebih dari 3 orang mengalami gejala serupa setelah makan dari sumber yang sama</li>
                          <li>• Makanan dengan tampilan, bau, atau rasa mencurigakan</li>
                        </ul>
                      </div>
                      <div className="bg-white rounded-xl p-4 border border-green-300">
                        <p className="text-sm text-green-900">
                          <strong>💡 Tips Pelaporan:</strong> Sertakan foto makanan (jika memungkinkan), waktu konsumsi,
                          nama dapur/sekolah, dan gejala yang dialami. Informasi yang akurat dan lengkap membantu
                          percepatan investigasi.
                        </p>
                      </div>
                    </div>
                  </div>
                </div>
              </div>
            )}
          </div>
        </div>

        {/* Pesan Penutup */}
        <section className="bg-gradient-to-r from-emerald-600 to-teal-600 rounded-2xl p-8 text-white">
          <div className="flex items-start gap-4">
            <div className="p-3 bg-white/20 rounded-xl">
              <Shield className="size-8 text-white" />
            </div>
            <div>
              <h3 className="text-white mb-3">Bersama Meningkatkan Keamanan Pangan</h3>
              <p className="text-emerald-100 leading-relaxed mb-4">
                Setiap laporan yang Anda berikan membantu meningkatkan sistem keamanan pangan dan kualitas 
                Program Makan Bergizi Gratis bagi seluruh anak sekolah di Indonesia. Transparansi data ini 
                adalah bentuk akuntabilitas publik untuk memastikan hak anak-anak mendapatkan makanan yang 
                aman, bergizi, dan berkualitas tinggi.
              </p>
              <p className="text-sm text-emerald-200">
                Mari bersama-sama menjaga kesehatan generasi masa depan Indonesia melalui sistem pemantauan 
                yang berbasis data, transparan, dan responsif.
              </p>
            </div>
          </div>
        </section>
      </div>
    </div>
  );
}