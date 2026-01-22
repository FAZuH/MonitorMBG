import { Book, Shield, CheckCircle2, AlertTriangle, Thermometer, Package } from 'lucide-react';

export function HACCPEducation() {
  const haccpPrinciples = [
    {
      icon: AlertTriangle,
      title: 'Pencegahan Bahaya',
      description: 'Identifikasi bahaya biologis, kimia, dan fisik dalam produksi makanan',
      color: 'text-red-600 bg-red-100'
    },
    {
      icon: Thermometer,
      title: 'Kontrol Kritis',
      description: 'Monitor suhu, waktu, dan proses dari produksi hingga konsumsi',
      color: 'text-blue-600 bg-blue-100'
    },
    {
      icon: Package,
      title: 'Akuntabilitas',
      description: 'Dokumentasi lengkap untuk kesehatan dan keamanan siswa',
      color: 'text-emerald-600 bg-emerald-100'
    }
  ];

  return (
    <div className="bg-gradient-to-br from-emerald-600 to-teal-600 rounded-3xl p-8 text-white">
      <div className="flex items-start gap-4 mb-6">
        <div className="p-3 bg-white/20 rounded-xl">
          <Book className="size-8 text-white" />
        </div>
        <div className="flex-1">
          <h2 className="text-white mb-2 font-bold">Mengapa HACCP Penting bagi Keamanan Pangan?</h2>
          <p className="text-emerald-100">
            HACCP (Hazard Analysis Critical Control Point) adalah sistem manajemen keamanan pangan 
            yang melindungi kesehatan siswa melalui pencegahan sistematis.
          </p>
        </div>
      </div>

      <div className="grid md:grid-cols-3 gap-6">
        {haccpPrinciples.map((principle, idx) => {
          const Icon = principle.icon;
          return (
            <div
              key={idx}
              className="bg-white/10 backdrop-blur-sm rounded-2xl p-6 border border-white/20"
            >
              <div className={`w-12 h-12 rounded-xl ${principle.color} flex items-center justify-center mb-4`}>
                <Icon className="size-6" />
              </div>
              <h3 className="text-white mb-2 font-bold">{principle.title}</h3>
              <p className="text-sm text-emerald-100">
                {principle.description}
              </p>
            </div>
          );
        })}
      </div>

      <div className="mt-6 bg-white/10 backdrop-blur-sm rounded-2xl p-4 border border-white/20">
        <div className="flex items-center gap-2">
          <Shield className="size-5 text-white" />
          <p className="text-sm text-emerald-100">
            <strong className="text-white">Standar MBG:</strong> Setiap dapur wajib memenuhi 6 kriteria HACCP untuk 
            memastikan makanan aman, bergizi, dan berkualitas tinggi.
          </p>
        </div>
      </div>
    </div>
  );
}
