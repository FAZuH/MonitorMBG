import { useState } from 'react';
import { Shield, Lock, AlertCircle, ArrowLeft } from 'lucide-react';
import logoMBG from 'figma:asset/51e94428001de32b8adf84d690546c9b5cfc362e.png';

interface LoginPageProps {
  onLogin: (uniqueCode: string) => void;
  onRegister: () => void;
  onBackToDashboard?: () => void;
}

export function LoginPage({ onLogin, onRegister, onBackToDashboard }: LoginPageProps) {
  const [uniqueCode, setUniqueCode] = useState('');
  const [error, setError] = useState('');
  const [loading, setLoading] = useState(false);

  const handleSubmit = async (e: React.FormEvent) => {
    e.preventDefault();
    setError('');
    
    if (uniqueCode.length < 8) {
      setError('Kode unik harus minimal 8 karakter');
      return;
    }

    setLoading(true);
    
    // Simulate API call
    setTimeout(() => {
      // For demo purposes, accept codes starting with specific prefixes
      const validPrefixes = ['KTCH', 'SUPL', 'SCHL'];
      const prefix = uniqueCode.substring(0, 4).toUpperCase();
      
      if (validPrefixes.includes(prefix)) {
        onLogin(uniqueCode);
      } else {
        setError('Kode unik tidak valid atau belum terdaftar');
      }
      setLoading(false);
    }, 1000);
  };

  return (
    <div className="min-h-screen bg-gradient-to-br from-emerald-50 via-green-50 to-teal-50 flex items-center justify-center p-4">
      <div className="max-w-md w-full">
        {/* Back to Dashboard Button */}
        {onBackToDashboard && (
          <button
            onClick={onBackToDashboard}
            className="mb-6 flex items-center gap-2 text-emerald-700 hover:text-emerald-900 transition-colors"
          >
            <ArrowLeft className="size-5" />
            <span>Kembali ke Dashboard Publik</span>
          </button>
        )}
        
        {/* Logo and Header */}
        <div className="text-center mb-8">
          <div className="flex justify-center mb-4">
            <img src={logoMBG} alt="Logo MBG" className="w-32 h-32" />
          </div>
          <h1 className="text-emerald-900 mb-2 font-[Aclonica]">MonitorMBG</h1>
          <p className="text-emerald-600">Makan Bergizi Gratis - Platform HACCP</p>
        </div>

        {/* Login Card */}
        <div className="bg-white rounded-3xl shadow-xl border border-emerald-100 p-8">
          <div className="flex items-center gap-3 mb-6">
            <div className="p-3 bg-emerald-100 rounded-xl">
              <Shield className="size-6 text-emerald-600" />
            </div>
            <div>
              <h2 className="text-emerald-900">Login untuk Mengisi Penilaian HACCP</h2>
              <p className="text-sm text-emerald-600">Masukkan kode unik institusi Anda</p>
            </div>
          </div>

          <form onSubmit={handleSubmit} className="space-y-6">
            <div>
              <label className="block text-emerald-900 mb-2">
                Kode Unik Institusi
              </label>
              <div className="relative">
                <Lock className="absolute left-4 top-1/2 -translate-y-1/2 size-5 text-emerald-400" />
                <input
                  type="text"
                  value={uniqueCode}
                  onChange={(e) => setUniqueCode(e.target.value.toUpperCase())}
                  className="w-full pl-12 pr-4 py-3 border-2 border-emerald-200 rounded-xl focus:outline-none focus:ring-2 focus:ring-emerald-500 focus:border-transparent"
                  placeholder="KTCH-XXXX-XXXX"
                  required
                />
              </div>
              <p className="text-xs text-emerald-600 mt-2">
                Format: KTCH (Dapur), SUPL (Supplier), SCHL (Sekolah)
              </p>
            </div>

            {error && (
              <div className="flex items-start gap-2 p-4 bg-red-50 border border-red-200 rounded-xl">
                <AlertCircle className="size-5 text-red-600 flex-shrink-0 mt-0.5" />
                <p className="text-sm text-red-700">{error}</p>
              </div>
            )}

            <button
              type="submit"
              disabled={loading}
              className="w-full bg-emerald-600 text-white py-3 rounded-xl hover:bg-emerald-700 transition-colors disabled:opacity-50 disabled:cursor-not-allowed"
            >
              {loading ? 'Memverifikasi...' : 'Login'}
            </button>
          </form>

          <div className="mt-6 pt-6 border-t border-emerald-100">
            <p className="text-sm text-emerald-600 text-center mb-3">
              Belum memiliki akun?
            </p>
            <button
              onClick={onRegister}
              className="w-full border-2 border-emerald-600 text-emerald-700 py-3 rounded-xl hover:bg-emerald-50 transition-colors"
            >
              Daftar Akun Baru
            </button>
          </div>
        </div>

        {/* Security Notice */}
        <div className="mt-6 p-4 bg-blue-50 border border-blue-200 rounded-xl">
          <p className="text-sm text-blue-900">
            <strong>Keamanan Data:</strong> Setiap login tercatat dalam sistem audit. 
            Kode unik hanya dapat digunakan oleh satu akun terverifikasi.
          </p>
        </div>
      </div>
    </div>
  );
}