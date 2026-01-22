import { ShieldCheck, ShieldAlert, Shield, User, Stethoscope, ClipboardCheck } from 'lucide-react';

interface ConfidenceBadgeProps {
  verificationStatus: 'unverified' | 'in_progress' | 'verified';
  reportSource: 'public' | 'official_inspector' | 'health_worker';
  confidenceLevel: 'low' | 'medium' | 'high';
  compact?: boolean;
}

export function ConfidenceBadge({ verificationStatus, reportSource, confidenceLevel, compact = false }: ConfidenceBadgeProps) {
  const getVerificationConfig = () => {
    switch (verificationStatus) {
      case 'verified':
        return {
          bg: 'bg-green-100',
          text: 'text-green-800',
          border: 'border-green-300',
          icon: ShieldCheck,
          label: 'Terverifikasi'
        };
      case 'in_progress':
        return {
          bg: 'bg-yellow-100',
          text: 'text-yellow-800',
          border: 'border-yellow-300',
          icon: Shield,
          label: 'Dalam Proses'
        };
      case 'unverified':
        return {
          bg: 'bg-gray-100',
          text: 'text-gray-700',
          border: 'border-gray-300',
          icon: ShieldAlert,
          label: 'Belum Diverifikasi'
        };
    }
  };

  const getSourceConfig = () => {
    switch (reportSource) {
      case 'official_inspector':
        return {
          icon: ClipboardCheck,
          label: 'Pengawas Resmi',
          color: 'text-blue-700'
        };
      case 'health_worker':
        return {
          icon: Stethoscope,
          label: 'Tenaga Kesehatan',
          color: 'text-purple-700'
        };
      case 'public':
        return {
          icon: User,
          label: 'Masyarakat',
          color: 'text-gray-700'
        };
    }
  };

  const getConfidenceConfig = () => {
    switch (confidenceLevel) {
      case 'high':
        return {
          bg: 'bg-emerald-500',
          label: 'Tinggi'
        };
      case 'medium':
        return {
          bg: 'bg-yellow-500',
          label: 'Sedang'
        };
      case 'low':
        return {
          bg: 'bg-orange-500',
          label: 'Rendah'
        };
    }
  };

  const verification = getVerificationConfig();
  const source = getSourceConfig();
  const confidence = getConfidenceConfig();
  const VerificationIcon = verification.icon;
  const SourceIcon = source.icon;

  if (compact) {
    return (
      <div className="flex items-center gap-2">
        <div className={`flex items-center gap-1 px-2 py-1 rounded-lg border ${verification.bg} ${verification.border}`}>
          <VerificationIcon className={`size-3 ${verification.text}`} />
          <span className={`text-xs ${verification.text}`}>{verification.label}</span>
        </div>
        <div className={`w-2 h-2 rounded-full ${confidence.bg}`} title={`Kepercayaan: ${confidence.label}`}></div>
      </div>
    );
  }

  return (
    <div className="space-y-2">
      {/* Verification Status */}
      <div className={`flex items-center gap-2 px-3 py-2 rounded-xl border ${verification.bg} ${verification.border}`}>
        <VerificationIcon className={`size-5 ${verification.text}`} />
        <div className="flex-1">
          <p className={`text-sm ${verification.text}`}>Status Verifikasi</p>
          <p className={`text-xs ${verification.text} opacity-75`}>{verification.label}</p>
        </div>
      </div>

      {/* Report Source */}
      <div className="flex items-center gap-2 px-3 py-2 rounded-xl bg-white border border-gray-200">
        <SourceIcon className={`size-5 ${source.color}`} />
        <div className="flex-1">
          <p className="text-sm text-gray-700">Sumber Laporan</p>
          <p className={`text-xs ${source.color}`}>{source.label}</p>
        </div>
      </div>

      {/* Confidence Level */}
      <div className="flex items-center gap-2 px-3 py-2 rounded-xl bg-white border border-gray-200">
        <div className={`w-3 h-3 rounded-full ${confidence.bg}`}></div>
        <div className="flex-1">
          <p className="text-sm text-gray-700">Tingkat Kepercayaan</p>
          <p className="text-xs text-gray-600">{confidence.label}</p>
        </div>
      </div>
    </div>
  );
}
