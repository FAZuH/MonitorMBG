import { useState } from 'react';
import { MapPin, Search, X, Star, Shield, AlertCircle } from 'lucide-react';

interface Kitchen {
  id: string;
  name: string;
  province: string;
  city: string;
  address: string;
  capacity: number;
  schoolsServed: number;
  rating: number;
  reviewCount: number;
  safetyStatus: 'excellent' | 'good' | 'needsImprovement';
  certifications: string[];
}

interface LocationFilterProps {
  onSelectKitchen?: (kitchen: Kitchen) => void;
}

// Simplified Indonesia location data (sampel - dapat diperluas)
const indonesiaLocations = [
  {
    province: 'DKI Jakarta',
    cities: ['Jakarta Pusat', 'Jakarta Utara', 'Jakarta Selatan', 'Jakarta Barat', 'Jakarta Timur', 'Kepulauan Seribu']
  },
  {
    province: 'Jawa Barat',
    cities: ['Bandung', 'Bekasi', 'Bogor', 'Cirebon', 'Depok', 'Sukabumi', 'Tasikmalaya', 'Cimahi']
  },
  {
    province: 'Jawa Tengah',
    cities: ['Semarang', 'Solo (Surakarta)', 'Magelang', 'Salatiga', 'Pekalongan', 'Tegal']
  },
  {
    province: 'Jawa Timur',
    cities: ['Surabaya', 'Malang', 'Kediri', 'Blitar', 'Mojokerto', 'Madiun', 'Pasuruan', 'Probolinggo']
  },
  {
    province: 'Banten',
    cities: ['Tangerang', 'Tangerang Selatan', 'Serang', 'Cilegon']
  },
  {
    province: 'Bali',
    cities: ['Denpasar', 'Badung', 'Gianyar', 'Tabanan']
  },
  {
    province: 'Sumatera Utara',
    cities: ['Medan', 'Binjai', 'Pematangsiantar', 'Tebing Tinggi']
  },
  {
    province: 'Sumatera Barat',
    cities: ['Padang', 'Bukittinggi', 'Payakumbuh', 'Solok']
  },
  {
    province: 'Sulawesi Selatan',
    cities: ['Makassar', 'Pare-Pare', 'Palopo']
  },
  {
    province: 'Kalimantan Timur',
    cities: ['Samarinda', 'Balikpapan', 'Bontang']
  }
];

// Mock kitchen data
const mockKitchens: Kitchen[] = [
  {
    id: '1',
    name: 'Dapur Gizi Jakarta Pusat',
    province: 'DKI Jakarta',
    city: 'Jakarta Pusat',
    address: 'Jl. Medan Merdeka No. 15',
    capacity: 5000,
    schoolsServed: 25,
    rating: 4.8,
    reviewCount: 142,
    safetyStatus: 'excellent',
    certifications: ['HACCP', 'Halal', 'ISO 22000']
  },
  {
    id: '2',
    name: 'Dapur Bergizi Surabaya',
    province: 'Jawa Timur',
    city: 'Surabaya',
    address: 'Jl. Pemuda No. 88',
    capacity: 4500,
    schoolsServed: 22,
    rating: 4.7,
    reviewCount: 98,
    safetyStatus: 'excellent',
    certifications: ['HACCP', 'Halal']
  },
  {
    id: '3',
    name: 'Dapur Sehat Bandung',
    province: 'Jawa Barat',
    city: 'Bandung',
    address: 'Jl. Asia Afrika No. 45',
    capacity: 3500,
    schoolsServed: 18,
    rating: 4.5,
    reviewCount: 76,
    safetyStatus: 'good',
    certifications: ['HACCP', 'Halal']
  },
  {
    id: '4',
    name: 'Dapur Gizi Jakarta Selatan',
    province: 'DKI Jakarta',
    city: 'Jakarta Selatan',
    address: 'Jl. Gatot Subroto No. 22',
    capacity: 4000,
    schoolsServed: 20,
    rating: 4.6,
    reviewCount: 87,
    safetyStatus: 'excellent',
    certifications: ['HACCP', 'Halal', 'ISO 22000']
  },
  {
    id: '5',
    name: 'Dapur Bergizi Medan',
    province: 'Sumatera Utara',
    city: 'Medan',
    address: 'Jl. Sisingamangaraja No. 10',
    capacity: 3000,
    schoolsServed: 15,
    rating: 4.4,
    reviewCount: 62,
    safetyStatus: 'good',
    certifications: ['HACCP', 'Halal']
  }
];

