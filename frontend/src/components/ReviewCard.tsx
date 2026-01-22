import { useState } from 'react';
import { Review } from '../App';
import { CheckCircle2, User, Building2, ChefHat, X } from 'lucide-react';
import { ConfidenceBadge } from './ConfidenceBadge';
import { RootCauseTag } from './RootCauseTag';
import { DisputePanel } from './DisputePanel';

interface ReviewCardProps {
  review: Review;
  showFullDetails?: boolean;
}

export function ReviewCard({ review, showFullDetails = false }: ReviewCardProps) {
  const [selectedPhoto, setSelectedPhoto] = useState<string | null>(null);
  const [showDispute, setShowDispute] = useState(false);

  const getReviewerIcon = () => {
    switch (review.reviewerType) {
      case 'consumer':
        return <User className="size-5 text-emerald-600" />;
      case 'supplier':
        return <Building2 className="size-5 text-emerald-600" />;
      case 'kitchen':
        return <ChefHat className="size-5 text-emerald-600" />;
    }
  };

  const getReviewerTypeLabel = () => {
    switch (review.reviewerType) {
      case 'consumer':
        return 'Consumer';
      case 'supplier':
        return 'Supplier';
      case 'kitchen':
        return 'Kitchen Staff';
    }
  };

  const haccpCriteria = [
    { key: 'taste', label: 'Taste (Cita Rasa)', value: review.ratings.taste },
    { key: 'hygiene', label: 'Hygiene (Kebersihan)', value: review.ratings.hygiene },
    { key: 'freshness', label: 'Freshness (Kesegaran)', value: review.ratings.freshness },
    { key: 'temperature', label: 'Temperature Control', value: review.ratings.temperature },
    { key: 'packaging', label: 'Packaging (Kemasan)', value: review.ratings.packaging },
    { key: 'handling', label: 'Handling (Penanganan)', value: review.ratings.handling }
  ];

  return (
    <div className="border border-emerald-100 rounded-2xl p-6 hover:border-emerald-200 transition-colors">
      <div className="flex items-start justify-between mb-4">
        <div className="flex items-center gap-3">
          <div className="p-2 bg-emerald-50 rounded-xl">
            {getReviewerIcon()}
          </div>
          <div>
            <div className="flex items-center gap-2">
              <p className="text-emerald-900">{review.reviewerName}</p>
              {review.verified && (
                <CheckCircle2 className="size-4 text-emerald-600" />
              )}
            </div>
            <p className="text-sm text-emerald-600">{getReviewerTypeLabel()}</p>
          </div>
        </div>
        <p className="text-sm text-emerald-600">
          {new Date(review.date).toLocaleDateString('id-ID', {
            year: 'numeric',
            month: 'long',
            day: 'numeric'
          })}
        </p>
      </div>

      <p className="text-emerald-800 mb-6">{review.comment}</p>

      {/* Photo Gallery */}
      {review.photos && review.photos.length > 0 && (
        <div className="mb-6">
          <p className="text-sm text-emerald-700 mb-3">Foto Makanan:</p>
          <div className="grid grid-cols-4 gap-2">
            {review.photos.map((photo, index) => (
              <button
                key={index}
                onClick={() => setSelectedPhoto(photo)}
                className="aspect-square rounded-xl overflow-hidden border-2 border-emerald-200 hover:border-emerald-400 transition-colors group"
              >
                <img
                  src={photo}
                  alt={`Foto makanan ${index + 1}`}
                  className="w-full h-full object-cover group-hover:scale-110 transition-transform duration-300"
                />
              </button>
            ))}
          </div>
        </div>
      )}

      {/* Photo Modal */}
      {selectedPhoto && (
        <div
          className="fixed inset-0 bg-black/80 backdrop-blur-sm flex items-center justify-center p-4 z-50"
          onClick={() => setSelectedPhoto(null)}
        >
          <div className="relative max-w-4xl w-full">
            <button
              onClick={() => setSelectedPhoto(null)}
              className="absolute -top-12 right-0 p-2 bg-white/10 hover:bg-white/20 rounded-full text-white transition-colors"
            >
              <X className="size-6" />
            </button>
            <img
              src={selectedPhoto}
              alt="Foto makanan"
              className="w-full h-auto rounded-2xl shadow-2xl"
              onClick={(e) => e.stopPropagation()}
            />
          </div>
        </div>
      )}

      <div className="grid grid-cols-2 md:grid-cols-3 gap-4">
        {haccpCriteria.map((criterion) => (
          <div key={criterion.key} className="bg-emerald-50 rounded-xl p-4">
            <p className="text-sm text-emerald-600 mb-2">{criterion.label}</p>
            <div className="flex items-center gap-2">
              <div className="flex-1 h-2 bg-emerald-200 rounded-full overflow-hidden">
                <div
                  className="h-full bg-gradient-to-r from-emerald-500 to-teal-500 rounded-full transition-all duration-500"
                  style={{ width: `${(criterion.value / 5) * 100}%` }}
                />
              </div>
              <span className="text-emerald-900">{criterion.value.toFixed(1)}</span>
            </div>
          </div>
        ))}
      </div>

      {/* Confidence Badge */}
      {(review.verificationStatus || review.reportSource || review.confidenceLevel) && (
        <div className="mt-6">
          <ConfidenceBadge
            verificationStatus={review.verificationStatus}
            reportSource={review.reportSource}
            confidenceLevel={review.confidenceLevel}
            compact={!showFullDetails}
          />
        </div>
      )}

      {/* Root Cause Tags */}
      {review.rootCauses && review.rootCauses.length > 0 && (
        <div className="mt-6">
          <RootCauseTag causes={review.rootCauses} compact={!showFullDetails} />
        </div>
      )}

      {/* Evidence Info */}
      {showFullDetails && review.evidence && (
        <div className="mt-6 bg-gray-50 rounded-xl p-4">
          <h4 className="text-gray-900 mb-3">Bukti Pendukung</h4>
          <div className="grid grid-cols-1 md:grid-cols-2 gap-3 text-sm">
            {review.evidence.menuCode && (
              <div>
                <span className="text-gray-600">Kode Menu:</span>{' '}
                <span className="text-gray-900">{review.evidence.menuCode}</span>
              </div>
            )}
            {review.evidence.schoolLocation && (
              <div>
                <span className="text-gray-600">Lokasi Sekolah:</span>{' '}
                <span className="text-gray-900">{review.evidence.schoolLocation}</span>
              </div>
            )}
            {review.evidence.consumptionTime && (
              <div>
                <span className="text-gray-600">Waktu Konsumsi:</span>{' '}
                <span className="text-gray-900">
                  {new Date(review.evidence.consumptionTime).toLocaleString('id-ID')}
                </span>
              </div>
            )}
            {review.evidence.photoTimestamp && (
              <div>
                <span className="text-gray-600">Foto Diambil:</span>{' '}
                <span className="text-gray-900">
                  {new Date(review.evidence.photoTimestamp).toLocaleString('id-ID')}
                </span>
              </div>
            )}
            {review.evidence.symptoms && review.evidence.symptoms.length > 0 && (
              <div className="md:col-span-2">
                <span className="text-gray-600">Gejala:</span>{' '}
                <span className="text-gray-900">{review.evidence.symptoms.join(', ')}</span>
              </div>
            )}
          </div>
        </div>
      )}

      {/* Dispute Status Badge */}
      {review.disputeStatus && review.disputeStatus !== 'none' && (
        <div className="mt-4">
          <div className="inline-flex items-center gap-2 px-3 py-2 bg-orange-100 border border-orange-300 rounded-lg">
            <span className="text-sm text-orange-800">
              {review.disputeStatus === 'disputed' && '⚠️ Dalam Klarifikasi'}
              {review.disputeStatus === 'under_review' && '🔍 Sedang Ditinjau'}
              {review.disputeStatus === 'resolved' && '✅ Sengketa Selesai'}
            </span>
          </div>
        </div>
      )}

      {/* Dispute Panel Toggle */}
      {showFullDetails && review.disputeStatus && review.disputeHistory && (
        <div className="mt-6">
          {!showDispute ? (
            <button
              onClick={() => setShowDispute(true)}
              className="text-sm text-emerald-600 hover:text-emerald-700 underline"
            >
              Lihat Detail Klarifikasi
            </button>
          ) : (
            <DisputePanel
              reviewId={review.id}
              disputeStatus={review.disputeStatus}
              disputeHistory={review.disputeHistory}
              canFileDispute={false}
            />
          )}
        </div>
      )}
    </div>
  );
}