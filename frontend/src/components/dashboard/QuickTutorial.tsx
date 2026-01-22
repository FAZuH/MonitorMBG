import { useState } from 'react';
import { PlayCircle, X, ChevronRight, Star, Camera, FileText } from 'lucide-react';

interface QuickTutorialProps {
  role: 'kitchen' | 'supplier' | 'school';
}

export function QuickTutorial({ role }: QuickTutorialProps) {
  const [showModal, setShowModal] = useState(false);

  const getTutorialSteps = () => {
    const commonSteps = [
      {
        icon: FileText,
        title: 'Pilih Dapur',
        description: 'Pilih dapur yang ingin Anda review dari daftar'
      },
      {
        icon: Star,
        title: 'Beri Penilaian HACCP',
        description: 'Nilai 6 kriteria: Taste, Hygiene, Freshness, Temperature, Packaging, Handling'
      },
      {
        icon: Camera,
        title: 'Upload Foto Bukti',
        description: 'Ambil foto makanan langsung dari kamera dengan timestamp otomatis'
      }
    ];

    const roleSpecific = {
      kitchen: [
        {
          icon: FileText,
          title: 'Monitor Compliance',
          description: 'Pastikan semua checklist kebersihan terpenuhi setiap hari'
        }
      ],
      supplier: [
        {
          icon: FileText,
          title: 'Catat Pengiriman',
          description: 'Dokumentasikan kondisi bahan saat pengiriman dengan foto'
        }
      ],
      school: [
        {
          icon: FileText,
          title: 'Laporkan Masalah',
          description: 'Segera laporkan jika ada keluhan atau insiden keamanan pangan'
        }
      ]
    };

    return [...commonSteps, ...roleSpecific[role]];
  };

  const steps = getTutorialSteps();

  return (
    <>
      <button
        onClick={() => setShowModal(true)}
        className="flex items-center gap-2 bg-blue-600 text-white px-6 py-3 rounded-xl hover:bg-blue-700 transition-colors shadow-lg"
      >
        <PlayCircle className="size-5" />
        <span>Panduan Penggunaan</span>
      </button>

      {showModal && (
        <div className="fixed inset-0 bg-black/50 backdrop-blur-sm flex items-center justify-center p-4 z-50">
          <div className="bg-white rounded-3xl max-w-2xl w-full max-h-[90vh] overflow-y-auto">
            <div className="sticky top-0 bg-white border-b border-gray-200 p-6 flex items-center justify-between">
              <h2 className="text-gray-900">Panduan Cepat</h2>
              <button
                onClick={() => setShowModal(false)}
                className="p-2 hover:bg-gray-100 rounded-xl transition-colors"
              >
                <X className="size-6 text-gray-600" />
              </button>
            </div>

            <div className="p-6 space-y-4">
              {steps.map((step, idx) => {
                const Icon = step.icon;
                return (
                  <div
                    key={idx}
                    className="flex gap-4 bg-gradient-to-r from-emerald-50 to-teal-50 rounded-2xl p-6 border border-emerald-200"
                  >
                    <div className="flex items-center justify-center size-12 bg-emerald-600 text-white rounded-xl flex-shrink-0">
                      {idx + 1}
                    </div>
                    <div className="flex-1">
                      <div className="flex items-center gap-2 mb-2">
                        <Icon className="size-5 text-emerald-600" />
                        <h3 className="text-emerald-900">{step.title}</h3>
                      </div>
                      <p className="text-emerald-700">{step.description}</p>
                    </div>
                    {idx < steps.length - 1 && (
                      <ChevronRight className="size-6 text-emerald-400 self-center" />
                    )}
                  </div>
                );
              })}

              <div className="bg-amber-50 border border-amber-200 rounded-2xl p-6 mt-6">
                <h3 className="text-amber-900 mb-3">Tips Penilaian yang Baik</h3>
                <ul className="space-y-2 text-sm text-amber-800">
                  <li className="flex gap-2">
                    <span>•</span>
                    <span>Fokus pada aspek keamanan pangan, bukan preferensi pribadi</span>
                  </li>
                  <li className="flex gap-2">
                    <span>•</span>
                    <span>Ambil foto dengan pencahayaan yang cukup dan fokus jelas</span>
                  </li>
                  <li className="flex gap-2">
                    <span>•</span>
                    <span>Berikan komentar yang spesifik dan konstruktif</span>
                  </li>
                  <li className="flex gap-2">
                    <span>•</span>
                    <span>Laporkan segera jika menemukan masalah kritis</span>
                  </li>
                </ul>
              </div>
            </div>
          </div>
        </div>
      )}
    </>
  );
}
