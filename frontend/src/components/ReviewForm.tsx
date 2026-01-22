import { useState } from 'react';
import { Review } from '../App';
import { X, Star, Upload, Image as ImageIcon } from 'lucide-react';

interface ReviewFormProps {
  kitchenId: string;
  onSubmit: (review: Omit<Review, 'id' | 'date'>) => void;
  onClose: () => void;
}

export function ReviewForm({ kitchenId, onSubmit, onClose }: ReviewFormProps) {
  const [reviewerName, setReviewerName] = useState('');
  const [reviewerType, setReviewerType] = useState<'consumer' | 'supplier' | 'kitchen'>('consumer');
  const [comment, setComment] = useState('');
  const [photos, setPhotos] = useState<string[]>([]);
  const [ratings, setRatings] = useState({
    taste: 5,
    hygiene: 5,
    freshness: 5,
    temperature: 5,
    packaging: 5,
    handling: 5
  });

  const handlePhotoUpload = (e: React.ChangeEvent<HTMLInputElement>) => {
    const files = e.target.files;
    if (!files) return;

    const newPhotos: string[] = [];
    Array.from(files).forEach((file) => {
      const reader = new FileReader();
      reader.onloadend = () => {
        newPhotos.push(reader.result as string);
        if (newPhotos.length === files.length) {
          setPhotos([...photos, ...newPhotos]);
        }
      };
      reader.readAsDataURL(file);
    });
  };

  const removePhoto = (index: number) => {
    setPhotos(photos.filter((_, i) => i !== index));
  };

  const handleSubmit = (e: React.FormEvent) => {
    e.preventDefault();
    onSubmit({
      kitchenId,
      reviewerName,
      reviewerType,
      ratings,
      comment,
      verified: false,
      photos: photos.length > 0 ? photos : undefined
    });
  };

  const criteria = [
    { key: 'taste' as const, label: 'Taste (Cita Rasa)', description: 'Kualitas rasa makanan' },
    { key: 'hygiene' as const, label: 'Hygiene (Kebersihan)', description: 'Kebersihan dapur dan peralatan' },
    { key: 'freshness' as const, label: 'Freshness (Kesegaran)', description: 'Kesegaran bahan dan makanan' },
    { key: 'temperature' as const, label: 'Temperature Control', description: 'Kontrol suhu penyimpanan dan pengiriman' },
    { key: 'packaging' as const, label: 'Packaging (Kemasan)', description: 'Kualitas dan kebersihan kemasan' },
    { key: 'handling' as const, label: 'Handling (Penanganan)', description: 'Penanganan makanan dan bahan baku' }
  ];

  return (
    <div className="fixed inset-0 bg-black/50 backdrop-blur-sm flex items-center justify-center p-4 z-50">
      <div className="bg-white rounded-3xl max-w-3xl w-full max-h-[90vh] overflow-y-auto">
        <div className="sticky top-0 bg-white border-b border-emerald-100 p-6 flex items-center justify-between">
          <h2 className="text-emerald-900">Add HACCP Review</h2>
          <button
            onClick={onClose}
            className="p-2 hover:bg-emerald-50 rounded-xl transition-colors"
          >
            <X className="size-6 text-emerald-600" />
          </button>
        </div>

        <form onSubmit={handleSubmit} className="p-6 space-y-6">
          {/* Reviewer Info */}
          <div className="space-y-4">
            <div>
              <label className="block text-emerald-900 mb-2">
                Reviewer Name / Organization
              </label>
              <input
                type="text"
                value={reviewerName}
                onChange={(e) => setReviewerName(e.target.value)}
                className="w-full px-4 py-3 border border-emerald-200 rounded-xl focus:outline-none focus:ring-2 focus:ring-emerald-500 focus:border-transparent"
                placeholder="e.g., SD Negeri 01, CV Sayur Segar"
                required
              />
            </div>

            <div>
              <label className="block text-emerald-900 mb-2">
                Reviewer Type
              </label>
              <div className="grid grid-cols-3 gap-4">
                {['consumer', 'supplier', 'kitchen'].map((type) => (
                  <button
                    key={type}
                    type="button"
                    onClick={() => setReviewerType(type as any)}
                    className={`px-4 py-3 rounded-xl border-2 transition-all ${
                      reviewerType === type
                        ? 'border-emerald-500 bg-emerald-50 text-emerald-900'
                        : 'border-emerald-200 text-emerald-600 hover:border-emerald-300'
                    }`}
                  >
                    {type.charAt(0).toUpperCase() + type.slice(1)}
                  </button>
                ))}
              </div>
            </div>
          </div>

          {/* HACCP Ratings */}
          <div className="space-y-6">
            <h3 className="text-emerald-900">HACCP Criteria Ratings</h3>
            {criteria.map((criterion) => (
              <div key={criterion.key} className="bg-emerald-50 rounded-xl p-4">
                <div className="flex items-start justify-between mb-3">
                  <div>
                    <p className="text-emerald-900">{criterion.label}</p>
                    <p className="text-sm text-emerald-600">{criterion.description}</p>
                  </div>
                  <div className="flex items-center gap-1 bg-white px-3 py-1 rounded-lg">
                    <Star className="size-4 fill-amber-400 text-amber-400" />
                    <span className="text-emerald-900">{ratings[criterion.key].toFixed(1)}</span>
                  </div>
                </div>
                <input
                  type="range"
                  min="0"
                  max="5"
                  step="0.1"
                  value={ratings[criterion.key]}
                  onChange={(e) => setRatings({ ...ratings, [criterion.key]: parseFloat(e.target.value) })}
                  className="w-full h-2 bg-emerald-200 rounded-full appearance-none cursor-pointer [&::-webkit-slider-thumb]:appearance-none [&::-webkit-slider-thumb]:w-5 [&::-webkit-slider-thumb]:h-5 [&::-webkit-slider-thumb]:rounded-full [&::-webkit-slider-thumb]:bg-emerald-600 [&::-webkit-slider-thumb]:cursor-pointer"
                />
                <div className="flex justify-between text-xs text-emerald-600 mt-1">
                  <span>0</span>
                  <span>1</span>
                  <span>2</span>
                  <span>3</span>
                  <span>4</span>
                  <span>5</span>
                </div>
              </div>
            ))}
          </div>

          {/* Comment */}
          <div>
            <label className="block text-emerald-900 mb-2">
              Comment / Feedback
            </label>
            <textarea
              value={comment}
              onChange={(e) => setComment(e.target.value)}
              className="w-full px-4 py-3 border border-emerald-200 rounded-xl focus:outline-none focus:ring-2 focus:ring-emerald-500 focus:border-transparent resize-none"
              rows={4}
              placeholder="Share your detailed feedback about the kitchen's performance..."
              required
            />
          </div>

          {/* Photo Upload */}
          <div>
            <label className="block text-emerald-900 mb-2">
              Foto Makanan (Opsional)
            </label>
            <p className="text-sm text-emerald-600 mb-3">
              Upload foto makanan untuk membantu reviewer lain melihat kondisi aktual
            </p>
            
            {/* Upload Button */}
            <label className="flex items-center justify-center gap-2 w-full px-4 py-6 border-2 border-dashed border-emerald-300 rounded-xl hover:border-emerald-500 hover:bg-emerald-50 transition-colors cursor-pointer">
              <input
                type="file"
                accept="image/*"
                multiple
                onChange={handlePhotoUpload}
                className="hidden"
              />
              <Upload className="size-5 text-emerald-600" />
              <span className="text-emerald-700">Click to upload photos (max 5 photos)</span>
            </label>

            {/* Photo Preview */}
            {photos.length > 0 && (
              <div className="grid grid-cols-3 gap-3 mt-4">
                {photos.map((photo, index) => (
                  <div key={index} className="relative group aspect-square rounded-xl overflow-hidden border-2 border-emerald-200">
                    <img
                      src={photo}
                      alt={`Preview ${index + 1}`}
                      className="w-full h-full object-cover"
                    />
                    <button
                      type="button"
                      onClick={() => removePhoto(index)}
                      className="absolute top-2 right-2 p-1 bg-red-500 text-white rounded-full opacity-0 group-hover:opacity-100 transition-opacity"
                    >
                      <X className="size-4" />
                    </button>
                  </div>
                ))}
              </div>
            )}
          </div>

          {/* Submit */}
          <div className="flex gap-4">
            <button
              type="button"
              onClick={onClose}
              className="flex-1 px-6 py-3 border border-emerald-200 text-emerald-700 rounded-xl hover:bg-emerald-50 transition-colors"
            >
              Cancel
            </button>
            <button
              type="submit"
              className="flex-1 px-6 py-3 bg-emerald-600 text-white rounded-xl hover:bg-emerald-700 transition-colors"
            >
              Submit Review
            </button>
          </div>
        </form>
      </div>
    </div>
  );
}
