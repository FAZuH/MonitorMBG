import { Kitchen } from '../App';
import { MapPin, Utensils, Award, Star } from 'lucide-react';
import { ImageWithFallback } from './figma/ImageWithFallback';

interface KitchenListProps {
  kitchens: Kitchen[];
  onSelectKitchen: (kitchen: Kitchen) => void;
}

export function KitchenList({ kitchens, onSelectKitchen }: KitchenListProps) {
  const calculateAverageRating = (kitchen: Kitchen): number => {
    if (kitchen.reviews.length === 0) return 0;
    
    const totalRatings = kitchen.reviews.reduce((acc, review) => {
      const avgReviewRating = (
        review.ratings.taste +
        review.ratings.hygiene +
        review.ratings.freshness +
        review.ratings.temperature +
        review.ratings.packaging +
        review.ratings.handling
      ) / 6;
      return acc + avgReviewRating;
    }, 0);
    
    return totalRatings / kitchen.reviews.length;
  };

  return (
    <div>
      <h2 className="text-emerald-900 mb-6">Dapur Terdaftar</h2>
      <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-6">
        {kitchens.map((kitchen) => {
          const avgRating = calculateAverageRating(kitchen);
          
          return (
            <div
              key={kitchen.id}
              onClick={() => onSelectKitchen(kitchen)}
              className="bg-white rounded-2xl overflow-hidden border border-emerald-100 hover:border-emerald-300 hover:shadow-xl transition-all duration-300 cursor-pointer group"
            >
              <div className="relative h-48 overflow-hidden bg-emerald-100">
                <ImageWithFallback
                  src={`https://source.unsplash.com/800x600/?${encodeURIComponent(kitchen.image)}`}
                  alt={kitchen.name}
                  className="w-full h-full object-cover group-hover:scale-110 transition-transform duration-500"
                />
                <div className="absolute top-4 right-4 bg-emerald-600 text-white px-3 py-1 rounded-full text-sm">
                  {kitchen.type}
                </div>
              </div>
              
              <div className="p-6">
                <h3 className="text-emerald-900 mb-2">{kitchen.name}</h3>
                
                <div className="flex items-center gap-2 text-emerald-600 mb-4">
                  <MapPin className="size-4" />
                  <span className="text-sm">{kitchen.location}</span>
                </div>

                <div className="flex items-center gap-4 mb-4">
                  <div className="flex items-center gap-1">
                    <Utensils className="size-4 text-emerald-500" />
                    <span className="text-sm text-emerald-700">{kitchen.mealsServed.toLocaleString()} meals/day</span>
                  </div>
                  {avgRating > 0 && (
                    <div className="flex items-center gap-1">
                      <Star className="size-4 fill-amber-400 text-amber-400" />
                      <span className="text-sm text-emerald-900">{avgRating.toFixed(1)}</span>
                    </div>
                  )}
                </div>

                <div className="flex flex-wrap gap-2 mb-4">
                  {kitchen.certifications.map((cert, idx) => (
                    <div key={idx} className="flex items-center gap-1 bg-emerald-50 px-3 py-1 rounded-full">
                      <Award className="size-3 text-emerald-600" />
                      <span className="text-xs text-emerald-700">{cert}</span>
                    </div>
                  ))}
                </div>

                <div className="pt-4 border-t border-emerald-100">
                  <p className="text-sm text-emerald-600">
                    {kitchen.reviews.length} {kitchen.reviews.length === 1 ? 'review' : 'reviews'}
                  </p>
                </div>
              </div>
            </div>
          );
        })}
      </div>
    </div>
  );
}
