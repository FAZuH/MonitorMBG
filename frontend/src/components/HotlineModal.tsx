import { useState } from 'react';
import { X, Phone, Mail, FileText, AlertTriangle, MapPin, Utensils, Clock, Activity, CheckCircle2, Copy } from 'lucide-react';

interface HotlineModalProps {
  onClose: () => void;
}

export function HotlineModal({ onClose }: HotlineModalProps) {
  const [step, setStep] = useState<'disclaimer' | 'form' | 'channels' | 'success'>('disclaimer');
  const [formData, setFormData] = useState({
    location: '',
    kitchenName: '',
    foodType: '',
    consumptionTime: '',
    symptoms: '',
    reporterName: '',
    reporterPhone: ''
  });
  const [ticketNumber, setTicketNumber] = useState('');

  const handleNext = () => {
    if (step === 'disclaimer') {
      setStep('form');
    } else if (step === 'form') {
      // Validate required fields
      if (!formData.location || !formData.foodType || !formData.consumptionTime) {
        alert('Mohon isi lokasi, jenis makanan, dan waktu konsumsi');
        return;
      }
      setStep('channels');
    }
  };

  const generateTicketNumber = () => {
    const timestamp = Date.now().toString(36).toUpperCase();
    const random = Math.random().toString(36).substring(2, 6).toUpperCase();
    return `MBG-${timestamp}-${random}`;
  };

  const handleChannelSelect = (channel: 'phone' | 'email' | 'form') => {
    const ticket = generateTicketNumber();
    setTicketNumber(ticket);

    if (channel === 'phone') {
      // In production, this would log to backend
      console.log('Phone report:', { ...formData, ticket, channel: 'phone' });
      setStep('success');
    } else if (channel === 'email') {
      const subject = `[${ticket}] Laporan Insiden MBG - ${formData.location}`;
      const body = `
Nomor Tiket: ${ticket}

Lokasi Dapur/Sekolah: ${formData.location}
Nama Dapur: ${formData.kitchenName}
Jenis Makanan: ${formData.foodType}
Waktu Konsumsi: ${formData.consumptionTime}
Gejala: ${formData.symptoms || 'Tidak ada'}

Nama Pelapor: ${formData.reporterName || 'Anonim'}
Telepon: ${formData.reporterPhone || 'Tidak disediakan'}
      `.trim();
      
      window.location.href = `mailto:pengaduan.mbg@example.com?subject=${encodeURIComponent(subject)}&body=${encodeURIComponent(body)}`;
      setStep('success');
    } else if (channel === 'form') {
      // In production, this would open pre-filled Google Form
      const googleFormUrl = `https://docs.google.com/forms/d/e/EXAMPLE_FORM_ID/viewform?entry.ticket=${ticket}&entry.location=${encodeURIComponent(formData.location)}&entry.food=${encodeURIComponent(formData.foodType)}`;
      window.open(googleFormUrl, '_blank');
      setStep('success');
    }
  };

  const copyTicketNumber = () => {
    navigator.clipboard.writeText(ticketNumber);
    alert('Nomor tiket berhasil disalin!');
  };

  const getSeverityColor = () => {
    if (formData.symptoms.toLowerCase().includes('keracunan') || 
        formData.symptoms.toLowerCase().includes('muntah') ||
        formData.symptoms.toLowerCase().includes('diare')) {
      return { bg: 'bg-red-100', text: 'text-red-900', border: 'border-red-300', label: 'KRITIS' };
    } else if (formData.symptoms.toLowerCase().includes('mual') ||
               formData.symptoms.toLowerCase().includes('sakit perut') ||
               formData.foodType.toLowerCase().includes('abnormal')) {
      return { bg: 'bg-orange-100', text: 'text-orange-900', border: 'border-orange-300', label: 'SEDANG' };
    }
    return { bg: 'bg-yellow-100', text: 'text-yellow-900', border: 'border-yellow-300', label: 'MINOR' };
  };

  return (
    <div className="fixed inset-0 bg-black/60 backdrop-blur-sm z-50 flex items-center justify-center p-4 animate-in fade-in duration-200">
      <div className="bg-white rounded-2xl shadow-2xl max-w-2xl w-full max-h-[90vh] overflow-y-auto">
        {/* Header */}
        <div className="sticky top-0 bg-gradient-to-r from-red-500 to-red-600 text-white p-6 rounded-t-2xl">
          <div className="flex items-center justify-between">
            <div className="flex items-center gap-3">
              <div className="p-2 bg-white/20 rounded-xl">
                <AlertTriangle className="size-6" />
              </div>
              <div>
                <h2 className="text-white">Hotline Insiden MBG</h2>
                <p className="text-sm text-red-100">Layanan Pelaporan Darurat Keamanan Pangan</p>
              </div>
            </div>
            <button
              onClick={onClose}
              className="p-2 hover:bg-white/20 rounded-lg transition-colors"
            >
              <X className="size-6" />
            </button>
          </div>
        </div>

        {/* Content */}
        <div className="p-6">
          {/* Disclaimer Step */}
          {step === 'disclaimer' && (
            <div className="space-y-6">
              <div className="bg-amber-50 border border-amber-200 rounded-xl p-4">
                <div className="flex gap-3">
                  <AlertTriangle className="size-5 text-amber-600 flex-shrink-0 mt-0.5" />
                  <div>
                    <h3 className="text-amber-900 mb-2">Perhatian</h3>
                    <p className="text-sm text-amber-800 leading-relaxed">
                      Gunakan hotline ini <strong>hanya untuk laporan terkait keamanan pangan Program Makan Bergizi Gratis</strong>. 
                      Laporan yang tidak relevan dapat menghambat penanganan insiden sebenarnya.
                    </p>
                  </div>
                </div>
              </div>

              <div className="space-y-4">
                <h3 className="text-gray-900">Kapan Anda Harus Melaporkan?</h3>
                
                <div className="space-y-3">
                  <div className="flex items-start gap-3 p-4 bg-red-50 border border-red-200 rounded-xl">
                    <div className="p-2 bg-red-100 rounded-lg">
                      <AlertTriangle className="size-5 text-red-600" />
                    </div>
                    <div>
                      <h4 className="text-red-900">KRITIS - Segera Lapor</h4>
                      <ul className="text-sm text-red-800 mt-2 space-y-1 list-disc list-inside">
                        <li>Keracunan makanan massal</li>
                        <li>Suhu makanan tidak sesuai standar</li>
                        <li>Kontaminasi bahan berbahaya</li>
                        <li>Gejala serius (muntah, diare berat, demam tinggi)</li>
                      </ul>
                    </div>
                  </div>

                  <div className="flex items-start gap-3 p-4 bg-orange-50 border border-orange-200 rounded-xl">
                    <div className="p-2 bg-orange-100 rounded-lg">
                      <AlertTriangle className="size-5 text-orange-600" />
                    </div>
                    <div>
                      <h4 className="text-orange-900">SEDANG - Perlu Pelaporan</h4>
                      <ul className="text-sm text-orange-800 mt-2 space-y-1 list-disc list-inside">
                        <li>Rasa atau bau makanan abnormal</li>
                        <li>Kemasan rusak atau bocor</li>
                        <li>Kebersihan kurang memadai</li>
                        <li>Bahan makanan tidak segar</li>
                      </ul>
                    </div>
                  </div>

                  <div className="flex items-start gap-3 p-4 bg-yellow-50 border border-yellow-200 rounded-xl">
                    <div className="p-2 bg-yellow-100 rounded-lg">
                      <AlertTriangle className="size-5 text-yellow-600" />
                    </div>
                    <div>
                      <h4 className="text-yellow-900">MINOR - Dokumentasi</h4>
                      <ul className="text-sm text-yellow-800 mt-2 space-y-1 list-disc list-inside">
                        <li>Keterlambatan distribusi</li>
                        <li>Porsi kurang sesuai</li>
                        <li>Variasi menu kurang</li>
                      </ul>
                    </div>
                  </div>
                </div>
              </div>

              <button
                onClick={handleNext}
                className="w-full py-4 bg-red-600 text-white rounded-xl hover:bg-red-700 transition-colors flex items-center justify-center gap-2"
              >
                Saya Mengerti, Lanjutkan Laporan
              </button>
            </div>
          )}

          {/* Form Step */}
          {step === 'form' && (
            <div className="space-y-6">
              <p className="text-gray-600">
                Mohon isi informasi berikut untuk membantu tim kami merespons dengan cepat dan tepat.
              </p>

              <div className="space-y-4">
                {/* Location */}
                <div>
                  <label className="flex items-center gap-2 text-gray-900 mb-2">
                    <MapPin className="size-4 text-red-600" />
                    Lokasi Dapur/Sekolah <span className="text-red-600">*</span>
                  </label>
                  <input
                    type="text"
                    value={formData.location}
                    onChange={(e) => setFormData({ ...formData, location: e.target.value })}
                    placeholder="Contoh: SD Negeri 01 Jakarta Pusat"
                    className="w-full px-4 py-3 border border-gray-300 rounded-xl focus:outline-none focus:ring-2 focus:ring-red-500"
                    required
                  />
                </div>

                {/* Kitchen Name */}
                <div>
                  <label className="flex items-center gap-2 text-gray-900 mb-2">
                    <MapPin className="size-4 text-gray-600" />
                    Nama Dapur (jika tahu)
                  </label>
                  <input
                    type="text"
                    value={formData.kitchenName}
                    onChange={(e) => setFormData({ ...formData, kitchenName: e.target.value })}
                    placeholder="Contoh: Dapur Sehat Jakarta Pusat"
                    className="w-full px-4 py-3 border border-gray-300 rounded-xl focus:outline-none focus:ring-2 focus:ring-red-500"
                  />
                </div>

                {/* Food Type */}
                <div>
                  <label className="flex items-center gap-2 text-gray-900 mb-2">
                    <Utensils className="size-4 text-red-600" />
                    Jenis Makanan yang Dikonsumsi <span className="text-red-600">*</span>
                  </label>
                  <input
                    type="text"
                    value={formData.foodType}
                    onChange={(e) => setFormData({ ...formData, foodType: e.target.value })}
                    placeholder="Contoh: Nasi, ayam goreng, sayur sop"
                    className="w-full px-4 py-3 border border-gray-300 rounded-xl focus:outline-none focus:ring-2 focus:ring-red-500"
                    required
                  />
                </div>

                {/* Consumption Time */}
                <div>
                  <label className="flex items-center gap-2 text-gray-900 mb-2">
                    <Clock className="size-4 text-red-600" />
                    Waktu Konsumsi <span className="text-red-600">*</span>
                  </label>
                  <input
                    type="datetime-local"
                    value={formData.consumptionTime}
                    onChange={(e) => setFormData({ ...formData, consumptionTime: e.target.value })}
                    className="w-full px-4 py-3 border border-gray-300 rounded-xl focus:outline-none focus:ring-2 focus:ring-red-500"
                    required
                  />
                </div>

                {/* Symptoms */}
                <div>
                  <label className="flex items-center gap-2 text-gray-900 mb-2">
                    <Activity className="size-4 text-red-600" />
                    Gejala yang Dialami (jika ada)
                  </label>
                  <textarea
                    value={formData.symptoms}
                    onChange={(e) => setFormData({ ...formData, symptoms: e.target.value })}
                    placeholder="Contoh: Mual, muntah, diare, sakit perut, atau gejala lainnya"
                    rows={4}
                    className="w-full px-4 py-3 border border-gray-300 rounded-xl focus:outline-none focus:ring-2 focus:ring-red-500 resize-none"
                  />
                </div>

                {/* Optional Reporter Info */}
                <div className="bg-blue-50 border border-blue-200 rounded-xl p-4">
                  <h4 className="text-blue-900 mb-3 flex items-center gap-2">
                    <FileText className="size-4" />
                    Informasi Pelapor (Opsional)
                  </h4>
                  <p className="text-sm text-blue-800 mb-4">
                    Anda dapat melaporkan secara anonim, namun informasi kontak membantu kami menghubungi jika diperlukan.
                  </p>
                  
                  <div className="space-y-3">
                    <input
                      type="text"
                      value={formData.reporterName}
                      onChange={(e) => setFormData({ ...formData, reporterName: e.target.value })}
                      placeholder="Nama (opsional)"
                      className="w-full px-4 py-2 border border-blue-300 rounded-lg focus:outline-none focus:ring-2 focus:ring-blue-500 bg-white"
                    />
                    <input
                      type="tel"
                      value={formData.reporterPhone}
                      onChange={(e) => setFormData({ ...formData, reporterPhone: e.target.value })}
                      placeholder="Nomor telepon (opsional)"
                      className="w-full px-4 py-2 border border-blue-300 rounded-lg focus:outline-none focus:ring-2 focus:ring-blue-500 bg-white"
                    />
                  </div>
                </div>
              </div>

              <div className="flex gap-3">
                <button
                  onClick={() => setStep('disclaimer')}
                  className="flex-1 py-3 border border-gray-300 text-gray-700 rounded-xl hover:bg-gray-50 transition-colors"
                >
                  Kembali
                </button>
                <button
                  onClick={handleNext}
                  className="flex-1 py-3 bg-red-600 text-white rounded-xl hover:bg-red-700 transition-colors"
                >
                  Lanjutkan
                </button>
              </div>
            </div>
          )}

          {/* Channels Step */}
          {step === 'channels' && (
            <div className="space-y-6">
              <div>
                <h3 className="text-gray-900 mb-2">Pilih Kanal Pelaporan</h3>
                <p className="text-gray-600">
                  Pilih metode yang paling nyaman untuk Anda melaporkan insiden ini.
                </p>
              </div>

              {/* Severity Badge */}
              {formData.symptoms && (
                <div className={`px-4 py-3 rounded-xl border ${getSeverityColor().bg} ${getSeverityColor().border}`}>
                  <div className="flex items-center gap-2">
                    <AlertTriangle className="size-5" />
                    <div>
                      <span className={`${getSeverityColor().text}`}>
                        Tingkat Prioritas: <strong>{getSeverityColor().label}</strong>
                      </span>
                    </div>
                  </div>
                </div>
              )}

              <div className="space-y-3">
                {/* Phone Channel */}
                <button
                  onClick={() => handleChannelSelect('phone')}
                  className="w-full p-5 border-2 border-gray-200 rounded-xl hover:border-red-500 hover:bg-red-50 transition-all text-left group"
                >
                  <div className="flex items-start gap-4">
                    <div className="p-3 bg-red-100 rounded-xl group-hover:bg-red-200 transition-colors">
                      <Phone className="size-6 text-red-600" />
                    </div>
                    <div className="flex-1">
                      <h4 className="text-gray-900 mb-1">Telepon Hotline</h4>
                      <p className="text-sm text-gray-600 mb-2">
                        Hubungi tim darurat kami untuk respons cepat
                      </p>
                      <p className="text-red-600">📞 0800-1234-5678</p>
                      <p className="text-xs text-gray-500 mt-1">24 jam / 7 hari</p>
                    </div>
                  </div>
                </button>

                {/* Email Channel */}
                <button
                  onClick={() => handleChannelSelect('email')}
                  className="w-full p-5 border-2 border-gray-200 rounded-xl hover:border-red-500 hover:bg-red-50 transition-all text-left group"
                >
                  <div className="flex items-start gap-4">
                    <div className="p-3 bg-blue-100 rounded-xl group-hover:bg-blue-200 transition-colors">
                      <Mail className="size-6 text-blue-600" />
                    </div>
                    <div className="flex-1">
                      <h4 className="text-gray-900 mb-1">Email Resmi Pengaduan</h4>
                      <p className="text-sm text-gray-600 mb-2">
                        Kirim laporan tertulis dengan dokumentasi lengkap
                      </p>
                      <p className="text-blue-600">📧 pengaduan.mbg@example.com</p>
                      <p className="text-xs text-gray-500 mt-1">Respons dalam 2-4 jam</p>
                    </div>
                  </div>
                </button>

                {/* Google Form Channel */}
                <button
                  onClick={() => handleChannelSelect('form')}
                  className="w-full p-5 border-2 border-gray-200 rounded-xl hover:border-red-500 hover:bg-red-50 transition-all text-left group"
                >
                  <div className="flex items-start gap-4">
                    <div className="p-3 bg-green-100 rounded-xl group-hover:bg-green-200 transition-colors">
                      <FileText className="size-6 text-green-600" />
                    </div>
                    <div className="flex-1">
                      <h4 className="text-gray-900 mb-1">Formulir Online</h4>
                      <p className="text-sm text-gray-600 mb-2">
                        Isi formulir Google Form terstruktur
                      </p>
                      <p className="text-green-600">📝 Form pelaporan insiden</p>
                      <p className="text-xs text-gray-500 mt-1">Data otomatis tercatat</p>
                    </div>
                  </div>
                </button>
              </div>

              <button
                onClick={() => setStep('form')}
                className="w-full py-3 border border-gray-300 text-gray-700 rounded-xl hover:bg-gray-50 transition-colors"
              >
                Kembali
              </button>
            </div>
          )}

          {/* Success Step */}
          {step === 'success' && (
            <div className="space-y-6 text-center py-8">
              <div className="w-20 h-20 bg-green-100 rounded-full flex items-center justify-center mx-auto">
                <CheckCircle2 className="size-10 text-green-600" />
              </div>

              <div>
                <h3 className="text-green-900 mb-2">Laporan Berhasil Dicatat!</h3>
                <p className="text-gray-600">
                  Tim kami akan segera menindaklanjuti laporan Anda
                </p>
              </div>

              {/* Ticket Number */}
              <div className="bg-green-50 border-2 border-green-200 rounded-xl p-6">
                <p className="text-sm text-green-800 mb-2">Nomor Tiket Anda:</p>
                <div className="flex items-center justify-center gap-3">
                  <p className="text-2xl text-green-900 tracking-wider">{ticketNumber}</p>
                  <button
                    onClick={copyTicketNumber}
                    className="p-2 hover:bg-green-100 rounded-lg transition-colors"
                    title="Salin nomor tiket"
                  >
                    <Copy className="size-5 text-green-600" />
                  </button>
                </div>
                <p className="text-sm text-green-700 mt-3">
                  Simpan nomor ini untuk melacak status penanganan
                </p>
              </div>

              <div className="bg-blue-50 border border-blue-200 rounded-xl p-4 text-left">
                <h4 className="text-blue-900 mb-2">Langkah Selanjutnya:</h4>
                <ul className="text-sm text-blue-800 space-y-2">
                  <li className="flex items-start gap-2">
                    <span className="text-blue-600 mt-0.5">1.</span>
                    <span>Tim pengawas akan mengklasifikasi tingkat risiko laporan Anda</span>
                  </li>
                  <li className="flex items-start gap-2">
                    <span className="text-blue-600 mt-0.5">2.</span>
                    <span>Investigasi akan dilakukan sesuai tingkat prioritas</span>
                  </li>
                  <li className="flex items-start gap-2">
                    <span className="text-blue-600 mt-0.5">3.</span>
                    <span>Anda dapat memeriksa status dengan nomor tiket di atas</span>
                  </li>
                </ul>
              </div>

              <button
                onClick={onClose}
                className="w-full py-4 bg-red-600 text-white rounded-xl hover:bg-red-700 transition-colors"
              >
                Tutup
              </button>
            </div>
          )}
        </div>
      </div>
    </div>
  );
}
