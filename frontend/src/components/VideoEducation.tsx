import { Play, Clock, Calendar, Tag, Youtube } from 'lucide-react';
import { useState } from 'react';
import bgnLogo from 'figma:asset/2f10d5e30854bd208367bc1c44ae5a984bc1712e.png';

interface VideoData {
  id: string;
  youtubeId: string;
  title: string;
  description: string;
  category: string;
  duration: string;
  uploadDate: string;
  thumbnail: string;
  haccpRelevance: string;
}

export function VideoEducation() {
  const [loadedVideos, setLoadedVideos] = useState<Set<string>>(new Set());
  const [showAll, setShowAll] = useState(false);

  // Curated videos from Badan Gizi Nasional YouTube channel - BGN Talk Series
  const videos: VideoData[] = [
    {
      id: '1',
      youtubeId: 'v0GI7WVfKwA',
      title: 'BGN Talk: Gizi dan Kesehatan Anak Indonesia',
      description: 'Diskusi mendalam tentang pentingnya gizi seimbang untuk kesehatan anak Indonesia. Video ini membahas strategi pemerintah dalam meningkatkan status gizi anak melalui program-program intervensi gizi termasuk Makan Bergizi Gratis.',
      category: 'BGN Talk',
      duration: '45:20',
      uploadDate: '2024-11-15',
      thumbnail: bgnLogo,
      haccpRelevance: 'Overview Program Gizi Nasional'
    },
    {
      id: '2',
      youtubeId: 'D9l7UpDzfgE',
      title: 'BGN Talk: Keamanan Pangan untuk Program MBG',
      description: 'Pembahasan komprehensif tentang standar keamanan pangan yang harus dipenuhi dalam program Makan Bergizi Gratis. Meliputi prinsip HACCP, kontrol kualitas, dan monitoring keamanan pangan di seluruh rantai distribusi.',
      category: 'BGN Talk',
      duration: '42:15',
      uploadDate: '2024-10-28',
      thumbnail: bgnLogo,
      haccpRelevance: 'Standar HACCP dan Keamanan Pangan'
    },
    {
      id: '3',
      youtubeId: 'MsvwTVvtljY',
      title: 'BGN Talk: Pencegahan Stunting Melalui Gizi Seimbang',
      description: 'Diskusi ahli gizi tentang strategi pencegahan stunting pada anak. Video menjelaskan hubungan antara asupan gizi yang cukup, keamanan pangan, dan pencegahan stunting untuk generasi Indonesia yang lebih sehat.',
      category: 'BGN Talk',
      duration: '38:50',
      uploadDate: '2024-10-12',
      thumbnail: bgnLogo,
      haccpRelevance: 'Dampak Gizi terhadap Kesehatan'
    },
    {
      id: '4',
      youtubeId: 'YlmxK_ik5k4',
      title: 'BGN Talk: Implementasi Program Makan Bergizi Gratis',
      description: 'Penjelasan detail tentang implementasi program Makan Bergizi Gratis di Indonesia. Membahas SOP, standar nutrisi, kontrol kualitas makanan, dan mekanisme distribusi yang aman dan efisien ke seluruh sekolah penerima.',
      category: 'BGN Talk',
      duration: '50:30',
      uploadDate: '2024-09-25',
      thumbnail: bgnLogo,
      haccpRelevance: 'SOP dan Implementasi MBG'
    },
    {
      id: '5',
      youtubeId: '36kfQyg5Tgo',
      title: 'BGN Talk: Hygiene dan Sanitasi Dapur Produksi',
      description: 'Diskusi tentang pentingnya hygiene dan sanitasi di dapur produksi makanan bergizi gratis. Video mendemonstrasikan best practices dalam menjaga kebersihan area produksi, peralatan, dan personal hygiene petugas dapur.',
      category: 'BGN Talk',
      duration: '40:15',
      uploadDate: '2024-09-10',
      thumbnail: bgnLogo,
      haccpRelevance: 'CCP 1: Kontrol Higiene dan Sanitasi'
    },
    {
      id: '6',
      youtubeId: 'Q5ggLfjGzeU',
      title: 'BGN Talk: Kontrol Suhu dan Penyimpanan Makanan',
      description: 'Pembahasan teknis tentang pentingnya kontrol suhu dalam keamanan pangan. Video menjelaskan critical temperature zones, teknik penyimpanan yang benar, dan monitoring suhu untuk mencegah pertumbuhan bakteri berbahaya.',
      category: 'BGN Talk',
      duration: '35:40',
      uploadDate: '2024-08-22',
      thumbnail: bgnLogo,
      haccpRelevance: 'CCP 2: Kontrol Suhu dan Penyimpanan'
    },
    {
      id: '7',
      youtubeId: 'gNp-5EIRiII',
      title: 'BGN Talk: Standar Nutrisi Menu MBG',
      description: 'Diskusi dengan ahli nutrisi tentang standar komposisi menu Makan Bergizi Gratis. Video menjelaskan kebutuhan kalori, protein, vitamin, dan mineral yang harus dipenuhi untuk mendukung tumbuh kembang optimal anak sekolah.',
      category: 'BGN Talk',
      duration: '43:25',
      uploadDate: '2024-08-05',
      thumbnail: bgnLogo,
      haccpRelevance: 'Standar Nutrisi MBG'
    },
    {
      id: '8',
      youtubeId: 'bu3m3ucU91g',
      title: 'BGN Talk: Supply Chain dan Keamanan Pangan',
      description: 'Penjelasan tentang manajemen rantai pasokan bahan makanan untuk program MBG. Membahas pemilihan supplier yang kredibel, kontrol kualitas bahan baku, dan cold chain management untuk menjaga kesegaran dan keamanan bahan makanan.',
      category: 'BGN Talk',
      duration: '47:10',
      uploadDate: '2024-07-18',
      thumbnail: bgnLogo,
      haccpRelevance: 'CCP 3: Supply Chain Management'
    },
    {
      id: '9',
      youtubeId: 'GhUklNdjQk4',
      title: 'BGN Talk: Monitoring dan Evaluasi Program MBG',
      description: 'Diskusi tentang sistem monitoring dan evaluasi program Makan Bergizi Gratis. Video menjelaskan mekanisme pelaporan, audit kualitas, penanganan komplain, dan continuous improvement untuk memastikan program berjalan optimal.',
      category: 'BGN Talk',
      duration: '41:30',
      uploadDate: '2024-07-02',
      thumbnail: bgnLogo,
      haccpRelevance: 'Monitoring dan Evaluasi Program'
    },
    {
      id: '10',
      youtubeId: 'k2QgviTI-Eg',
      title: 'BGN Talk: Pencegahan Kontaminasi Silang',
      description: 'Diskusi tentang pencegahan kontaminasi silang di dapur produksi makanan. Video menjelaskan teknik pemisahan bahan mentah dan matang, penggunaan peralatan yang tepat, dan protokol sanitasi untuk mencegah kontaminasi bakteri.',
      category: 'BGN Talk',
      duration: '39:15',
      uploadDate: '2024-06-20',
      thumbnail: bgnLogo,
      haccpRelevance: 'CCP 4: Pencegahan Kontaminasi'
    },
    {
      id: '11',
      youtubeId: 'FTMMMFhXng0',
      title: 'BGN Talk: Standar Kemasan dan Distribusi',
      description: 'Pembahasan tentang standar kemasan yang aman untuk distribusi makanan MBG. Video menjelaskan material kemasan food-grade, teknik packaging yang menjaga kualitas makanan, dan labeling yang informatif sesuai regulasi.',
      category: 'BGN Talk',
      duration: '36:50',
      uploadDate: '2024-06-05',
      thumbnail: bgnLogo,
      haccpRelevance: 'CCP 5: Kemasan dan Distribusi'
    },
    {
      id: '12',
      youtubeId: '689KKPkBm08',
      title: 'BGN Talk: Peran Protein dalam Tumbuh Kembang Anak',
      description: 'Diskusi dengan ahli nutrisi tentang pentingnya protein dalam tumbuh kembang anak. Video menjelaskan kebutuhan protein harian, sumber protein berkualitas, dan cara mengolah protein agar nutrisinya tetap terjaga.',
      category: 'BGN Talk',
      duration: '44:30',
      uploadDate: '2024-05-22',
      thumbnail: bgnLogo,
      haccpRelevance: 'Komponen Nutrisi Penting'
    },
    {
      id: '13',
      youtubeId: '3KJ3LN0EDtY',
      title: 'BGN Talk: Manajemen Risiko Keamanan Pangan',
      description: 'Pembahasan tentang identifikasi dan manajemen risiko keamanan pangan dalam program MBG. Video menjelaskan hazard analysis, critical control points, dan tindakan korektif untuk mengelola risiko dengan efektif.',
      category: 'BGN Talk',
      duration: '48:20',
      uploadDate: '2024-05-08',
      thumbnail: bgnLogo,
      haccpRelevance: 'Hazard Analysis & Risk Management'
    },
    {
      id: '14',
      youtubeId: '7l7HuA-3-2A',
      title: 'BGN Talk: Mikronutrien untuk Kesehatan Anak',
      description: 'Diskusi tentang pentingnya mikronutrien (vitamin dan mineral) dalam menu MBG. Video menjelaskan fungsi berbagai mikronutrien, sumber makanan yang kaya mikronutrien, dan cara mempertahankan kandungan mikronutrien saat memasak.',
      category: 'BGN Talk',
      duration: '41:40',
      uploadDate: '2024-04-25',
      thumbnail: bgnLogo,
      haccpRelevance: 'Standar Nutrisi Mikronutrien'
    },
    {
      id: '15',
      youtubeId: 'bhuX6P3pUu8',
      title: 'BGN Talk: Traceability dalam Rantai Pasokan MBG',
      description: 'Penjelasan tentang sistem traceability untuk memastikan keamanan dan kualitas bahan makanan. Video membahas pentingnya dokumentasi, tracking bahan baku dari supplier hingga konsumen, dan rapid response saat terjadi insiden.',
      category: 'BGN Talk',
      duration: '37:55',
      uploadDate: '2024-04-10',
      thumbnail: bgnLogo,
      haccpRelevance: 'Traceability & Documentation'
    },
    {
      id: '16',
      youtubeId: 'vhQN4UxIN-U',
      title: 'BGN Talk: Edukasi Gizi untuk Orang Tua',
      description: 'Diskusi tentang pentingnya edukasi gizi untuk orang tua dalam mendukung program MBG. Video menjelaskan cara mengajarkan anak pola makan sehat, mengenali makanan bergizi, dan membangun kebiasaan makan yang baik sejak dini.',
      category: 'BGN Talk',
      duration: '40:25',
      uploadDate: '2024-03-28',
      thumbnail: bgnLogo,
      haccpRelevance: 'Edukasi Gizi dan Pola Makan Sehat'
    },
    {
      id: '17',
      youtubeId: 'jD4J4gc9_4E',
      title: 'BGN Talk: Audit dan Sertifikasi Dapur MBG',
      description: 'Pembahasan tentang proses audit dan sertifikasi dapur produksi program MBG. Video menjelaskan standar yang harus dipenuhi, prosedur audit, dokumentasi yang diperlukan, dan continuous improvement untuk mempertahankan sertifikasi.',
      category: 'BGN Talk',
      duration: '45:10',
      uploadDate: '2024-03-15',
      thumbnail: bgnLogo,
      haccpRelevance: 'Audit, Sertifikasi & Compliance'
    },
    {
      id: '18',
      youtubeId: 'v0GI7WVfKwA',
      title: 'BGN Talk: Kolaborasi Multi-Stakeholder untuk MBG Berkualitas',
      description: 'Diskusi tentang pentingnya kolaborasi antara pemerintah, dapur produksi, supplier, sekolah, dan masyarakat. Video menjelaskan peran masing-masing stakeholder dan bagaimana sinergi dapat meningkatkan kualitas program MBG secara keseluruhan.',
      category: 'BGN Talk',
      duration: '46:35',
      uploadDate: '2024-03-01',
      thumbnail: bgnLogo,
      haccpRelevance: 'Kolaborasi & Ekosistem MBG'
    }
  ];

  // Show only first 6 videos initially
  const displayedVideos = showAll ? videos : videos.slice(0, 6);

  const handleLoadVideo = (videoId: string) => {
    setLoadedVideos(prev => new Set([...prev, videoId]));
  };

  return (
    <div className="space-y-8">
      {/* Header */}
      <div className="text-center">
        <div className="inline-flex items-center gap-2 bg-red-100 px-4 py-2 rounded-xl mb-4">
          <Youtube className="size-5 text-red-600" />
          <span className="text-red-900 font-bold">Materi Video Resmi</span>
        </div>
        <h2 className="text-green-900 mb-2">Edukasi Keamanan Pangan & Gizi</h2>
        <p className="text-gray-600 max-w-2xl mx-auto mb-4">
          Video pembelajaran dari Badan Gizi Nasional tentang standar HACCP dan pengelolaan MBG
        </p>
        <p className="text-sm text-gray-500">
          Sumber: <a href="https://www.youtube.com/@BadanGiziNasional" target="_blank" rel="noopener noreferrer" className="text-red-600 hover:text-red-700 underline">Badan Gizi Nasional – YouTube</a>
        </p>
      </div>

      {/* Video Grid */}
      <div className="grid md:grid-cols-2 lg:grid-cols-3 gap-6">
        {displayedVideos.map((video) => (
          <div key={video.id} className="bg-white rounded-2xl shadow-lg border border-gray-200 overflow-hidden hover:shadow-xl transition-all">
            {/* Video Player / Thumbnail */}
            <div className="relative aspect-video bg-gradient-to-br from-sky-100 to-blue-100">
              {loadedVideos.has(video.id) ? (
                <iframe
                  src={`https://www.youtube-nocookie.com/embed/${video.youtubeId}?rel=0&modestbranding=1`}
                  title={video.title}
                  allow="accelerometer; autoplay; clipboard-write; encrypted-media; gyroscope; picture-in-picture"
                  allowFullScreen
                  loading="lazy"
                  className="w-full h-full"
                />
              ) : (
                <button
                  onClick={() => handleLoadVideo(video.id)}
                  className="w-full h-full relative group"
                >
                  <img
                    src={video.thumbnail}
                    alt={video.title}
                    className="w-full h-full object-contain p-8"
                    loading="lazy"
                  />
                  <div className="absolute inset-0 bg-black/30 group-hover:bg-black/40 transition-all flex items-center justify-center">
                    <div className="bg-red-600 rounded-full p-4 group-hover:scale-110 transition-transform shadow-xl">
                      <Play className="size-8 text-white fill-white" />
                    </div>
                  </div>
                  <div className="absolute bottom-2 right-2 bg-black/80 text-white px-2 py-1 rounded text-xs">
                    {video.duration}
                  </div>
                </button>
              )}
            </div>

            {/* Video Info */}
            <div className="p-5 space-y-4">
              {/* Category & HACCP Badge */}
              <div className="flex items-center gap-2 flex-wrap">
                <span className="inline-flex items-center gap-1 bg-green-100 text-green-800 px-2 py-1 rounded-lg text-xs">
                  <Tag className="size-3" />
                  {video.category}
                </span>
                <span className="text-xs text-emerald-700 bg-emerald-50 px-2 py-1 rounded-lg">
                  {video.haccpRelevance}
                </span>
              </div>

              {/* Title */}
              <h3 className="text-green-900 text-lg leading-snug">{video.title}</h3>

              {/* Fallback Info */}
              {!loadedVideos.has(video.id) && (
                <div className="bg-blue-50 border border-blue-200 rounded-xl p-3 text-sm">
                  <p className="text-blue-900 mb-2">
                    📡 <span className="italic">Jika koneksi lambat, baca ringkasan berikut:</span>
                  </p>
                  <p className="text-gray-700 text-sm leading-relaxed">
                    {video.description}
                  </p>
                </div>
              )}

              {/* Description (shown after load) */}
              {loadedVideos.has(video.id) && (
                <p className="text-sm text-gray-700 leading-relaxed">
                  {video.description}
                </p>
              )}

              {/* Metadata */}
              <div className="flex items-center gap-4 text-xs text-gray-500 pt-3 border-t border-gray-100">
                <div className="flex items-center gap-1">
                  <Calendar className="size-3" />
                  {new Date(video.uploadDate).toLocaleDateString('id-ID', { 
                    day: 'numeric', 
                    month: 'short', 
                    year: 'numeric' 
                  })}
                </div>
                <div className="flex items-center gap-1">
                  <Clock className="size-3" />
                  {video.duration}
                </div>
              </div>
            </div>
          </div>
        ))}
      </div>

      {/* Load More Button */}
      {!showAll && videos.length > 6 && (
        <div className="text-center">
          <button
            onClick={() => setShowAll(true)}
            className="inline-flex items-center gap-2 px-8 py-4 bg-gradient-to-r from-red-600 to-red-700 text-white rounded-xl hover:from-red-700 hover:to-red-800 transition-all shadow-lg hover:shadow-xl font-[Abril_Fatface]"
          >
            <Youtube className="size-5" />
            Lihat Selengkapnya ({videos.length - 6} video lainnya)
          </button>
          <p className="text-sm text-gray-500 mt-3">
            Total {videos.length} video BGN Talk tersedia
          </p>
        </div>
      )}

      {/* Show Less Button */}
      {showAll && (
        <div className="text-center">
          <button
            onClick={() => {
              setShowAll(false);
              // Scroll to video section
              document.getElementById('video-education')?.scrollIntoView({ behavior: 'smooth' });
            }}
            className="inline-flex items-center gap-2 px-8 py-4 bg-gray-600 text-white rounded-xl hover:bg-gray-700 transition-all shadow-lg font-[Abril_Fatface]"
          >
            Tampilkan Lebih Sedikit
          </button>
        </div>
      )}

      {/* Footer Note */}
      <div className="bg-gradient-to-r from-red-50 to-pink-50 border border-red-200 rounded-2xl p-6 text-center">
        <Youtube className="size-8 text-red-600 mx-auto mb-3" />
        <p className="text-gray-700 mb-2">
          Semua video bersumber dari channel resmi <strong>Badan Gizi Nasional</strong>
        </p>
        <p className="text-sm text-gray-600">
          Video ditampilkan menggunakan YouTube Embed sesuai Terms of Service. Untuk menonton lebih banyak konten edukasi, kunjungi channel resmi di YouTube.
        </p>
        <a
          href="https://www.youtube.com/@BadanGiziNasional/videos"
          target="_blank"
          rel="noopener noreferrer"
          className="inline-flex items-center gap-2 mt-4 px-6 py-3 bg-red-600 text-white rounded-xl hover:bg-red-700 transition-all shadow-lg font-[Abril_Fatface]"
        >
          <Youtube className="size-5" />
          Kunjungi Channel BGN
        </a>
      </div>
    </div>
  );
}