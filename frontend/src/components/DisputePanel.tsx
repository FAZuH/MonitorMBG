import { useState } from 'react';
import { AlertCircle, FileText, Clock, CheckCircle2, XCircle, MessageSquare } from 'lucide-react';

interface DisputeHistory {
  timestamp: string;
  action: string;
  by: string;
  notes: string;
}

interface DisputePanelProps {
  reviewId: string;
  disputeStatus: 'none' | 'disputed' | 'under_review' | 'resolved';
  disputeHistory: DisputeHistory[];
  onFileDispute?: (reviewId: string, reason: string) => void;
  canFileDispute?: boolean;
}

export function DisputePanel({ 
  reviewId, 
  disputeStatus, 
  disputeHistory, 
  onFileDispute,
  canFileDispute = false 
}: DisputePanelProps) {
  const [showDisputeForm, setShowDisputeForm] = useState(false);
  const [disputeReason, setDisputeReason] = useState('');

  const getStatusConfig = () => {
    switch (disputeStatus) {
      case 'none':
        return {
          icon: CheckCircle2,
          label: 'Tidak Ada Sengketa',
          color: 'text-green-700',
          bg: 'bg-green-100',
          border: 'border-green-300'
        };
      case 'disputed':
        return {
          icon: AlertCircle,
          label: 'Dalam Klarifikasi',
          color: 'text-orange-700',
          bg: 'bg-orange-100',
          border: 'border-orange-300'
        };
      case 'under_review':
        return {
          icon: Clock,
          label: 'Sedang Ditinjau',
          color: 'text-blue-700',
          bg: 'bg-blue-100',
          border: 'border-blue-300'
        };
      case 'resolved':
        return {
          icon: CheckCircle2,
          label: 'Sengketa Selesai',
          color: 'text-green-700',
          bg: 'bg-green-100',
          border: 'border-green-300'
        };
    }
  };

  const handleSubmitDispute = () => {
    if (!disputeReason.trim()) {
      alert('Mohon isi alasan keberatan');
      return;
    }
    
    if (onFileDispute) {
      onFileDispute(reviewId, disputeReason);
    }
    
    setDisputeReason('');
    setShowDisputeForm(false);
  };

  const statusConfig = getStatusConfig();
  const StatusIcon = statusConfig.icon;

  return (
    <div className="bg-white rounded-2xl border border-gray-200 overflow-hidden">
      {/* Header */}
      <div className={`px-6 py-4 ${statusConfig.bg} border-b ${statusConfig.border}`}>
        <div className="flex items-center justify-between">
          <div className="flex items-center gap-3">
            <StatusIcon className={`size-5 ${statusConfig.color}`} />
            <div>
              <h4 className={`${statusConfig.color}`}>Status Sengketa</h4>
              <p className={`text-sm ${statusConfig.color} opacity-75`}>{statusConfig.label}</p>
            </div>
          </div>
          
          {canFileDispute && disputeStatus === 'none' && (
            <button
              onClick={() => setShowDisputeForm(!showDisputeForm)}
              className="px-4 py-2 bg-white border border-gray-300 text-gray-700 rounded-xl hover:bg-gray-50 transition-colors text-sm"
            >
              Ajukan Keberatan
            </button>
          )}
        </div>
      </div>

      {/* Dispute Form */}
      {showDisputeForm && (
        <div className="px-6 py-4 bg-orange-50 border-b border-orange-200">
          <h4 className="text-orange-900 mb-3">Ajukan Keberatan</h4>
          <textarea
            value={disputeReason}
            onChange={(e) => setDisputeReason(e.target.value)}
            placeholder="Jelaskan alasan keberatan Anda dengan detail dan bukti pendukung..."
            rows={4}
            className="w-full px-4 py-3 border border-orange-300 rounded-xl focus:outline-none focus:ring-2 focus:ring-orange-500 bg-white resize-none"
          />
          <div className="flex gap-3 mt-3">
            <button
              onClick={handleSubmitDispute}
              className="px-4 py-2 bg-orange-600 text-white rounded-xl hover:bg-orange-700 transition-colors text-sm"
            >
              Kirim Keberatan
            </button>
            <button
              onClick={() => {
                setShowDisputeForm(false);
                setDisputeReason('');
              }}
              className="px-4 py-2 bg-white border border-gray-300 text-gray-700 rounded-xl hover:bg-gray-50 transition-colors text-sm"
            >
              Batal
            </button>
          </div>
        </div>
      )}

      {/* Dispute History */}
      {disputeHistory && disputeHistory.length > 0 && (
        <div className="px-6 py-4">
          <h4 className="text-gray-900 mb-4 flex items-center gap-2">
            <FileText className="size-5 text-gray-600" />
            Riwayat Klarifikasi
          </h4>
          <div className="space-y-3">
            {disputeHistory.map((entry, idx) => (
              <div
                key={idx}
                className="flex gap-4 pb-3 border-b border-gray-100 last:border-b-0"
              >
                <div className="flex-shrink-0">
                  <div className="w-2 h-2 bg-emerald-500 rounded-full mt-2"></div>
                </div>
                <div className="flex-1 min-w-0">
                  <div className="flex items-start justify-between gap-2 mb-1">
                    <p className="text-sm text-gray-900">{entry.action}</p>
                    <p className="text-xs text-gray-500 flex-shrink-0">
                      {new Date(entry.timestamp).toLocaleString('id-ID', {
                        day: 'numeric',
                        month: 'short',
                        year: 'numeric',
                        hour: '2-digit',
                        minute: '2-digit'
                      })}
                    </p>
                  </div>
                  <p className="text-xs text-gray-600 mb-1">Oleh: {entry.by}</p>
                  {entry.notes && (
                    <p className="text-sm text-gray-700 bg-gray-50 px-3 py-2 rounded-lg mt-2">
                      {entry.notes}
                    </p>
                  )}
                </div>
              </div>
            ))}
          </div>
        </div>
      )}

      {/* Info */}
      <div className="px-6 py-4 bg-blue-50 border-t border-blue-200">
        <div className="flex items-start gap-3">
          <MessageSquare className="size-5 text-blue-600 flex-shrink-0 mt-0.5" />
          <div>
            <h4 className="text-blue-900 mb-1">Mekanisme Klarifikasi Dua Arah</h4>
            <ul className="text-sm text-blue-800 space-y-1">
              <li>• Dapur dapat mengajukan keberatan terhadap review yang dianggap tidak akurat</li>
              <li>• Panel pengawas akan meninjau bukti dari kedua belah pihak</li>
              <li>• Keputusan verifikasi final akan ditampilkan secara transparan</li>
              <li>• Semua riwayat klarifikasi tercatat untuk akuntabilitas</li>
            </ul>
          </div>
        </div>
      </div>
    </div>
  );
}
