import { useState, useRef } from 'react';
import { ArrowLeft, Camera, Phone, IdCard, Shield, AlertCircle, CheckCircle2, Upload } from 'lucide-react';
import logoMBG from 'figma:asset/51e94428001de32b8adf84d690546c9b5cfc362e.png';

interface RegistrationPageProps {
  onBack: () => void;
  onComplete: (data: any) => void;
}

export function RegistrationPage({ onBack, onComplete }: RegistrationPageProps) {
  const [step, setStep] = useState(1);
  const [formData, setFormData] = useState({
    uniqueCode: '',
    institutionName: '',
    name: '',
    phone: '',
    ktpPhoto: '',
    otpCode: '',
    consentGiven: false
  });
  const [errors, setErrors] = useState<Record<string, string>>({});
  const [cameraActive, setCameraActive] = useState(false);
  const [otpSent, setOtpSent] = useState(false);
  const videoRef = useRef<HTMLVideoElement>(null);
  const canvasRef = useRef<HTMLCanvasElement>(null);

  const validateStep = (stepNumber: number): boolean => {
    const newErrors: Record<string, string> = {};

    if (stepNumber === 1) {
      if (formData.uniqueCode.length < 8) {
        newErrors.uniqueCode = 'Kode unik harus minimal 8 karakter';
      }
      if (!formData.institutionName.trim()) {
        newErrors.institutionName = 'Nama institusi wajib diisi';
      }
    }

    if (stepNumber === 2) {
      if (!formData.name.trim()) {
        newErrors.name = 'Nama lengkap wajib diisi';
      }
      if (!/^(\+62|62|0)[0-9]{9,12}$/.test(formData.phone)) {
        newErrors.phone = 'Nomor WhatsApp tidak valid';
      }
    }

    if (stepNumber === 3) {
      if (!formData.ktpPhoto) {
        newErrors.ktpPhoto = 'Foto KTP wajib diupload';
      }
    }

    if (stepNumber === 4) {
      if (!formData.consentGiven) {
        newErrors.consent = 'Anda harus menyetujui pemrosesan data';
      }
      if (!otpSent || !formData.otpCode || formData.otpCode.length !== 6) {
        newErrors.otp = 'Kode OTP harus 6 digit';
      }
    }

    setErrors(newErrors);
    return Object.keys(newErrors).length === 0;
  };

  const handleNext = () => {
    if (validateStep(step)) {
      setStep(step + 1);
    }
  };

  const startCamera = async () => {
    try {
      const stream = await navigator.mediaDevices.getUserMedia({ 
        video: { facingMode: 'environment' } 
      });
      if (videoRef.current) {
        videoRef.current.srcObject = stream;
        setCameraActive(true);
      }
    } catch (err) {
      setErrors({ camera: 'Tidak dapat mengakses kamera. Pastikan izin kamera diberikan.' });
    }
  };

  const capturePhoto = () => {
    if (videoRef.current && canvasRef.current) {
      const context = canvasRef.current.getContext('2d');
      if (context) {
        canvasRef.current.width = videoRef.current.videoWidth;
        canvasRef.current.height = videoRef.current.videoHeight;
        context.drawImage(videoRef.current, 0, 0);
        
        const imageData = canvasRef.current.toDataURL('image/jpeg');
        setFormData({ ...formData, ktpPhoto: imageData });
        
        // Stop camera
        const stream = videoRef.current.srcObject as MediaStream;
        stream?.getTracks().forEach(track => track.stop());
        setCameraActive(false);
      }
    }
  };

  const sendOTP = () => {
    // Simulate sending OTP
    setOtpSent(true);
    // In real app, this would call an API to send OTP to the phone number
  };

  const handleSubmit = () => {
    if (validateStep(4)) {
      onComplete(formData);
    }
  };

  const getRoleFromCode = (code: string): string => {
    if (code.startsWith('KTCH')) return 'Dapur';
    if (code.startsWith('SUPL')) return 'Supplier';
    if (code.startsWith('SCHL')) return 'Sekolah';
    return 'Unknown';
  };

  return (
    <div className="min-h-screen bg-gradient-to-br from-emerald-50 via-white to-teal-50 p-4">
      <div className="max-w-2xl mx-auto py-8">
        {/* Header */}
        <div className="flex items-center gap-4 mb-8">
          <button
            onClick={onBack}
            className="p-2 hover:bg-emerald-100 rounded-xl transition-colors"
          >
            <ArrowLeft className="size-6 text-emerald-600" />
          </button>
          <div className="flex items-center gap-3">
            <img src={logoMBG} alt="Logo MBG" className="w-16 h-16" />
            <div>
              <h1 className="text-emerald-900">Registrasi Akun</h1>
              <p className="text-sm text-emerald-600">Langkah {step} dari 4</p>
            </div>
          </div>
        </div>

        {/* Progress Bar */}
        <div className="mb-8">
          <div className="flex gap-2">
            {[1, 2, 3, 4].map((s) => (
              <div
                key={s}
                className={`flex-1 h-2 rounded-full transition-colors ${
                  s <= step ? 'bg-emerald-600' : 'bg-emerald-200'
                }`}
              />
            ))}
          </div>
        </div>

        {/* Step Content */}
        <div className="bg-white rounded-3xl shadow-xl border border-emerald-100 p-8">
          {/* Step 1: Kode Unik */}
          {step === 1 && (
            <div className="space-y-6">
              <div className="flex items-center gap-3 mb-6">
                <div className="p-3 bg-emerald-100 rounded-xl">
                  <Shield className="size-6 text-emerald-600" />
                </div>
                <div>
                  <h2 className="text-emerald-900">Verifikasi Kode Unik</h2>
                  <p className="text-sm text-emerald-600">Masukkan kode unik institusi Anda</p>
                </div>
              </div>

              <div>
                <label className="block text-emerald-900 mb-2">
                  Kode Unik Institusi *
                </label>
                <input
                  type="text"
                  value={formData.uniqueCode}
                  onChange={(e) => setFormData({ ...formData, uniqueCode: e.target.value.toUpperCase() })}
                  className="w-full px-4 py-3 border-2 border-emerald-200 rounded-xl focus:outline-none focus:ring-2 focus:ring-emerald-500 focus:border-transparent"
                  placeholder="KTCH-XXXX-XXXX"
                />
                {errors.uniqueCode && (
                  <p className="text-sm text-red-600 mt-2">{errors.uniqueCode}</p>
                )}
                {formData.uniqueCode.length >= 4 && (
                  <p className="text-sm text-emerald-600 mt-2">
                    Tipe: {getRoleFromCode(formData.uniqueCode)}
                  </p>
                )}
              </div>

              <div>
                <label className="block text-emerald-900 mb-2">
                  Nama Institusi *
                </label>
                <input
                  type="text"
                  value={formData.institutionName}
                  onChange={(e) => setFormData({ ...formData, institutionName: e.target.value })}
                  className="w-full px-4 py-3 border-2 border-emerald-200 rounded-xl focus:outline-none focus:ring-2 focus:ring-emerald-500 focus:border-transparent"
                  placeholder="Contoh: Dapur Pusat Jakarta Timur"
                />
                {errors.institutionName && (
                  <p className="text-sm text-red-600 mt-2">{errors.institutionName}</p>
                )}
              </div>
            </div>
          )}

          {/* Step 2: Data Penanggung Jawab */}
          {step === 2 && (
            <div className="space-y-6">
              <div className="flex items-center gap-3 mb-6">
                <div className="p-3 bg-emerald-100 rounded-xl">
                  <IdCard className="size-6 text-emerald-600" />
                </div>
                <div>
                  <h2 className="text-emerald-900">Data Penanggung Jawab</h2>
                  <p className="text-sm text-emerald-600">Informasi pribadi yang akan diverifikasi</p>
                </div>
              </div>

              <div>
                <label className="block text-emerald-900 mb-2">
                  Nama Lengkap (sesuai KTP) *
                </label>
                <input
                  type="text"
                  value={formData.name}
                  onChange={(e) => setFormData({ ...formData, name: e.target.value })}
                  className="w-full px-4 py-3 border-2 border-emerald-200 rounded-xl focus:outline-none focus:ring-2 focus:ring-emerald-500 focus:border-transparent"
                  placeholder="Nama lengkap sesuai KTP"
                />
                {errors.name && (
                  <p className="text-sm text-red-600 mt-2">{errors.name}</p>
                )}
              </div>

              <div>
                <label className="block text-emerald-900 mb-2">
                  Nomor WhatsApp Aktif *
                </label>
                <div className="relative">
                  <Phone className="absolute left-4 top-1/2 -translate-y-1/2 size-5 text-emerald-400" />
                  <input
                    type="tel"
                    value={formData.phone}
                    onChange={(e) => setFormData({ ...formData, phone: e.target.value })}
                    className="w-full pl-12 pr-4 py-3 border-2 border-emerald-200 rounded-xl focus:outline-none focus:ring-2 focus:ring-emerald-500 focus:border-transparent"
                    placeholder="08123456789"
                  />
                </div>
                {errors.phone && (
                  <p className="text-sm text-red-600 mt-2">{errors.phone}</p>
                )}
                <p className="text-xs text-emerald-600 mt-2">
                  Nomor ini akan digunakan untuk verifikasi OTP
                </p>
              </div>
            </div>
          )}

          {/* Step 3: Upload KTP */}
          {step === 3 && (
            <div className="space-y-6">
              <div className="flex items-center gap-3 mb-6">
                <div className="p-3 bg-emerald-100 rounded-xl">
                  <Camera className="size-6 text-emerald-600" />
                </div>
                <div>
                  <h2 className="text-emerald-900">Foto KTP</h2>
                  <p className="text-sm text-emerald-600">Ambil foto KTP menggunakan kamera</p>
                </div>
              </div>

              <div className="bg-amber-50 border border-amber-200 rounded-xl p-4">
                <p className="text-sm text-amber-900">
                  <strong>Penting:</strong> Pastikan foto KTP jelas dan terbaca. Nama dan NIK harus terlihat dengan jelas.
                </p>
              </div>

              {!formData.ktpPhoto && !cameraActive && (
                <button
                  onClick={startCamera}
                  className="w-full flex items-center justify-center gap-3 border-2 border-dashed border-emerald-300 rounded-xl py-12 hover:border-emerald-500 hover:bg-emerald-50 transition-colors"
                >
                  <Camera className="size-8 text-emerald-600" />
                  <div className="text-left">
                    <p className="text-emerald-900">Buka Kamera</p>
                    <p className="text-sm text-emerald-600">Ambil foto KTP Anda</p>
                  </div>
                </button>
              )}

              {cameraActive && (
                <div className="space-y-4">
                  <video
                    ref={videoRef}
                    autoPlay
                    playsInline
                    className="w-full rounded-xl border-2 border-emerald-200"
                  />
                  <button
                    onClick={capturePhoto}
                    className="w-full bg-emerald-600 text-white py-3 rounded-xl hover:bg-emerald-700 transition-colors"
                  >
                    Ambil Foto
                  </button>
                </div>
              )}

              {formData.ktpPhoto && (
                <div className="space-y-4">
                  <img
                    src={formData.ktpPhoto}
                    alt="KTP Preview"
                    className="w-full rounded-xl border-2 border-emerald-200"
                  />
                  <button
                    onClick={() => setFormData({ ...formData, ktpPhoto: '' })}
                    className="w-full border-2 border-emerald-600 text-emerald-700 py-3 rounded-xl hover:bg-emerald-50 transition-colors"
                  >
                    Ambil Ulang
                  </button>
                </div>
              )}

              <canvas ref={canvasRef} className="hidden" />

              {errors.camera && (
                <div className="flex items-start gap-2 p-4 bg-red-50 border border-red-200 rounded-xl">
                  <AlertCircle className="size-5 text-red-600 flex-shrink-0 mt-0.5" />
                  <p className="text-sm text-red-700">{errors.camera}</p>
                </div>
              )}

              {errors.ktpPhoto && (
                <p className="text-sm text-red-600">{errors.ktpPhoto}</p>
              )}
            </div>
          )}

          {/* Step 4: Verifikasi & Consent */}
          {step === 4 && (
            <div className="space-y-6">
              <div className="flex items-center gap-3 mb-6">
                <div className="p-3 bg-emerald-100 rounded-xl">
                  <CheckCircle2 className="size-6 text-emerald-600" />
                </div>
                <div>
                  <h2 className="text-emerald-900">Verifikasi & Persetujuan</h2>
                  <p className="text-sm text-emerald-600">Langkah terakhir</p>
                </div>
              </div>

              <div>
                <label className="block text-emerald-900 mb-2">
                  Verifikasi WhatsApp
                </label>
                {!otpSent ? (
                  <button
                    onClick={sendOTP}
                    className="w-full bg-green-600 text-white py-3 rounded-xl hover:bg-green-700 transition-colors flex items-center justify-center gap-2"
                  >
                    <Phone className="size-5" />
                    Kirim Kode OTP ke {formData.phone}
                  </button>
                ) : (
                  <div className="space-y-3">
                    <div className="flex items-start gap-2 p-4 bg-green-50 border border-green-200 rounded-xl">
                      <CheckCircle2 className="size-5 text-green-600 flex-shrink-0 mt-0.5" />
                      <p className="text-sm text-green-700">
                        Kode OTP telah dikirim ke nomor WhatsApp Anda
                      </p>
                    </div>
                    <input
                      type="text"
                      value={formData.otpCode}
                      onChange={(e) => setFormData({ ...formData, otpCode: e.target.value.replace(/\D/g, '').slice(0, 6) })}
                      className="w-full px-4 py-3 border-2 border-emerald-200 rounded-xl focus:outline-none focus:ring-2 focus:ring-emerald-500 focus:border-transparent text-center text-2xl tracking-widest"
                      placeholder="000000"
                      maxLength={6}
                    />
                  </div>
                )}
                {errors.otp && (
                  <p className="text-sm text-red-600 mt-2">{errors.otp}</p>
                )}
              </div>

              <div className="bg-blue-50 border border-blue-200 rounded-xl p-4">
                <h3 className="text-blue-900 mb-3">Persetujuan Pemrosesan Data</h3>
                <ul className="text-sm text-blue-800 space-y-2 mb-4">
                  <li>• Data KTP Anda akan disimpan terenkripsi</li>
                  <li>• Hanya admin berwenang yang dapat mengakses</li>
                  <li>• Data digunakan untuk verifikasi identitas</li>
                  <li>• Foto KTP tidak ditampilkan di antarmuka publik</li>
                  <li>• Kode unik hanya dapat digunakan satu kali</li>
                </ul>
                <label className="flex items-start gap-3 cursor-pointer">
                  <input
                    type="checkbox"
                    checked={formData.consentGiven}
                    onChange={(e) => setFormData({ ...formData, consentGiven: e.target.checked })}
                    className="mt-1 size-5 text-emerald-600 border-emerald-300 rounded focus:ring-emerald-500"
                  />
                  <span className="text-sm text-blue-900">
                    Saya menyetujui pemrosesan data pribadi saya sesuai dengan ketentuan di atas
                  </span>
                </label>
                {errors.consent && (
                  <p className="text-sm text-red-600 mt-2">{errors.consent}</p>
                )}
              </div>
            </div>
          )}

          {/* Navigation Buttons */}
          <div className="flex gap-4 mt-8">
            {step > 1 && (
              <button
                onClick={() => setStep(step - 1)}
                className="flex-1 border-2 border-emerald-600 text-emerald-700 py-3 rounded-xl hover:bg-emerald-50 transition-colors"
              >
                Kembali
              </button>
            )}
            {step < 4 ? (
              <button
                onClick={handleNext}
                className="flex-1 bg-emerald-600 text-white py-3 rounded-xl hover:bg-emerald-700 transition-colors"
              >
                Lanjut
              </button>
            ) : (
              <button
                onClick={handleSubmit}
                className="flex-1 bg-emerald-600 text-white py-3 rounded-xl hover:bg-emerald-700 transition-colors"
              >
                Selesai & Daftar
              </button>
            )}
          </div>
        </div>
      </div>
    </div>
  );
}