export function LocationFilter({ onSelectKitchen }: LocationFilterProps) {
  const [selectedProvince, setSelectedProvince] = useState<string>('');
  const [selectedCity, setSelectedCity] = useState<string>('');
  const [searchQuery, setSearchQuery] = useState('');
  const [isProvinceOpen, setIsProvinceOpen] = useState(false);
  const [isCityOpen, setIsCityOpen] = useState(false);

  const filteredProvinces = indonesiaLocations.filter(loc =>
    loc.province.toLowerCase().includes(searchQuery.toLowerCase())
  );

  const cities = indonesiaLocations.find(loc => loc.province === selectedProvince)?.cities || [];

  const filteredCities = cities.filter(city =>
    city.toLowerCase().includes(searchQuery.toLowerCase())
  );

  const filteredKitchens = mockKitchens.filter(kitchen => {
    const provinceMatch = !selectedProvince || kitchen.province === selectedProvince;
    const cityMatch = !selectedCity || kitchen.city === selectedCity;
    const searchMatch = !searchQuery || 
      kitchen.name.toLowerCase().includes(searchQuery.toLowerCase()) ||
      kitchen.city.toLowerCase().includes(searchQuery.toLowerCase()) ||
      kitchen.province.toLowerCase().includes(searchQuery.toLowerCase());
    
    return provinceMatch && cityMatch && searchMatch;
  });

  const getSafetyColor = (status: string) => {
    switch (status) {
      case 'excellent': return 'bg-green-100 text-green-800 border-green-300';
      case 'good': return 'bg-blue-100 text-blue-800 border-blue-300';
      case 'needsImprovement': return 'bg-orange-100 text-orange-800 border-orange-300';
      default: return 'bg-gray-100 text-gray-800 border-gray-300';
    }
  };

  const getSafetyLabel = (status: string) => {
    switch (status) {
      case 'excellent': return 'Sangat Baik';
      case 'good': return 'Baik';
      case 'needsImprovement': return 'Perlu Perbaikan';
      default: return status;
    }
  };

  const handleReset = () => {
    setSelectedProvince('');
    setSelectedCity('');
    setSearchQuery('');
  };

  return (
    <div className="bg-white rounded-2xl shadow-lg border border-gray-200 p-6">
      <div className="flex items-center gap-3 mb-6">
        <div className="p-3 bg-green-100 rounded-xl">
          <MapPin className="size-6 text-green-600" />
        </div>
        <div>
          <h2 className="text-green-900">Filter Lokasi Dapur</h2>
          <p className="text-sm text-gray-600">Cari dapur MBG berdasarkan wilayah</p>
        </div>
      </div>

      {/* Search Bar */}
      <div className="mb-4">
        <div className="relative">
          <Search className="absolute left-3 top-1/2 -translate-y-1/2 size-5 text-gray-400" />
          <input
            type="text"
            placeholder="Cari provinsi, kota, atau nama dapur..."
            value={searchQuery}
            onChange={(e) => setSearchQuery(e.target.value)}
            className="w-full pl-11 pr-10 py-3 border border-gray-300 rounded-xl focus:outline-none focus:ring-2 focus:ring-green-500"
          />
          {searchQuery && (
            <button
              onClick={() => setSearchQuery('')}
              className="absolute right-3 top-1/2 -translate-y-1/2 p-1 hover:bg-gray-100 rounded-lg"
            >
              <X className="size-4 text-gray-400" />
            </button>
          )}
        </div>
      </div>

      {/* Province and City Dropdowns */}
      <div className="grid md:grid-cols-2 gap-4 mb-6">
        {/* Province Dropdown */}
        <div className="relative">
          <label className="block text-sm text-gray-700 mb-2">Provinsi</label>
          <button
            onClick={() => setIsProvinceOpen(!isProvinceOpen)}
            className="w-full px-4 py-3 border border-gray-300 rounded-xl text-left hover:border-green-500 focus:outline-none focus:ring-2 focus:ring-green-500"
          >
            {selectedProvince || 'Pilih Provinsi'}
          </button>
          {isProvinceOpen && (
            <div className="absolute z-10 w-full mt-2 bg-white border border-gray-200 rounded-xl shadow-lg max-h-60 overflow-y-auto">
              {filteredProvinces.map((loc) => (
                <button
                  key={loc.province}
                  onClick={() => {
                    setSelectedProvince(loc.province);
                    setSelectedCity('');
                    setIsProvinceOpen(false);
                  }}
                  className="w-full px-4 py-2 text-left hover:bg-green-50 transition-colors"
                >
                  {loc.province}
                </button>
              ))}
              {filteredProvinces.length === 0 && (
                <div className="px-4 py-6 text-center text-gray-500">
                  Provinsi tidak ditemukan
                </div>
              )}
            </div>
          )}
        </div>

        {/* City Dropdown */}
        <div className="relative">
          <label className="block text-sm text-gray-700 mb-2">Kabupaten/Kota</label>
          <button
            onClick={() => selectedProvince && setIsCityOpen(!isCityOpen)}
            disabled={!selectedProvince}
            className={`w-full px-4 py-3 border border-gray-300 rounded-xl text-left ${
              !selectedProvince ? 'bg-gray-100 cursor-not-allowed' : 'hover:border-green-500 focus:outline-none focus:ring-2 focus:ring-green-500'
            }`}
          >
            {selectedCity || 'Pilih Kabupaten/Kota'}
          </button>
          {isCityOpen && selectedProvince && (
            <div className="absolute z-10 w-full mt-2 bg-white border border-gray-200 rounded-xl shadow-lg max-h-60 overflow-y-auto">
              {filteredCities.map((city) => (
                <button
                  key={city}
                  onClick={() => {
                    setSelectedCity(city);
                    setIsCityOpen(false);
                  }}
                  className="w-full px-4 py-2 text-left hover:bg-green-50 transition-colors"
                >
                  {city}
                </button>
              ))}
              {filteredCities.length === 0 && (
                <div className="px-4 py-6 text-center text-gray-500">
                  Kabupaten/Kota tidak ditemukan
                </div>
              )}
            </div>
          )}
        </div>
      </div>

      {/* Reset Button */}
      {(selectedProvince || selectedCity || searchQuery) && (
        <button
          onClick={handleReset}
          className="mb-6 px-4 py-2 bg-gray-100 text-gray-700 rounded-lg hover:bg-gray-200 transition-colors text-sm flex items-center gap-2"
        >
          <X className="size-4" />
          Reset Filter
        </button>
      )}

      {/* Results */}
      <div className="space-y-4">
        <div className="flex items-center justify-between">
          <h3 className="text-gray-900">Hasil Pencarian</h3>
          <span className="text-sm text-gray-600">{filteredKitchens.length} dapur ditemukan</span>
        </div>

        {filteredKitchens.length === 0 ? (
          <div className="bg-amber-50 border border-amber-200 rounded-xl p-8 text-center">
            <AlertCircle className="size-12 text-amber-500 mx-auto mb-3" />
            <p className="text-amber-900 mb-2">Belum tersedia dapur terdaftar untuk wilayah ini</p>
            <p className="text-sm text-amber-700 mb-4">
              Kami terus memperluas jangkauan layanan MBG di seluruh Indonesia
            </p>
            <button className="px-4 py-2 bg-amber-600 text-white rounded-lg hover:bg-amber-700 transition-colors text-sm">
              Ajukan Koreksi Data
            </button>
          </div>
        ) : (
          <div className="grid gap-4">
            {filteredKitchens.map((kitchen) => (
              <div
                key={kitchen.id}
                className="border border-gray-200 rounded-xl p-5 hover:shadow-lg transition-shadow bg-gradient-to-br from-white to-green-50/30"
              >
                <div className="flex items-start justify-between mb-3">
                  <div className="flex-1">
                    <h4 className="text-green-900 mb-1">{kitchen.name}</h4>
                    <div className="flex items-center gap-1 text-sm text-gray-600">
                      <MapPin className="size-4" />
                      {kitchen.city}, {kitchen.province}
                    </div>
                    <p className="text-sm text-gray-500 mt-1">{kitchen.address}</p>
                  </div>
                  <div className={`px-3 py-1 rounded-full border text-xs ${getSafetyColor(kitchen.safetyStatus)}`}>
                    {getSafetyLabel(kitchen.safetyStatus)}
                  </div>
                </div>

                <div className="grid grid-cols-2 md:grid-cols-4 gap-3 mb-4">
                  <div className="bg-white p-3 rounded-lg border border-gray-100">
                    <p className="text-xs text-gray-600 mb-1">Kapasitas</p>
                    <p className="text-green-900">{kitchen.capacity.toLocaleString()}/hari</p>
                  </div>
                  <div className="bg-white p-3 rounded-lg border border-gray-100">
                    <p className="text-xs text-gray-600 mb-1">Sekolah Dilayani</p>
                    <p className="text-green-900">{kitchen.schoolsServed} sekolah</p>
                  </div>
                  <div className="bg-white p-3 rounded-lg border border-gray-100">
                    <p className="text-xs text-gray-600 mb-1">Rating</p>
                    <div className="flex items-center gap-1">
                      <Star className="size-4 fill-amber-500 text-amber-500" />
                      <span className="text-green-900">{kitchen.rating}</span>
                    </div>
                  </div>
                  <div className="bg-white p-3 rounded-lg border border-gray-100">
                    <p className="text-xs text-gray-600 mb-1">Review</p>
                    <p className="text-green-900">{kitchen.reviewCount} review</p>
                  </div>
                </div>

                <div className="flex items-center justify-between pt-3 border-t border-gray-100">
                  <div className="flex flex-wrap gap-2">
                    {kitchen.certifications.map((cert) => (
                      <span key={cert} className="px-2 py-1 bg-green-100 text-green-700 rounded text-xs flex items-center gap-1">
                        <Shield className="size-3" />
                        {cert}
                      </span>
                    ))}
                  </div>
                  <button
                    onClick={() => onSelectKitchen?.(kitchen)}
                    className="px-4 py-2 bg-green-600 text-white rounded-lg hover:bg-green-700 transition-colors text-sm"
                  >
                    Lihat Detail
                  </button>
                </div>
              </div>
            ))}
          </div>
        )}
      </div>
    </div>
  );
}