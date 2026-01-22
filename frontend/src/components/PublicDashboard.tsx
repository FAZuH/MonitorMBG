import { Star, MapPin, TrendingUp, Shield, CheckCircle2, Eye, LogIn, FileText, Search, Book, Database, BookOpen, Youtube, Scale, Newspaper, AlertTriangle, Map } from 'lucide-react';
import { HotlineButton } from './HotlineButton';
import { EducationMode } from './EducationMode';
import { OpenDataStats } from './OpenDataStats';
import { VideoEducation } from './VideoEducation';
import logoMBG from 'figma:asset/51e94428001de32b8adf84d690546c9b5cfc362e.png';
import mealPhoto1 from 'figma:asset/f6c1a2b02ee187084390c4ac8f63de849e7855a5.png';
import mealPhoto2 from 'figma:asset/74c03ecdc0c56a8c574c5aef5ecdfa704770a75e.png';
import mealPhoto3 from 'figma:asset/2d792d4c86c4ad1bd0b76d5123476c6975538c42.png';

interface PublicDashboardProps {
  onLoginClick: () => void;
  onViewIncidentMap: () => void;
  onViewKitchenLocation: () => void;
  onViewKitchenDetail: (kitchenId: string) => void;
}

export function PublicDashboard({ onLoginClick, onViewIncidentMap, onViewKitchenLocation, onViewKitchenDetail }: PublicDashboardProps) {
  // Mock verified public reviews (anonymized)
  const publicReviews = [
    {
      id: '1',
      kitchenName: 'Dapur Gizi Jakarta Pusat',
      location: 'Jakarta Pusat, DKI Jakarta',
      rating: 4.8,
      reviewerType: 'Sekolah',
      comment: 'Makanan tiba dengan kondisi sangat baik, suhu terjaga, dan anak-anak sangat menyukai rasanya.',
      date: '2025-11-25',
      verified: true,
      photo: mealPhoto1,
      haccpScores: {
        taste: 4.8,
        hygiene: 5.0,
        freshness: 4.9,
        temperature: 5.0
      }
    },
    {
      id: '2',
      kitchenName: 'Dapur Gizi Surabaya',
      location: 'Surabaya, Jawa Timur',
      rating: 4.5,
      reviewerType: 'Supplier',
      comment: 'Penanganan bahan baku sangat profesional, cold chain terjaga dengan baik.',
      date: '2025-11-24',
      verified: true,
      photo: mealPhoto2,
      haccpScores: {
        taste: 4.5,
        hygiene: 5.0,
        freshness: 5.0,
        temperature: 5.0
      }
    },
    {
      id: '3',
      kitchenName: 'Dapur Bergizi Bandung',
      location: 'Bandung, Jawa Barat',
      rating: 4.7,
      reviewerType: 'Sekolah',
      comment: 'Kualitas makanan konsisten, kemasan rapi dan aman.',
      date: '2025-11-23',
      verified: true,
      photo: mealPhoto3,
      haccpScores: {
        taste: 4.7,
        hygiene: 4.8,
        freshness: 4.9,
        temperature: 4.8
      }
    }
  ];

  const nationalStats = {
    totalKitchens: 1248,
    totalReviews: 15678,
    averageRating: 4.6,
    verifiedIncidents: 23,
    resolvedIncidents: 18
  };

  return (
    <div className="min-h-screen bg-gradient-to-br from-emerald-50 via-green-50 to-teal-50">
      {/* Header */}
      <header className="bg-white border-b border-emerald-100 sticky top-0 z-40 shadow-sm">
        <div className="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8">
          <div className="flex items-center justify-between h-20">
            <div className="flex items-center gap-4">
              <img src={logoMBG} alt="Logo MBG" className="w-14 h-14" />
              <div>
                <h1 className="text-emerald-800 font-bold text-[13px] font-[Aclonica]">MonitorMBG</h1>
                <p className="text-sm text-emerald-600 text-[13px]">Dashboard Transparansi MBG</p>
              </div>
            </div>
            <button
              onClick={onLoginClick}
              className="flex items-center gap-2 px-6 py-3 bg-emerald-600 text-white rounded-[12px] hover:bg-emerald-700 transition-all shadow-lg hover:shadow-xl font-bold text-[11px] text-left not-italic font-[Abril_Fatface]"
            >
              <LogIn className="size-5" />
              Login
            </button>
          </div>
        </div>
      </header>

      {/* Hero Section */}
      <section className="bg-gradient-to-r from-emerald-600 to-teal-600 text-white py-16">
        <div className="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8">
          <div className="max-w-3xl">
            <h2 className="text-white mb-4 text-[20px] font-bold">
              Platform Pengawasan Keamanan Pangan Makan Bergizi Gratis
            </h2>
            <p className="text-xl text-slate-200 mb-8 leading-relaxed">
              Sistem transparansi publik untuk memastikan kualitas dan keamanan makanan bergizi gratis 
              di seluruh Indonesia berdasarkan standar HACCP.
            </p>
            
            {/* Quick Access Cards */}
            <div className="grid md:grid-cols-3 gap-6 mb-6">
              {/* Cari Dapur Card */}
              <div className="bg-white rounded-2xl p-6 shadow-lg hover:shadow-xl transition-all">
                <div className="flex items-center gap-3 mb-3">
                  <div className="p-3 bg-emerald-100 rounded-xl">
                    <Search className="size-6 text-emerald-700" />
                  </div>
                  <h3 className="text-emerald-900 font-bold">Cari Dapur</h3>
                </div>
                <p className="text-gray-600 text-sm mb-4">
                  Temukan dapur MBG terdekat dan lihat review kualitas makanan per lokasi
                </p>
                <button
                  onClick={onViewKitchenLocation}
                  className="w-full px-4 py-3 bg-emerald-600 text-white rounded-xl hover:bg-emerald-700 transition-all flex items-center justify-center gap-2 font-[Abril_Fatface]"
                >
                  <Search className="size-5" />
                  Cari Dapur
                </button>
              </div>

              {/* Panduan Gizi MBG Card */}
              <div className="bg-white rounded-2xl p-6 shadow-lg hover:shadow-xl transition-all">
                <div className="flex items-center gap-3 mb-3">
                  <div className="p-3 bg-teal-100 rounded-xl">
                    <BookOpen className="size-6 text-teal-700" />
                  </div>
                  <h3 className="text-emerald-900 font-bold">Panduan Gizi MBG</h3>
                </div>
                <p className="text-gray-600 text-sm mb-4">
                  Pelajari standar nutrisi dan pedoman gizi Makan Bergizi Gratis untuk anak Indonesia
                </p>
                <a
                  href="https://drive.google.com/file/d/1gwt9tXSqwXCNCwBfZKAAHRAtUc_FOrd8/view?usp=sharing"
                  target="_blank"
                  rel="noopener noreferrer"
                  className="w-full px-4 py-3 bg-teal-600 text-white rounded-xl hover:bg-teal-700 transition-all flex items-center justify-center gap-2 font-[Abril_Fatface]"
                >
                  <BookOpen className="size-5" />
                  Buka Panduan
                </a>
              </div>

              {/* Regulasi Terkait MBG Card */}
              <div className="bg-white rounded-2xl p-6 shadow-lg hover:shadow-xl transition-all">
                <div className="flex items-center gap-3 mb-3">
                  <div className="p-3 bg-cyan-100 rounded-xl">
                    <Scale className="size-6 text-cyan-700" />
                  </div>
                  <h3 className="text-emerald-900 font-bold">Regulasi Terkait MBG</h3>
                </div>
                <p className="text-gray-600 text-sm mb-4">
                  Akses dokumen peraturan dan regulasi resmi terkait program Makan Bergizi Gratis
                </p>
                <a
                  href="https://drive.google.com/file/d/1R-9GJDXwrI-HEAS8B27GUOyndx5AOLd0/view?usp=sharing"
                  target="_blank"
                  rel="noopener noreferrer"
                  className="w-full px-4 py-3 bg-cyan-600 text-white rounded-xl hover:bg-cyan-700 transition-all flex items-center justify-center gap-2 font-[Abril_Fatface]"
                >
                  <Scale className="size-5" />
                  Buka Regulasi
                </a>
              </div>
            </div>

            {/* Other Action Buttons */}
            <div className="flex flex-wrap gap-4">
              <button
                onClick={() => document.getElementById('reviews')?.scrollIntoView({ behavior: 'smooth' })}
                className="px-6 py-3 bg-emerald-700 text-white rounded-xl hover:bg-emerald-800 transition-all border-2 border-white/20 font-bold"
              >
                Lihat Review Publik
              </button>
              <button
                onClick={() => document.getElementById('video-education')?.scrollIntoView({ behavior: 'smooth' })}
                className="px-6 py-3 bg-emerald-700 text-white rounded-xl hover:bg-emerald-800 transition-all border-2 border-white/20 font-bold flex items-center gap-2"
              >
                <Youtube className="size-5" />
                Video dari BGN
              </button>
              <button
                onClick={onViewIncidentMap}
                className="px-6 py-3 bg-emerald-700 text-white rounded-xl hover:bg-emerald-800 transition-all border-2 border-white/20 font-bold"
              >
                Lihat Peta Insiden
              </button>
            </div>
          </div>
        </div>
      </section>

      {/* National Statistics */}
      <section className="py-12 bg-white">
        <div className="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8">
          <div className="text-center mb-8">
            <h2 className="text-green-900 mb-2 font-bold text-[32px]">Statistik Nasional</h2>
            <p className="text-gray-600">Data keamanan pangan MBG real-time</p>
          </div>
          <div className="grid grid-cols-2 md:grid-cols-5 gap-6">
            <div className="bg-gradient-to-br from-green-50 to-emerald-50 p-6 rounded-2xl border border-green-200 text-center">
              <div className="text-3xl mb-2">{nationalStats.totalKitchens.toLocaleString()}</div>
              <p className="text-sm text-green-700">Dapur Terdaftar</p>
            </div>
            <div className="bg-gradient-to-br from-blue-50 to-cyan-50 p-6 rounded-2xl border border-blue-200 text-center">
              <div className="text-3xl mb-2">{nationalStats.totalReviews.toLocaleString()}</div>
              <p className="text-sm text-blue-700">Review Terverifikasi</p>
            </div>
            <div className="bg-gradient-to-br from-amber-50 to-yellow-50 p-6 rounded-2xl border border-amber-200 text-center">
              <div className="flex items-center justify-center gap-1 text-3xl mb-2">
                <Star className="size-6 fill-amber-500 text-amber-500" />
                {nationalStats.averageRating}
              </div>
              <p className="text-sm text-amber-700">Rating Rata-rata</p>
            </div>
            <div className="bg-gradient-to-br from-orange-50 to-red-50 p-6 rounded-2xl border border-orange-200 text-center">
              <div className="text-3xl mb-2">{nationalStats.verifiedIncidents}</div>
              <p className="text-sm text-orange-700">Insiden Tercatat</p>
            </div>
            <div className="bg-gradient-to-br from-emerald-50 to-green-50 p-6 rounded-2xl border border-emerald-200 text-center">
              <div className="text-3xl mb-2">{nationalStats.resolvedIncidents}</div>
              <p className="text-sm text-emerald-700">Insiden Diselesaikan</p>
            </div>
          </div>
        </div>
      </section>

      {/* Mitos & Fakta Section (Moved before Reviews) */}
      <section className="py-16 bg-gradient-to-br from-gray-50 to-blue-50">
        <div className="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8">
          <div className="text-center mb-8">
            <div className="inline-flex items-center gap-2 bg-blue-100 px-4 py-2 rounded-xl mb-4">
              <Book className="size-5 text-blue-600" />
              <span className="text-blue-900 font-bold">Seputar HACCP</span>
            </div>
            <h2 className="text-green-900 mb-2">Pahami Keamanan Pangan Berbasis Sains</h2>
            <p className="text-gray-600 max-w-2xl mx-auto">
              Pelajari standar HACCP, miskonsepsi umum, dan cara melaporkan dengan benar
            </p>
          </div>
          <EducationMode />
        </div>
      </section>

      {/* Public Reviews Section */}
      <section id="reviews" className="py-16 bg-gradient-to-br from-gray-50 to-green-50/30">
        <div className="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8">
          <div className="flex items-center justify-between mb-8">
            <div>
              <h2 className="text-green-900 mb-2 font-bold">Review Terverifikasi</h2>
              <p className="text-gray-600">Laporan kualitas makanan yang sudah divalidasi pengawas</p>
            </div>
            <div className="flex items-center gap-2 bg-green-100 px-4 py-2 rounded-xl">
              <Shield className="size-5 text-green-700" />
              <span className="text-sm text-green-800">Semua review diverifikasi</span>
            </div>
          </div>

          <div className="grid md:grid-cols-3 gap-6">
            {publicReviews.map((review) => (
              <div key={review.id} className="bg-white rounded-2xl shadow-lg border border-gray-200 overflow-hidden hover:shadow-xl transition-shadow">
                {/* Image */}
                <div className="relative h-48 bg-gray-100">
                  <img 
                    src={review.photo} 
                    alt="Makanan MBG" 
                    className="w-full h-full object-cover"
                  />
                  {review.verified && (
                    <div className="absolute top-3 right-3 bg-green-600 text-white px-3 py-1 rounded-full text-xs flex items-center gap-1 shadow-lg">
                      <CheckCircle2 className="size-3" />
                      Terverifikasi
                    </div>
                  )}
                </div>

                <div className="p-5 bg-[rgba(132,44,44,0)]">
                  {/* Kitchen Info */}
                  <h3 className="text-green-900 mb-1">{review.kitchenName}</h3>
                  <div className="flex items-center gap-1 text-sm text-gray-600 mb-3">
                    <MapPin className="size-4" />
                    {review.location}
                  </div>

                  {/* Rating */}
                  <div className="flex items-center gap-2 mb-3">
                    <div className="flex items-center gap-1 bg-amber-50 px-3 py-1 rounded-lg">
                      <Star className="size-4 fill-amber-500 text-amber-500" />
                      <span className="text-amber-900">{review.rating}</span>
                    </div>
                    <span className="text-sm text-gray-500">• {review.reviewerType}</span>
                  </div>

                  {/* HACCP Scores */}
                  <div className="grid grid-cols-2 gap-2 mb-4 p-3 bg-green-50 rounded-xl">
                    <div className="text-xs">
                      <span className="text-gray-600">Rasa:</span>
                      <span className="ml-1 text-green-700">{review.haccpScores.taste}</span>
                    </div>
                    <div className="text-xs">
                      <span className="text-gray-600">Kebersihan:</span>
                      <span className="ml-1 text-green-700">{review.haccpScores.hygiene}</span>
                    </div>
                    <div className="text-xs">
                      <span className="text-gray-600">Kesegaran:</span>
                      <span className="ml-1 text-green-700">{review.haccpScores.freshness}</span>
                    </div>
                    <div className="text-xs">
                      <span className="text-gray-600">Suhu:</span>
                      <span className="ml-1 text-green-700">{review.haccpScores.temperature}</span>
                    </div>
                  </div>

                  {/* Comment */}
                  <p className="text-sm text-gray-700 mb-4 line-clamp-3">
                    "{review.comment}"
                  </p>

                  {/* Meta */}
                  <div className="flex items-center justify-between text-xs text-gray-500 pt-3 border-t border-gray-100">
                    <span>📅 {new Date(review.date).toLocaleDateString('id-ID')}</span>
                    <button 
                      onClick={() => onViewKitchenDetail(review.id)}
                      className="flex items-center gap-1 text-green-600 hover:text-green-700"
                    >
                      <Eye className="size-3" />
                      Detail
                    </button>
                  </div>
                </div>
              </div>
            ))}
          </div>
        </div>
      </section>

      {/* Open Data API Section */}
      <section className="py-16 bg-white">
        <div className="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8">
          <div className="text-center mb-8">
            <div className="inline-flex items-center gap-2 bg-indigo-100 px-4 py-2 rounded-xl mb-4">
              <Database className="size-5 text-indigo-600" />
              <span className="text-indigo-900 font-bold">Open Data API</span>
            </div>
            <h2 className="text-green-900 mb-2">Akses Data Publik MBG</h2>
            <p className="text-gray-600 max-w-2xl mx-auto">
              API terbatas untuk peneliti, jurnalis, dan organisasi yang membutuhkan data agregat
            </p>
          </div>
          <OpenDataStats />
        </div>
      </section>

      {/* MBG Watch News Section */}
      <section className="py-16 bg-gradient-to-br from-purple-50 via-white to-indigo-50">
        <div className="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8">
          <div className="text-center mb-8">
            <div className="inline-flex items-center gap-2 bg-purple-100 px-4 py-2 rounded-xl mb-4">
              <Newspaper className="size-5 text-purple-600" />
              <span className="text-purple-900 font-bold">Pemberitaan & Pengaduan</span>
            </div>
            <h2 className="text-green-900 mb-2 font-bold">MBG Watch - Portal Berita & Pengaduan</h2>
            <p className="text-gray-600 max-w-2xl mx-auto">
              Akses portal berita independen dan platform pengaduan publik mengenai program MBG
            </p>
          </div>

          {/* Highlight Cards */}
          <div className="grid md:grid-cols-2 gap-6 mb-8">
            {/* Portal Berita */}
            <div className="bg-white rounded-2xl p-8 shadow-lg border-2 border-purple-200 hover:shadow-xl transition-all">
              <div className="flex items-center gap-3 mb-4">
                <div className="p-3 bg-purple-100 rounded-xl">
                  <Newspaper className="size-8 text-purple-700" />
                </div>
                <div>
                  <h3 className="text-green-900 font-bold text-xl">Portal Berita MBG</h3>
                  <p className="text-sm text-purple-600">Liputan investigatif & transparansi</p>
                </div>
              </div>
              <div className="space-y-3 mb-6">
                <div className="flex items-start gap-2">
                  <CheckCircle2 className="size-5 text-purple-600 mt-0.5 flex-shrink-0" />
                  <p className="text-sm text-gray-700">Berita terkini seputar implementasi program MBG di seluruh Indonesia</p>
                </div>
                <div className="flex items-start gap-2">
                  <CheckCircle2 className="size-5 text-purple-600 mt-0.5 flex-shrink-0" />
                  <p className="text-sm text-gray-700">Investigasi mendalam tentang kualitas dan distribusi makanan</p>
                </div>
                <div className="flex items-start gap-2">
                  <CheckCircle2 className="size-5 text-purple-600 mt-0.5 flex-shrink-0" />
                  <p className="text-sm text-gray-700">Analisis kebijakan dan dampak program terhadap gizi anak</p>
                </div>
              </div>
              <a
                href="https://mbgwatch.org"
                target="_blank"
                rel="noopener noreferrer"
                className="block w-full px-6 py-3 bg-purple-600 text-white rounded-xl hover:bg-purple-700 transition-all text-center font-bold shadow-md"
              >
                Baca Berita MBG Watch
              </a>
            </div>

            {/* Platform Pengaduan */}
            <div className="bg-gradient-to-br from-orange-50 to-red-50 rounded-2xl p-8 shadow-lg border-2 border-orange-300 hover:shadow-xl transition-all">
              <div className="flex items-center gap-3 mb-4">
                <div className="p-3 bg-orange-100 rounded-xl">
                  <Shield className="size-8 text-orange-700" />
                </div>
                <div>
                  <h3 className="text-green-900 font-bold text-xl">Platform Pengaduan</h3>
                  <p className="text-sm text-orange-600">Suara publik untuk MBG lebih baik</p>
                </div>
              </div>
              <div className="space-y-3 mb-6">
                <div className="flex items-start gap-2">
                  <CheckCircle2 className="size-5 text-orange-600 mt-0.5 flex-shrink-0" />
                  <p className="text-sm text-gray-700">Sampaikan pengaduan atau masukan terkait program MBG</p>
                </div>
                <div className="flex items-start gap-2">
                  <CheckCircle2 className="size-5 text-orange-600 mt-0.5 flex-shrink-0" />
                  <p className="text-sm text-gray-700">Monitoring pengaduan publik yang sudah ditindaklanjuti</p>
                </div>
                <div className="flex items-start gap-2">
                  <CheckCircle2 className="size-5 text-orange-600 mt-0.5 flex-shrink-0" />
                  <p className="text-sm text-gray-700">Transparansi penanganan keluhan dan perbaikan sistem</p>
                </div>
              </div>
              <a
                href="https://mbgwatch.org/#pengaduan"
                target="_blank"
                rel="noopener noreferrer"
                className="block w-full px-6 py-3 bg-orange-600 text-white rounded-xl hover:bg-orange-700 transition-all text-center font-bold shadow-md"
              >
                Akses Platform Pengaduan
              </a>
            </div>
          </div>

          {/* Info Note */}
          <div className="bg-gradient-to-r from-indigo-100 to-purple-100 border-2 border-indigo-300 rounded-2xl p-6 text-center">
            <p className="text-gray-700 mb-2">
              <strong>MBG Watch</strong> adalah portal berita independen yang didedikasikan untuk transparansi dan akuntabilitas program Makan Bergizi Gratis
            </p>
            <p className="text-sm text-gray-600">
              Platform ini dikelola oleh jurnalis dan aktivis yang peduli terhadap kualitas gizi dan kesehatan anak Indonesia
            </p>
          </div>
        </div>
      </section>

      {/* Video Education Section */}
      <section id="video-education" className="py-16 bg-gradient-to-br from-red-50 via-white to-orange-50">
        <div className="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8">
          <VideoEducation />
        </div>
      </section>

      {/* CTA Section */}
      <section className="py-16 bg-gradient-to-r from-green-600 to-emerald-700">
        <div className="max-w-4xl mx-auto px-4 sm:px-6 lg:px-8 text-center">
          <h2 className="text-white mb-4">Berpartisipasi dalam Pengawasan</h2>
          <p className="text-green-100 mb-8 text-lg">
            Jika Anda menemukan masalah keamanan pangan, laporkan melalui sistem kami. 
            Login dengan kode unik untuk mengakses formulir pelaporan yang aman dan terverifikasi. Identitas pelapor akan dirahasiakan.
          </p>
          <button
            onClick={onLoginClick}
            className="px-8 py-4 bg-white text-green-700 rounded-xl hover:bg-green-50 transition-all shadow-xl text-lg inline-flex items-center gap-2"
          >
            <LogIn className="size-6" />
            Login untuk Melaporkan
          </button>
          <p className="text-sm text-green-200 mt-4">
            Yuk, bantu Pemerintah untuk mengimplementasikan MBG lebih baik!
          </p>
        </div>
      </section>

      {/* Footer */}
      <footer className="bg-green-900 text-white py-8">
        <div className="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8 text-center">
          <p className="text-green-200">
            © 2025 MonitorMBG - Dashboard Transparansi Keamanan Pangan Makan Bergizi Gratis
          </p>
          <p className="text-sm text-green-300 mt-2">
            Kelompok 2 Mata Kuliah P4K FKM UI 2025
          </p>
        </div>
      </footer>

      {/* Hotline Button - Floating */}
      <HotlineButton />
    </div>
  );
}