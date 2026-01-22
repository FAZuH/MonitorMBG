import { useState } from 'react';
import { Kitchen } from '../App';
import { ArrowLeft, MapPin, Utensils, Award, Plus, CheckCircle2, ImageIcon } from 'lucide-react';
import { ReviewCard } from './ReviewCard';
import { HACCPChart } from './HACCPChart';
import { ImageWithFallback } from './figma/ImageWithFallback';
import { TrendAnalysis } from './TrendAnalysis';
import { PerformanceBadges } from './PerformanceBadges';

interface KitchenDetailProps {
  kitchen: Kitchen;
  onBack: () => void;
  onAddReview: () => void;
}

export function KitchenDetail({ kitchen, onBack, onAddReview }: KitchenDetailProps) {
  const [showPhotoGallery, setShowPhotoGallery] = useState(false);

  const getAllPhotos = () => {
    const photos: Array<{ url: string; reviewerName: string; date: string }> = [];
    if (kitchen.reviews && Array.isArray(kitchen.reviews)) {
      kitchen.reviews.forEach(review => {
        if (review.photos) {
          review.photos.forEach(photo => {
            photos.push({
              url: photo,
              reviewerName: review.reviewerName,
              date: review.date
            });
          });
        }
      });
    }
    return photos;
  };

  const calculateAverageHACCP = () => {
    if (!kitchen.reviews || kitchen.reviews.length === 0) {
      return {
        taste: 0,
        hygiene: 0,
        freshness: 0,
        temperature: 0,
        packaging: 0,
        handling: 0
      };
    }

    const totals = kitchen.reviews.reduce((acc, review) => ({
      taste: acc.taste + review.ratings.taste,
      hygiene: acc.hygiene + review.ratings.hygiene,
      freshness: acc.freshness + review.ratings.freshness,
      temperature: acc.temperature + review.ratings.temperature,
      packaging: acc.packaging + review.ratings.packaging,
      handling: acc.handling + review.ratings.handling
    }), { taste: 0, hygiene: 0, freshness: 0, temperature: 0, packaging: 0, handling: 0 });

    return {
      taste: totals.taste / kitchen.reviews.length,
      hygiene: totals.hygiene / kitchen.reviews.length,
      freshness: totals.freshness / kitchen.reviews.length,
      temperature: totals.temperature / kitchen.reviews.length,
      packaging: totals.packaging / kitchen.reviews.length,
      handling: totals.handling / kitchen.reviews.length
    };
  };

  const averageRatings = calculateAverageHACCP();
  const allPhotos = getAllPhotos();

  return (
    <div>
      {/* Back Button */}
      <button
        onClick={onBack}
        className="flex items-center gap-2 text-emerald-600 hover:text-emerald-700 mb-6 group"
      >
        <ArrowLeft className="size-5 group-hover:-translate-x-1 transition-transform" />
        <span>Kembali ke Daftar Dapur</span>
      </button>

      {/* Kitchen Header */}
      <div className="bg-white rounded-3xl overflow-hidden border border-emerald-100 mb-8">
        <div className="relative h-80 overflow-hidden bg-emerald-100">
          <ImageWithFallback
            src={`https://source.unsplash.com/1200x800/?${encodeURIComponent(kitchen.image)}`}
            alt={kitchen.name}
            className="w-full h-full object-cover"
          />
          <div className="absolute inset-0 bg-gradient-to-t from-black/60 to-transparent" />
          <div className="absolute bottom-0 left-0 right-0 p-8 text-white">
            <div className="flex items-center gap-2 mb-2">
              <div className="bg-emerald-500 px-3 py-1 rounded-full text-sm">
                {kitchen.type}
              </div>
            </div>
            <h1 className="mb-2">{kitchen.name}</h1>
            <div className="flex items-center gap-2 text-emerald-100">
              <MapPin className="size-5" />
              <span>{kitchen.location}</span>
            </div>
          </div>
        </div>

        <div className="p-8">
          <div className="grid grid-cols-2 md:grid-cols-4 gap-6 mb-8">
            <div className="text-center">
              <div className="flex items-center justify-center gap-2 mb-2">
                <Utensils className="size-6 text-emerald-600" />
              </div>
              <p className="text-emerald-900">{kitchen.mealsServed.toLocaleString()}</p>
              <p className="text-sm text-emerald-600">Meals per Day</p>
            </div>
            <div className="text-center">
              <div className="flex items-center justify-center gap-2 mb-2">
                <CheckCircle2 className="size-6 text-emerald-600" />
              </div>
              <p className="text-emerald-900">{kitchen.certifications.length}</p>
              <p className="text-sm text-emerald-600">Certifications</p>
            </div>
            <div className="text-center">
              <div className="flex items-center justify-center gap-2 mb-2">
                <Award className="size-6 text-emerald-600" />
              </div>
              <p className="text-emerald-900">{kitchen.reviews?.length || 0}</p>
              <p className="text-sm text-emerald-600">Total Reviews</p>
            </div>
            <div className="text-center">
              <div className="flex items-center justify-center gap-2 mb-2">
                <div className="size-6 rounded-full bg-emerald-600" />
              </div>
              <p className="text-emerald-900">
                {kitchen.reviews && kitchen.reviews.length > 0 
                  ? ((Object.values(averageRatings).reduce((a, b) => a + b, 0) / 6).toFixed(1))
                  : 'N/A'
                }
              </p>
              <p className="text-sm text-emerald-600">Avg HACCP Score</p>
            </div>
          </div>

          <div className="flex flex-wrap gap-3">
            {kitchen.certifications.map((cert, idx) => (
              <div key={idx} className="flex items-center gap-2 bg-emerald-50 px-4 py-2 rounded-xl border border-emerald-200">
                <Award className="size-5 text-emerald-600" />
                <span className="text-emerald-900">{cert}</span>
              </div>
            ))}
          </div>
        </div>
      </div>

      {/* Photo Gallery Section */}
      {allPhotos.length > 0 && (
        <div className="bg-white rounded-3xl p-8 border border-emerald-100 mb-8">
          <div className="flex items-center justify-between mb-6">
            <div className="flex items-center gap-3">
              <div className="p-2 bg-emerald-100 rounded-xl">
                <ImageIcon className="size-6 text-emerald-600" />
              </div>
              <div>
                <h2 className="text-emerald-900">Galeri Foto Makanan</h2>
                <p className="text-sm text-emerald-600">{allPhotos.length} foto dari {kitchen.reviews?.filter(r => r.photos && r.photos.length > 0).length || 0} reviewer</p>
              </div>
            </div>
            <button
              onClick={() => setShowPhotoGallery(!showPhotoGallery)}
              className="text-emerald-600 hover:text-emerald-700 text-sm"
            >
              {showPhotoGallery ? 'Sembunyikan' : 'Lihat Semua'}
            </button>
          </div>
          
          <div className={`grid grid-cols-2 md:grid-cols-4 gap-4 ${showPhotoGallery ? '' : 'max-h-64 overflow-hidden'}`}>
            {allPhotos.map((photo, index) => (
              <div key={index} className="relative group aspect-square rounded-xl overflow-hidden border-2 border-emerald-200 hover:border-emerald-400 transition-colors">
                <img
                  src={photo.url}
                  alt={`Foto dari ${photo.reviewerName}`}
                  className="w-full h-full object-cover group-hover:scale-110 transition-transform duration-300"
                />
                <div className="absolute inset-0 bg-gradient-to-t from-black/70 via-black/0 to-transparent opacity-0 group-hover:opacity-100 transition-opacity">
                  <div className="absolute bottom-0 left-0 right-0 p-3">
                    <p className="text-white text-sm">{photo.reviewerName}</p>
                    <p className="text-emerald-200 text-xs">
                      {new Date(photo.date).toLocaleDateString('id-ID', {
                        day: 'numeric',
                        month: 'short',
                        year: 'numeric'
                      })}
                    </p>
                  </div>
                </div>
              </div>
            ))}
          </div>
        </div>
      )}

      {/* HACCP Ratings Chart */}
      {kitchen.reviews && kitchen.reviews.length > 0 && (
        <div className="bg-white rounded-3xl p-8 border border-emerald-100 mb-8">
          <h2 className="text-emerald-900 mb-6">HACCP Performance Overview</h2>
          <HACCPChart ratings={averageRatings} />
        </div>
      )}

      {/* Trend Analysis */}
      {kitchen.complianceTrend && kitchen.complianceTrend.length > 0 && (
        <div className="mb-8">
          <TrendAnalysis trendData={kitchen.complianceTrend} />
        </div>
      )}

      {/* Performance Badges */}
      {kitchen.performanceBadges && kitchen.performanceBadges.length > 0 && (
        <div className="mb-8">
          <PerformanceBadges badges={kitchen.performanceBadges} />
        </div>
      )}

      {/* Reviews Section */}
      <div className="bg-white rounded-3xl p-8 border border-emerald-100">
        <div className="flex items-center justify-between mb-6">
          <h2 className="text-emerald-900">Reviews</h2>
          <button
            onClick={onAddReview}
            className="flex items-center gap-2 bg-emerald-600 text-white px-6 py-3 rounded-xl hover:bg-emerald-700 transition-colors"
          >
            <Plus className="size-5" />
            <span>Add Review</span>
          </button>
        </div>

        {!kitchen.reviews || kitchen.reviews.length === 0 ? (
          <div className="text-center py-12">
            <p className="text-emerald-600 mb-4">Belum ada review untuk dapur ini</p>
            <p className="text-sm text-emerald-500">Jadilah yang pertama memberikan review!</p>
          </div>
        ) : (
          <div className="space-y-6">
            {kitchen.reviews.map((review) => (
              <ReviewCard key={review.id} review={review} />
            ))}
          </div>
        )}
      </div>
    </div>
  );
}